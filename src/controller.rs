use crate::sorted_hash::SortedHasher;
use i3ipc::{reply::Workspace, I3Connection};
use lazy_static::lazy_static;
use log::info;
use std::{
    collections::{btree_map::Entry, BTreeMap},
    sync::Mutex,
};

lazy_static! {
    static ref sorted_hasher: Mutex<SortedHasher<String>> =
        Mutex::new(SortedHasher::new(2_u64.pow(30) as usize));
}

#[derive(Debug, Clone)]
struct Group {
    name: String,
    number: usize,
}

impl Group {
    fn new(name: &str, number: usize) -> Group {
        Group {
            name: name.to_owned(),
            number,
        }
    }
}

#[derive(Debug, Clone)]
struct CustomWorkspace {
    group: Option<Group>,
    local_number: usize,
    name: String,
}

impl CustomWorkspace {
    fn new(group: Option<Group>, local_number: usize) -> CustomWorkspace {
        let name = match &group {
            Some(group) => format!(
                "{}:{}:{}",
                group.number + local_number,
                group.name,
                local_number
            ),
            None => format!("{}", local_number),
        };
        CustomWorkspace {
            group,
            local_number,
            name,
        }
    }

    fn from_name(name: &str) -> CustomWorkspace {
        let fields = name.split(":").collect::<Vec<&str>>();
        let global_number = fields[0]
            .parse::<usize>()
            .expect("failed to parse workspace number: first field is not a number");
        let (group, local_number) = match fields.len() {
            3 => {
                let local_number = fields[2]
                    .parse::<usize>()
                    .expect("failed to parse workspace number: third field is not a number");
                (
                    Some(Group::new(fields[1], global_number - local_number)),
                    local_number,
                )
            }
            _ => (None, global_number),
        };
        CustomWorkspace {
            group,
            local_number,
            name: name.to_owned(),
        }
    }
}

pub struct WorkspaceGroupsController {
    i3connection: Mutex<I3Connection>,
    dry_run: bool,
    workspaces: Option<Vec<Workspace>>,
    groups: Option<BTreeMap<String, (Group, Vec<CustomWorkspace>)>>,
}

impl WorkspaceGroupsController {
    pub fn new(i3connection: I3Connection, dry_run: bool) -> WorkspaceGroupsController {
        WorkspaceGroupsController {
            i3connection: Mutex::new(i3connection),
            dry_run,
            workspaces: None,
            groups: None,
        }
    }

    fn send_i3_command(&self, command: &str) {
        if !self.dry_run {
            info!("Running command: `i3-msg {}`", command);
            self.i3connection
                .lock()
                .unwrap()
                .run_command(command)
                .expect("failed to execute i3-msg command");
        } else {
            info!("Dry-running command: `i3-msg {}`", command);
        }
    }

    fn get_workspaces(&mut self) -> &[Workspace] {
        if self.workspaces.is_none() {
            self.workspaces = Some(
                self.i3connection
                    .lock()
                    .unwrap()
                    .get_workspaces()
                    .expect("failed to get i3 workspaces")
                    .workspaces,
            )
        }
        self.workspaces.as_ref().unwrap()
    }

    fn get_focused_workspace(&mut self) -> &Workspace {
        self.get_workspaces()
            .iter()
            .find(|&workspace| workspace.focused)
            .expect("no focused workspace")
    }

    fn get_focused_group(&mut self) -> Option<Group> {
        CustomWorkspace::from_name(&self.get_focused_workspace().name).group
    }

    fn get_groups(&mut self) -> &mut BTreeMap<String, (Group, Vec<CustomWorkspace>)> {
        if self.groups.is_none() {
            self.groups = Some(
                self.get_workspaces()
                    .iter()
                    .map(|workspace| CustomWorkspace::from_name(&workspace.name))
                    .filter(|workspace| !workspace.group.is_none())
                    .fold(BTreeMap::new(), |mut map, workspace| {
                        let group = workspace.group.clone().unwrap();
                        let entry = map.entry(group.name.to_owned()).or_insert({
                            sorted_hasher
                                .lock()
                                .unwrap()
                                .set(group.number, group.name.to_owned());
                            (group, Vec::new())
                        });
                        entry.1.push(workspace);
                        map
                    }),
            )
        }
        self.groups.as_mut().unwrap()
    }

    pub fn get_group_names(&mut self) -> Vec<&str> {
        keys_to_strptr_vec!(self.get_groups())
    }

    pub fn focus_workspace(&mut self, local_number: usize) {
        let focused_group = self.get_focused_group();
        let new_workspace_name = CustomWorkspace::new(focused_group.clone(), local_number).name;
        let query = match focused_group.is_some() {
            true => format!("workspace {}", new_workspace_name),
            false => format!("workspace number {}", new_workspace_name),
        };
        self.send_i3_command(&query);
    }

    pub fn focus_group(&mut self, group_name: &str) {
        let groups = self.get_groups();
        let entry = groups.entry(group_name.to_owned()).or_insert({
            let group = Group::new(
                group_name,
                sorted_hasher.lock().unwrap().hash(group_name.to_owned()),
            );
            (group.clone(), vec![CustomWorkspace::new(Some(group), 1)])
        });
        let workspace_name = entry.1[0].name.clone();
        println!("{}", workspace_name);
        self.send_i3_command(&format!("workspace {}", workspace_name));
    }

    pub fn move_container_to_workspace(&mut self, local_number: usize) {
        let focused_group = self.get_focused_group();
        self.send_i3_command(&format!(
            "move to workspace {}",
            CustomWorkspace::new(focused_group, local_number).name
        ));
    }

    pub fn move_container_to_group(&mut self, group_name: &str) {
        if let Some(focused_group) = self.get_focused_group() {
            if focused_group.name == group_name {
                return;
            }
        }
        let new_workspace_name = {
            let groups = self.get_groups();
            match groups.entry(group_name.to_string()) {
                Entry::Occupied(entry) => {
                    let group = entry.get().0.clone();
                    let local_number = entry
                        .get()
                        .1
                        .iter()
                        .enumerate()
                        .filter(|(i, workspace)| workspace.local_number == i + 1)
                        .collect::<Vec<(usize, &CustomWorkspace)>>()
                        .len()
                        + 1;
                    CustomWorkspace::new(Some(group), local_number).name
                }
                Entry::Vacant(_entry) => {
                    CustomWorkspace::new(
                        Some(Group::new(
                            group_name,
                            sorted_hasher.lock().unwrap().hash(group_name.to_owned()),
                        )),
                        1,
                    )
                    .name
                }
            }
        };
        self.send_i3_command(&format!("move to workspace {}", new_workspace_name,));
    }

    pub fn rename_group(&mut self, new_group_name: &str) {
        let focused_group_name = match self.get_focused_group() {
            Some(x) => x.name,
            None => return,
        };
        let groups = self.get_groups();
        if groups.contains_key(new_group_name) {
            return;
        }
        let new_hash = sorted_hasher
            .lock()
            .unwrap()
            .hash(focused_group_name.to_string());
        let new_group = Group::new(new_group_name, new_hash);
        groups
            .get(&focused_group_name)
            .unwrap()
            .1
            .iter()
            .map(|workspace| {
                (
                    workspace.name.to_owned(),
                    CustomWorkspace::new(Some(new_group.clone()), workspace.local_number).name,
                )
            })
            .collect::<Vec<(String, String)>>()
            .iter()
            .for_each(|(old_name, new_name)| {
                self.send_i3_command(&format!("rename workspace {} to {}", old_name, new_name,))
            });
    }
}

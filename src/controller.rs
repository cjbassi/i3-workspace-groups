use i3ipc::{reply::Workspace, I3Connection};
use log::info;
use std::collections::BTreeMap;

const GROUP_SIZE: usize = 100;

#[derive(Debug, Clone)]
struct Group {
    group_number: usize,
    name: String,
}

impl Group {
    fn new(name: &str, group_number: usize) -> Group {
        Group {
            name: name.to_owned(),
            group_number,
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
                (group.group_number * GROUP_SIZE) + local_number,
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
            .expect("failed to parse workspace name: first field is not a number");
        let local_number = global_number % GROUP_SIZE;
        let group = match fields.len() {
            3 => Some(Group::new(fields[1], global_number / GROUP_SIZE)),
            _ => None,
        };
        CustomWorkspace {
            group,
            local_number,
            name: name.to_owned(),
        }
    }
}

pub struct WorkspaceGroupsController {
    i3connection: I3Connection,
    dry_run: bool,
    workspaces: Option<Vec<Workspace>>,
    groups: Option<BTreeMap<String, (Group, Vec<CustomWorkspace>)>>,
}

impl WorkspaceGroupsController {
    pub fn new(i3connection: I3Connection, dry_run: bool) -> WorkspaceGroupsController {
        WorkspaceGroupsController {
            i3connection,
            dry_run,
            workspaces: None,
            groups: None,
        }
    }

    fn send_i3_command(&mut self, command: &str) {
        if !self.dry_run {
            info!("Running command: `i3-msg {}`", command);
            self.i3connection
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

    fn get_groups(&mut self) -> &BTreeMap<String, (Group, Vec<CustomWorkspace>)> {
        if self.groups.is_none() {
            self.groups = Some(
                self.get_workspaces()
                    .iter()
                    .map(|workspace| CustomWorkspace::from_name(&workspace.name))
                    .filter(|workspace| !workspace.group.is_none())
                    .fold(BTreeMap::new(), |mut map, workspace| {
                        let group = workspace.group.clone().unwrap();
                        let entry = map
                            .entry(group.name.to_owned())
                            .or_insert((group, Vec::new()));
                        entry.1.push(workspace);
                        map
                    }),
            )
        }
        self.groups.as_ref().unwrap()
    }

    pub fn get_group_names(&mut self) -> Vec<&str> {
        keys_to_strptr_vec!(self.get_groups())
    }

    pub fn focus_workspace(&mut self, local_number: usize) {
        let focused_group = self.get_focused_group();
        self.send_i3_command(&format!(
            "workspace {}",
            CustomWorkspace::new(focused_group, local_number).name
        ));
    }

    // TODO switch to the last focused workspace in that group
    // TODO organize groups alphabetically
    pub fn focus_group(&mut self, group_name: &str) {
        let groups = self.get_groups();
        let group = match groups.get(group_name) {
            Some(x) => (*x).0.clone(),
            None => Group::new(group_name, groups.len() + 1),
        };
        self.send_i3_command(&format!(
            "workspace {}",
            CustomWorkspace::new(Some(group), 1).name
        ));
        // TODO renumber/reorder groups
    }

    pub fn move_container_to_workspace(&mut self, local_number: usize) {
        let focused_group = self.get_focused_group();
        self.send_i3_command(&format!(
            "move to workspace {}",
            CustomWorkspace::new(focused_group, local_number).name
        ));
    }

    pub fn move_workspace_to_group(&mut self, group_name: &str) {
        if let Some(focused_group) = self.get_focused_group() {
            if focused_group.name == group_name {
                return;
            }
        }
        let new_workspace_name = {
            let groups = self.get_groups();
            match groups.get(group_name) {
                Some(x) => {
                    let group = (*x).0.clone();
                    let local_number =
                        x.1.iter()
                            .enumerate()
                            .filter(|(i, workspace)| workspace.local_number == i + 1)
                            .collect::<Vec<(usize, &CustomWorkspace)>>()
                            .len()
                            + 1;
                    println!("{}", local_number);
                    CustomWorkspace::new(Some(group), local_number).name
                }
                None => {
                    CustomWorkspace::new(Some(Group::new(group_name, groups.len() + 1)), 1).name
                }
            }
        };
        let focused_workspace_name = self.get_focused_workspace().name.clone();
        self.send_i3_command(&format!(
            "rename workspace {} to {}",
            focused_workspace_name, new_workspace_name,
        ));
        // TODO reorder/renumber groups
    }

    pub fn rename_group(&mut self, group_name: &str, new_group_name: &str) {
        if let Some((group, workspaces)) = self.get_groups().get(group_name) {
            workspaces
                .iter()
                .map(|workspace| {
                    (
                        workspace.name.clone(),
                        CustomWorkspace::new(
                            Some(Group::new(new_group_name, group.group_number)),
                            workspace.local_number,
                        )
                        .name,
                    )
                })
                .collect::<Vec<(String, String)>>()
                .iter()
                .for_each(|(old, new)| {
                    self.send_i3_command(&format!("rename workspace {} to {}", old, new))
                });
        }
        // TODO reorder/renumber groups
    }
}

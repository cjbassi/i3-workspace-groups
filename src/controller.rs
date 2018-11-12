use crate::common::query_rofi;
use i3ipc::{reply::Workspace, I3Connection};
use log::info;

fn get_group_name_and_local_number(workspace_name: &str) -> (String, usize) {
    match workspace_name.find(":") {
        Some(x) => (
            workspace_name[..x].to_owned(),
            workspace_name[x + 1..]
                .parse::<usize>()
                .expect("workspace name is not a number"),
        ),
        None => (
            "Default".to_owned(),
            workspace_name
                .parse::<usize>()
                .expect("workspace name is not a number"),
        ),
    }
}

fn get_workspace_name(group_name: &str, local_number: usize) -> String {
    if group_name == "Default" {
        return local_number.to_string();
    }
    format!("{}:{}", group_name, local_number)
}

fn rofi_get_group_name(group_name: Option<String>, group_names: Vec<String>) -> Option<String> {
    match group_name {
        Some(x) => Some(x),
        None => query_rofi("Group name", Some(group_names)),
    }
}

fn rofi_get_local_number(local_number: Option<usize>) -> Option<usize> {
    match local_number {
        Some(x) => Some(x),
        None => match query_rofi("Workspace number", None) {
            Some(x) => Some(x.parse::<usize>().expect("please give a workspace number")),
            None => None,
        },
    }
}

pub struct WorkspaceGroupsController {
    i3connection: I3Connection,
    dry_run: bool,
    workspaces: Option<Vec<Workspace>>,
}

impl WorkspaceGroupsController {
    pub fn new(i3connection: I3Connection, dry_run: bool) -> WorkspaceGroupsController {
        WorkspaceGroupsController {
            i3connection,
            dry_run,
            workspaces: None,
        }
    }

    fn send_i3_command(&mut self, command: &str) {
        if !self.dry_run {
            info!("Running command: `i3-msg {}`", command);
            self.i3connection
                .run_command(command)
                .expect("could not execute i3-msg command");
        } else {
            info!("Dry-running command: `i3-msg {}`", command);
        }
    }

    fn get_workspaces(&mut self) -> &[Workspace] {
        if self.workspaces.is_none() {
            self.workspaces = Some(
                self.i3connection
                    .get_workspaces()
                    .expect("could not get i3 workspaces")
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

    fn get_focused_group_name(&mut self) -> String {
        get_group_name_and_local_number(&self.get_focused_workspace().name).0
    }

    fn get_group_names(&mut self) -> Vec<String> {
        let mut group_names = self
            .get_workspaces()
            .into_iter()
            .map(|workspace| get_group_name_and_local_number(&workspace.name).0)
            .collect::<Vec<String>>();
        group_names.sort();
        group_names.dedup();
        group_names
    }

    pub fn focus_workspace(&mut self, local_number: Option<usize>) {
        let focused_group_name = self.get_focused_group_name();
        let local_number = unwrap_option_or_return!(rofi_get_local_number(local_number));
        self.send_i3_command(&format!(
            "workspace {}",
            get_workspace_name(&focused_group_name, local_number,)
        ));
    }

    // TODO switch to the last focused workspace in that group
    pub fn focus_group(&mut self, group_name: Option<String>) {
        let group_name =
            unwrap_option_or_return!(rofi_get_group_name(group_name, self.get_group_names()));
        self.send_i3_command(&format!("workspace {}", get_workspace_name(&group_name, 1)));
    }

    pub fn move_container_to_workspace(&mut self, local_number: Option<usize>) {
        let focused_group_name = self.get_focused_group_name();
        let local_number = unwrap_option_or_return!(rofi_get_local_number(local_number));
        self.send_i3_command(&format!(
            "move to workspace {}",
            get_workspace_name(&focused_group_name, local_number)
        ));
    }

    pub fn move_workspace_to_group(&mut self, group_name: Option<String>) {
        let group_name =
            unwrap_option_or_return!(rofi_get_group_name(group_name, self.get_group_names()));
        let focused_workspace_name = self.get_focused_workspace().name.to_owned();
        if get_group_name_and_local_number(&focused_workspace_name).0 == group_name {
            return;
        }
        let local_numbers_in_group = self
            .get_workspaces()
            .iter()
            .map(|workspace| get_group_name_and_local_number(&workspace.name))
            .filter(|(g, _)| *g == group_name)
            .map(|(_, l)| l)
            .collect::<Vec<usize>>();
        let new_number = if local_numbers_in_group.is_empty() {
            1
        } else {
            let offset_nums = local_numbers_in_group
                .iter()
                .enumerate()
                .filter(|(i, l)| (i + 1) != **l)
                .collect::<Vec<(usize, &usize)>>();
            if offset_nums.is_empty() {
                local_numbers_in_group.len() + 1
            } else {
                offset_nums[0].0 + 1
            }
        };
        let new_workspace_name = get_workspace_name(&group_name, new_number);
        self.send_i3_command(&format!(
            "rename workspace {} to {}",
            focused_workspace_name, new_workspace_name
        ));
    }

    pub fn rename_group(&mut self, group_name: Option<String>, new_group_name: Option<String>) {
        let group_name =
            unwrap_option_or_return!(rofi_get_group_name(group_name, self.get_group_names()));
        let new_group_name = match new_group_name {
            Some(x) => x,
            None => unwrap_option_or_return!(query_rofi("New group name", None)),
        };
        self.get_workspaces()
            .iter()
            .filter(|workspace| get_group_name_and_local_number(&workspace.name).0 == group_name)
            .map(|workspace| workspace.name.clone())
            .collect::<Vec<String>>()
            .iter()
            .for_each(|workspace_name| {
                let local_number = get_group_name_and_local_number(&workspace_name).1;
                let new_workspace_name = get_workspace_name(&new_group_name, local_number);
                self.send_i3_command(&format!(
                    "rename workspace {} to {}",
                    workspace_name, new_workspace_name
                ))
            });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_group_name_and_local_number() {
        assert_eq!(
            get_group_name_and_local_number("hello:1"),
            ("hello".to_owned(), 1)
        );
        assert_eq!(
            get_group_name_and_local_number("1"),
            ("Default".to_owned(), 1)
        );
    }

    #[test]
    fn test_get_workspace_name() {
        assert_eq!(get_workspace_name("hello", 1), "hello:1".to_owned());
        assert_eq!(get_workspace_name("Default", 1), "1".to_owned());
    }
}

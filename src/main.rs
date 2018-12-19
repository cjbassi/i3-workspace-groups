mod args;
#[macro_use]
mod common;
mod controller;
mod sorted_hash;

use self::args::{Args, Subcommands};
use crate::common::query_rofi;
use i3ipc::I3Connection;
use structopt::StructOpt;

fn rofi_get_group_name(group_name: Option<String>, group_names: &[&str]) -> Option<String> {
    group_name.or_else(|| query_rofi("Select group", Some(group_names)))
}

fn rofi_get_local_number(local_number: Option<usize>) -> Option<usize> {
    local_number.or_else(|| {
        query_rofi("Select workspace number", None).map(|num| {
            num.parse::<usize>().expect(&format!(
                "failed to parse '{}': input needs to be an integer",
                num
            ))
        })
    })
}

fn rofi_get_new_group_name(new_group_name: Option<String>) -> Option<String> {
    new_group_name.or_else(|| query_rofi("Rename to", None))
}

fn main() {
    env_logger::init();
    let args = Args::from_args();
    let connection = I3Connection::connect().expect("failed to connect to i3-msg");
    let mut controller = controller::WorkspaceGroupsController::new(connection, args.dry_run);
    let group_names = controller.get_group_names();
    match args.subcommands {
        Subcommands::FocusWorkspace { local_number } => {
            if let Some(local_number) = rofi_get_local_number(local_number) {
                controller.focus_workspace(local_number);
            }
        }
        Subcommands::FocusGroup { group_name } => {
            if let Some(group_name) = rofi_get_group_name(group_name, &group_names) {
                controller.focus_group(&group_name);
            }
        }
        Subcommands::MoveContainerToWorkspace { local_number } => {
            if let Some(local_number) = rofi_get_local_number(local_number) {
                controller.move_container_to_workspace(local_number);
            }
        }
        Subcommands::MoveContainerToGroup { group_name } => {
            if let Some(group_name) = rofi_get_group_name(group_name, &group_names) {
                controller.move_container_to_group(&group_name);
            }
        }
        Subcommands::RenameGroup { new_group_name } => {
            if let Some(new_group_name) = rofi_get_new_group_name(new_group_name) {
                controller.rename_group(&new_group_name);
            }
        }
    }
}

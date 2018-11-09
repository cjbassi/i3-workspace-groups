mod args;
#[macro_use]
mod common;
mod controller;

use self::args::{Args, Subcommands};
use i3ipc::I3Connection;
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<Error>> {
    env_logger::init();
    let args = Args::from_args();
    let connection = I3Connection::connect().expect("could not connect to i3-msg");
    let mut controller = controller::WorkspaceGroupsController::new(connection, args.dry_run);

    match args.subcommands {
        Subcommands::FocusWorkspace { local_number } => {
            controller.focus_workspace(local_number);
        }
        Subcommands::FocusGroup { group_name } => {
            controller.focus_group(group_name);
        }
        Subcommands::MoveContainerToWorkspace { local_number } => {
            controller.move_container_to_workspace(local_number);
        }
        Subcommands::MoveWorkspaceToGroup { group_name } => {
            controller.move_workspace_to_group(group_name);
        }
        Subcommands::RenameGroup {
            group_name,
            new_group_name,
        } => {
            controller.rename_group(group_name, new_group_name);
        }
    }

    Ok(())
}

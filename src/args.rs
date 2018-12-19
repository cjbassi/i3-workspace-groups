use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommands: Subcommands,
    /// Dry-run i3-msg commands
    #[structopt(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(StructOpt, Debug)]
pub enum Subcommands {
    /// Focus a different workspace in the focused group
    #[structopt(name = "focus-workspace")]
    FocusWorkspace {
        #[structopt(name = "local-number")]
        local_number: Option<usize>,
    },
    /// Focus a different group
    #[structopt(name = "focus-group")]
    FocusGroup {
        #[structopt(name = "group-name")]
        group_name: Option<String>,
    },
    /// Move selected container to a different workspace in the focused group
    #[structopt(name = "move-container-to-workspace")]
    MoveContainerToWorkspace {
        #[structopt(name = "local-number")]
        local_number: Option<usize>,
    },
    /// Move selected container to a different group
    #[structopt(name = "move-container-to-group")]
    MoveContainerToGroup {
        #[structopt(name = "group-name")]
        group_name: Option<String>,
    },
    /// Rename focused group
    #[structopt(name = "rename-group")]
    RenameGroup {
        #[structopt(name = "new-group-name")]
        new_group_name: Option<String>,
    },
}

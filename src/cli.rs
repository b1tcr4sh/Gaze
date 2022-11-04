use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WayfarerCli {
    #[clap(subcommand)]
    pub operation: Operation

}
#[derive(Debug, Subcommand)]
pub enum Operation {
    Add(AddCommand),
    Sync,
    Generate,
    Profile(ProfileCommand),
    List,
    Search {
        name: String
    },
    Info {
        name: String,
        #[arg(short)]
        service: String
    }
}

#[derive(Debug, Args)]
pub struct ProfileCommand {
    #[clap(subcommand)]
    pub subcommand: ProfileSubcommand
}

#[derive(Debug, Subcommand)] 
pub enum ProfileSubcommand {
    Create(CreateSubcommand),
    Delete(DeleteSubcommand),
    List
}

#[derive(Debug, Args)] 
pub struct AddCommand {
    pub id: String,
}

#[derive(Debug, Args)]
pub struct CreateSubcommand {
    name: String,
    minecraft_version: String,
    mod_loader: String,
    is_server_side: bool
}

#[derive(Debug, Args)]
pub struct DeleteSubcommand {
    name: String
}
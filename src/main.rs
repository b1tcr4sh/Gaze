mod search;
mod cli;
mod stargate;

use clap::Parser;
use cli::{WayfarerCli, Operation};


#[tokio::main]
async fn main() {
    let args: WayfarerCli = WayfarerCli::parse();

    match args.operation {
        Operation::Search { name } => match search::get(&name).await {
            Ok(res) => res.print(),
            Err(e) => eprintln!("{}", e)
        }
        Operation::Add(_) => todo!(),
        Operation::Sync => todo!(),
        Operation::Generate => todo!(),
        Operation::Profile(_) => todo!(),
        Operation::List => todo!(),
        Operation::Info {name: _, service: _ } => todo!(),
    }
}
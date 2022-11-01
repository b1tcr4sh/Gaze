mod search;
mod cli;

use clap::Parser;
use cli::{WayfarerCli, Operation};


#[tokio::main]
async fn main() {

    let args: WayfarerCli = WayfarerCli::parse();

    match &args.operation {
        Operation::Search(SearchCommand) => match search::get(&service, &name).await {
            Ok(res) => println!("Success"),
            Err(e) => eprintln!("{}", e)
        }
    }
}
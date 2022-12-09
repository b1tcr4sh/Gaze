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
        Operation::List => {
            let connection = stargate::Connect().await;
            let messenger = stargate::GetProfileMessenger(&connection).await;
            let profiles = stargate::GetProfiles(&connection, messenger).await;

            for profile in profiles {
                let info = profile.GetProfileInfo().await.unwrap();

                println!("{0} / {1}", info.name, info.minecraft_version);
            }
        },
        Operation::Info {name: _, service: _ } => todo!(),
    }
}
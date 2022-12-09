mod search;
mod cli;
mod stargate;

use clap::Parser;
use cli::{WayfarerCli, Operation};

use self::stargate::profile_model::ProfileInfo;
use self::stargate::LOADERS;


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
            let connection = stargate::connect().await;
            let messenger = stargate::get_profile_messenger(&connection).await;
            let profiles = stargate::get_profiles(&connection, messenger).await;

            for profile in profiles {
                let info = profile.GetProfileInfo().await.unwrap();

                print_profile(&info);
            }
        },
        Operation::Info {name: _, service: _ } => todo!(),
    }
}

fn print_profile(info: &ProfileInfo) {
    match info.is_server_side {
        true => {
            println!("Profile {0}: ({1}) -> {2} {3}", info.name, "server", LOADERS[info.loader as usize], info.minecraft_version);
        },
        false => {
            println!("Profile {0}: ({1}) -> {2} {3}", info.name, "client", LOADERS[info.loader as usize], info.minecraft_version);
        }
    }
}
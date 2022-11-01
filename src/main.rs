mod api_client;
mod cli;

use clap::Parser;
use cli::WayfarerCli;


#[tokio::main]
async fn main() {

    let args = WayfarerCli::parse();

}
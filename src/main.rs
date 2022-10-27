mod api_client;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() <= 1 {
        eprintln!("No args passed... ?");
        return;
    }

    let command: &String = &args[1];

    if command == &String::from("help") {
        println!("Help message ahhhh");
        return;
    }
    

    let mod_res =  api_client::get_mod(api_client::Service::Modrinth, &args[1]).await;

    println!("{}", mod_res.expect("Request failed"));
}
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

    if !mod_res.is_ok() {
        println!("Couldn't complete request.. ?");
        return;
    }

    print_search(&mod_res.unwrap().hits);
}

fn print_search(hits: &[api_client::Hit]) {
    for hit in hits {
        println!("==");
        print!("{}\n\n{}\n", &hit.title, &hit.description);
        print!("Uploaded by: {}\n", &hit.author);
    }
}
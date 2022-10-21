use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();


    if args.len() <= 1 {
        eprintln!("No args passed... ?");
        return;
    }


    let command: &String = &args[1];

    if command == &String::from("help") {
        println!("Help message ahhhh");
    }
}

extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std::env;
use std::fs;

fn main() {
    let file_path: &String;
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => file_path = &args[1],
        1 => {
            println!("Error >> No file provided");
            return;
        }
        _ => {
            println!("Error >> Wrong amount of arguments");
            return;
        }
    }

    let mut ctx = ClipboardContext::new().unwrap();

    let file_content = fs::read_to_string(file_path);

    let file_content = match file_content {
        Ok(file_content) => file_content,
        Err(_) => {
            println!("Error >> Unable to find {}", file_path);
            return;
        }
    };

    ctx.set_contents(file_content.to_owned()).unwrap();
    ctx.get_contents().unwrap();

    println!("Copied {}", file_path);
}

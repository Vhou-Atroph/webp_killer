use std::env;
use std::process;

use webp_killer::WebpFile;

fn main() {
    let args: Vec<String> = env::args().collect(); //collect arguments from command prompt
    if args.len() > 1 { //the first arg will ALWAYS be the program itself- you need args after 0.
        let img = WebpFile::new(&args).unwrap_or_else(|err| { //use constructor from lib.rs to create a WebpFile instance
            println!("Problem parsing arguments: {}", err);
            process::exit(1); //if something happens while going through the args, print the error and exit at non-zero state
        });
    img.kill(); //kill the webp file
    } else{webp_killer::watcher().unwrap_or_else(|err| {println!("An error occured while watching: {}",err);}); //if there are no other args besides 0, start watching the directory for newly created webp files.
    }
}
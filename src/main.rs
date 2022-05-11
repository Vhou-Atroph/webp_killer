use std::env;
use std::process;

use webp_killer::WebpFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let img = WebpFile::new(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    img.kill();
    } else{webp_killer::watcher().unwrap_or_else(|err| {println!("An error occured while watching: {}",err);});
    }
}
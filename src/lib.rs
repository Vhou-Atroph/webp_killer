extern crate notify;
use notify::Watcher;
use std::fs;

///Struct WebpFile consists of two parts: its filestem, and its output.

pub struct WebpFile {
    pub name: String,
    pub output: String,
}
impl WebpFile {

    ///Creates a new WebpFile struct. Accepts 1-2 arguments from the command line. If only the filestem is specified, the program defaults to setting the output as a png.

    pub fn new(args: &[String]) -> Result<Self, &'static str> { 
        if args.len() > 3 {
            return Err("incorrect number of arguments! requires 1-2 args."); 
        }
        let output = if args.len() < 3 {"png".to_string()} else {args[2].clone()};
        let name = args[1].clone(); 
        Ok(Self{name,output}) 
    }

    ///Saves a copy of the .webp file as your desired output and deletes the original file.

    pub fn kill(&self) {
        let img = image::open(format!("{}.webp",&self.name)).expect("Could not open the webp file!"); 
        img.save(format!("{}.{}",&self.name,&self.output)).expect("Could not save file to new file!"); 
        println!("Converted {}.webp to {}.{}!",&self.name,&self.name,&self.output); 
        fs::remove_file(format!("{}.webp",&self.name)).unwrap(); 
    }
}

///If no arguments are provided to the terminal, the program creates a watcher to automatically detect webp files downloaded to the directory. Uses fn webp_killer for event handling.

pub fn watcher() -> notify::Result<()> {
    let mut watcher = notify::recommended_watcher(webp_killer)?;
    watcher.watch(std::path::Path::new("."), notify::RecursiveMode::Recursive)?;
    std::thread::sleep(std::time::Duration::MAX);
    Ok(())
}

///Event handler for the .webp watcher. If a webp file is created, it is converted to a .png file at blazingly fast speeds.

fn webp_killer(res: notify::Result<notify::Event>) {
    match res {
        Ok(event) => {
            let p = &event.paths[0];
            if p.extension().unwrap_or_else(|| { 
                std::ffi::OsStr::new("NONE")
            }) == "webp" && event.kind == notify::EventKind::Create(notify::event::CreateKind::Any) {
                std::thread::sleep(std::time::Duration::from_micros(5));
                let evil = WebpFile { 
                    name: p.file_stem().unwrap().to_str().unwrap().to_string(),
                    output: "png".to_string(),
                };
                evil.kill();
            }
        },
        Err(e) => println!("Error: {:?}",e),
    }
}
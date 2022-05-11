extern crate notify;
use crate::notify::Watcher;
use std::fs;

pub struct WebpFile { //public struct WebpFile takes two inputs, both Strings.
    pub name: String,
    pub output: String,
}
impl WebpFile { //functions implemented by WebpFile - these can be used as methods e.g. img.kill() will convert the webp file to a png and delete it. plus it looks cooler than kill(img)
    pub fn new(args: &[String]) -> Result<WebpFile, &'static str> { //function to create a new WebpFile struct
        if args.len() > 3 {
            return Err("incorrect number of arguments! requires 1-2 args."); //if something went wrong, return this error.
        }
        let output; //creates empty variable- this puts it in scope for the following if else statements
        if args.len() < 3 {
            output = "png".to_string(); //if less than 2 inputs (since the first arg is always the program itself) default to output as png
        } else {output = args[2].clone();} //otherwise, the output is the second input.
        let name = args[1].clone(); //the first input is always required, and must always be cloned from the arg vector
        Ok(WebpFile{name,output}) //if everything is ok, return the WebpFile struct
    }
    pub fn kill(&self) { //function to Kill the file
        let img = image::open(format!("{}.webp",&self.name)).expect("Could not open the webp file!"); //opens the webp file, not case sensitive. if it cant find the file, send the expect statement (panic)
        img.save(format!("{}.{}",&self.name,&self.output)).expect("Could not save file to new file!"); //saves the webp file as a better format, but also removes any capitalization from the file name if user did not type it exactly as it was originally. if it cant save, send the expect statement (panic) this is most likely to happen due to an invalid file extension, but idk something else could probably happen here as well
        println!("Converted {}.webp to {}.{}!",&self.name,&self.name,&self.output); //lets user know the process happened
        fs::remove_file(format!("{}.webp",&self.name)).unwrap(); //removes the webp file from its terrible existence
    }
}
pub fn watcher() -> notify::Result<()> {
    let mut watcher = notify::recommended_watcher(webp_killer)?;
    watcher.watch(std::path::Path::new("."), notify::RecursiveMode::Recursive)?;
    loop{}
}
fn webp_killer(res: notify::Result<notify::Event>) {
    match res {
        Ok(event) => {
            let p = &event.paths[0];
            if p.extension().unwrap_or_else(|| {
                std::ffi::OsStr::new("NONE")
            }) == "webp" && event.kind == notify::EventKind::Create(notify::event::CreateKind::Any) {
                std::thread::sleep(std::time::Duration::from_secs(1));
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
pub struct WebpFile {
    pub name: String,
    pub output: String,
}
impl WebpFile {
    pub fn new(args: &[String]) -> Result<WebpFile, &'static str> {
        if args.len() > 3 {
            return Err("incorrect number of arguments! requires 1-2 args.");
        }
        let output;
        if args.len() < 3 {
            output = "png".to_string();
        } else {output = args[2].clone();}
        let name = args[1].clone();
        Ok(WebpFile{name,output})
    }
    pub fn kill(&self) {
        let img = image::open(format!("{}.webp",&self.name)).unwrap();
        img.save(format!("{}.{}",&self.name,&self.output)).unwrap();
        println!("Output {}.webp as {}.{}!",&self.name,&self.name,&self.output);
    }
}
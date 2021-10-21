use std::{ fmt, env };

pub struct Config {
    pub name: String,
    pub image_path: String,
    pub options: Options
}

pub struct Options {
    pub wakelock: bool,
    pub theme_color: String,
    pub bg_color: String,
    pub desc: String,
    pub orientation: String
}

impl Options {
    fn new(mut args: env::Args) -> Result<Options, &'static str> {
        // TODO let user know about unsupported options (check args.count())
        let wakelock = args.find(|arg| arg == "--wakelock").is_some();
        let theme_color = get_option_value(&mut args, "theme_color:", "#fff");
        let bg_color = get_option_value(&mut args, "bg_color:", "#000");
        let desc = get_option_value(&mut args, "desc:", "A generated pwa");
        let orientation = get_option_value(&mut args, "orientation:", "any");

        Ok(Options { wakelock, theme_color, bg_color, desc, orientation })
    }
}

fn get_option_value(args: &mut env::Args, key: &str, default: &str) -> String {
    match args.find(|arg| arg.contains(key)) {
        Some(option) => option.replace(key, ""),
        None => String::from(default)
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // first arg is the current file
        args.next();

        let name = match args.next() {
            // TODO validate name-format
            Some(arg) => arg,
            None => return Err("Didn't specify a name")
        };

        let image_path = match args.next() {
            // TODO load image here?
            Some(arg) => arg,
            None => String::from("./src/assets/default-icon.png")
        };

        let options = Options::new(args).unwrap();

        Ok(Config { name, image_path, options })
    }
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "wakelock: {}, theme_color: {}, bg_color: {}, desc: {}, orientation: {}",
            self.wakelock,
            self.theme_color,
            self.bg_color,
            self.desc,
            self.orientation
        )
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, image_path: {}, options: {}",
            self.name,
            self.image_path,
            self.options
        )
    }
}
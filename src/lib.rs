use clap::{ Arg, App };
use std::fmt;

pub struct Config {
    pub path: String,
    pub icon: String,
    pub wakelock: bool,
    pub theme_color: String,
    pub background_color: String,
    pub description: String,
    pub orientation: String
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = App::new("PWA shell generator")
            .version("0.1.0")
            .author("Andre Nuechter")
            .about("Generate the shell of a Progressive Web App")
            .arg(Arg::with_name("path")
                .required(true)
                .takes_value(true)
                // TODO validate name-format?
                // .validator(fn: (v: String) -> Result<(), String>)
                .help("The path of your new PWA"))
            .arg(Arg::with_name("icon")
                .short("i")
                .long("icon")
                .takes_value(true)
                .default_value("./src/assets/default-icon.png")
                .help("The icon you want to use"))
            .arg(Arg::with_name("theme_color")
                .short("t")
                .long("theme")
                .takes_value(true)
                .default_value("#fff")
                .help("The desired theme color"))
            .arg(Arg::with_name("background_color")
                .short("bg")
                .long("background")
                .takes_value(true)
                .default_value("#000")
                .help("The desired background color"))
            .arg(Arg::with_name("description")
                .short("d")
                .long("description")
                .takes_value(true)
                .default_value("A generated PWA")
                .help("A short description of the app"))
            .arg(Arg::with_name("orientation")
                .short("o")
                .long("orientation")
                .takes_value(true)
                .default_value("any")
                // TODO verify this against list of allowed values (s. readme)
                // .validator(fn: (v: String) -> Result<(), String>)
                .help("The screen orientation for the manifest"))
            .arg(Arg::with_name("wakelock")
                .short("w")
                .long("wakelock")
                .help("Flag to tell whether the PWA should keep the screen awake"))
            .get_matches();

        let path = matches.value_of("path").unwrap().to_string();
        let icon = matches.value_of("icon").unwrap().to_string();
        let theme_color = matches.value_of("theme_color").unwrap().to_string();
        let background_color = matches.value_of("background_color").unwrap().to_string();
        let description = matches.value_of("description").unwrap().to_string();
        let orientation = matches.value_of("orientation").unwrap().to_string();
        let wakelock = matches.is_present("wakelock");

        Ok(Config {
            path,
            icon,
            theme_color,
            background_color,
            description,
            orientation,
            wakelock
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, image_path: {}, theme_color: {}, background_color: {}, description: {}, orientation: {}, wakelock: {}",
            self.path,
            self.icon,
            self.theme_color,
            self.background_color,
            self.description,
            self.orientation,
            self.wakelock
        )
    }
}
use clap::{ Arg, App };
use std::fmt;
use regex::Regex;

const ORIENTATION_VALUES: [&str; 8] = [
    "any",
    "natural",
    "landscape",
    "landscape-primary",
    "landscape-secondary",
    "portrait",
    "portrait-primary",
    "portrait-secondary"
];

pub struct Config {
    pub name: String,
    pub icon: String,
    pub wakelock: bool,
    pub theme_color: String,
    pub background_color: String,
    pub description: String,
    pub orientation: String,
    pub nodejs_based: bool
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = App::new("PWA shell generator")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Andre Nuechter")
            .about("Generate the shell of a Progressive Web App")
            .arg(Arg::with_name("name")
                .required(true)
                .takes_value(true)
                .validator(|value| {
                    let name_regexp: Regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
                    if name_regexp.is_match(&value) {
                        Ok(())
                    } else {
                        Err(String::from("Invalid name format"))
                    }
                })
                .help("The name of your new PWA (^[a-z0-9]+(?:-[a-z0-9]+)*$"))
            .arg(Arg::with_name("icon")
                .short("i")
                .long("icon")
                .takes_value(true)
                .default_value("default")
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
                .validator(|value| {
                    if ORIENTATION_VALUES.contains(&value.as_str()) {
                        Ok(())
                    } else {
                        Err(String::from("Unknown orientation value"))
                    }
                })
                .help("The screen orientation for the manifest"))
            .arg(Arg::with_name("wakelock")
                .short("w")
                .long("wakelock")
                .help("Flag to tell whether the PWA should keep the screen awake"))
            .arg(Arg::with_name("nodejs_based")
                .short("N")
                .long("nodejs_based")
                .takes_value(true)
                .default_value("true")
                .help("Include a NodeJS based build-setup"))
            .get_matches();

        let name = matches.value_of("name").unwrap().to_string();
        let icon = matches.value_of("icon").unwrap().to_string();
        let theme_color = matches.value_of("theme_color").unwrap().to_string();
        let background_color = matches.value_of("background_color").unwrap().to_string();
        let description = matches.value_of("description").unwrap().to_string();
        let orientation = matches.value_of("orientation").unwrap().to_string();
        let wakelock = matches.is_present("wakelock");
        let nodejs_based = matches.value_of("nodejs_based").unwrap().to_string() == "true";

        Ok(Config {
            name,
            icon,
            theme_color,
            background_color,
            description,
            orientation,
            wakelock,
            nodejs_based
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, image_path: {}, theme_color: {}, background_color: {}, description: {}, orientation: {}, wakelock: {}",
            self.name,
            self.icon,
            self.theme_color,
            self.background_color,
            self.description,
            self.orientation,
            self.wakelock
        )
    }
}
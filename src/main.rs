use std::fs;
use image::{ io as image_io, imageops };

use pwa_shell_generator::Config;

fn main() -> std::io::Result<()> {
    let config = Config::new().unwrap();

    // create folders
    // TODO create /docs or rm from readme
    fs::create_dir(format!("../{}", config.name))?;
    fs::create_dir(format!("../{}/src", config.name))?;
    fs::create_dir(format!("../{}/src/images", config.name))?;
    fs::create_dir(format!("../{}/src/js", config.name))?;

    // add icons
    let img = image_io::Reader::open(config.icon)?.decode()
        .unwrap_or_else(|err| panic!("could't decode the image: {}", err));
    resize_and_save_icon(&config.name, &img, 512);
    resize_and_save_icon(&config.name, &img, 192);

    // TODO clean up (=delete created folders), if there's been an error while dealing w the imgs

    // add files
    fs::copy(
        "./src/assets/service-worker-init.js",
        format!("../{}/src/js/service-worker-init.js", config.name)
    )?;

    fs::copy(
        "./src/assets/index.css",
        format!("../{}/src/index.css", config.name)
    )?;

    let html_src = fs::read_to_string("./src/assets/templates/index.html")?
        .replace("<APP_NAME>", &config.name)
        .replace("<APP_THEME_COLOR>", &config.theme_color)
        .replace("<APP_DESCRIPTION>", &config.background_color);
    fs::write(
        format!("../{}/src/index.html", config.name),
        html_src
    )?;

    let manifest_src = fs::read_to_string("./src/assets/templates/manifest.json")?
        .replace("<APP_NAME>", &config.name)
        .replace("<APP_NAME_CAMELIZED>", &format_app_name(&config.name))
        .replace("<APP_ORIENTATION>", &config.orientation)
        .replace("<APP_THEME_COLOR>", &config.theme_color)
        .replace("<APP_BG_COLOR>", &config.background_color);
    fs::write(
        format!("../{}/src/manifest.json", config.name),
        manifest_src
    )?;

    let mut js_src = fs::read_to_string("./src/assets/templates/index.js")?;
    if config.wakelock {
        js_src.push_str("\n");
        js_src.push_str("import './js/wakelock.js';");
        fs::copy(
            "./src/assets/wakelock.js",
            format!("../{}/src/js/wakelock.js", config.name)
        )?;
    }
    fs::write(
        format!("../{}/src/index.js", config.name),
        js_src
    )?;

    // TODO add deno-based build script
    // TODO add node-based build setup and add flag to set that up
    let sw_src = fs::read_to_string("./src/assets/templates/service-worker.js")?
        .replace("<APP_NAME>", &config.name);
    fs::write(
        format!("../{}/src/service-worker.js", config.name),
        sw_src
    )?;

    let readme_src = fs::read_to_string("./src/assets/templates/README.md")?
        .replace("<APP_NAME>", &config.name);
    fs::write(
        format!("../{}/README.md", config.name),
        readme_src
    )?;

    // TODO add .gitignore and init as git repo

    Ok(())
}

fn resize_and_save_icon(app_name: &str, img: &image::DynamicImage, size: u32) {
    imageops::resize(img, size, size, imageops::FilterType::CatmullRom)
        .save(format!("../{}/src/images/icons-{}.png", app_name, size))
        .unwrap_or_else(|err| panic!("couldn't save img: {}", err));
}

fn format_app_name(app_name: &str) -> String {
    app_name
        .split("-")
        .enumerate()
        .map(|(index, part)| if index == 0 {
            String::from(part)
        } else {
            some_kind_of_uppercase_first_letter(&part)
        })
        .collect()
}

// https://stackoverflow.com/a/38406885/7732282
fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
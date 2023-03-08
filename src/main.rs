use image::{imageops, io as image_io, ImageFormat::Png};
use std::fs;
use std::io::Cursor;

use pwa_shell_generator::Config;

fn main() -> std::io::Result<()> {
    let config = Config::new().unwrap();
    let icon = if config.icon == "default" {
        // TODO use a pngReader
        let mut reader =
            image_io::Reader::new(Cursor::new(include_bytes!("./assets/default-icon.png")));
        reader.set_format(Png);
        reader.decode().unwrap()
    } else {
        image_io::Reader::open(config.icon)?
            .decode()
            .unwrap_or_else(|err| panic!("couldn't decode the icon: {}", err))
    };
    let target_dir = format!("./{}", config.name);

    fs::create_dir(&target_dir)?;
    fs::create_dir(format!("{}/src", target_dir))?;
    fs::create_dir(format!("{}/src/images", target_dir))?;
    fs::create_dir(format!("{}/src/js", target_dir))?;

    resize_and_save_icon(&target_dir, &icon, 512);
    resize_and_save_icon(&target_dir, &icon, 192);

    fs::write(
        format!("{}/src/js/service-worker-init.js", target_dir),
        include_bytes!("./assets/service-worker-init.js"),
    )?;
    fs::write(
        format!("{}/src/index.css", target_dir),
        include_bytes!("./assets/index.css"),
    )?;

    let html_src = std::str::from_utf8(include_bytes!("./assets/templates/index.html"))
        .unwrap()
        .replace("<APP_NAME>", &config.name)
        .replace("<APP_THEME_COLOR>", &config.theme_color)
        .replace("<APP_DESCRIPTION>", &config.description);
    fs::write(format!("{}/src/index.html", target_dir), html_src)?;

    let manifest_src = std::str::from_utf8(include_bytes!("./assets/templates/manifest.json"))
        .unwrap()
        .replace("<APP_NAME>", &config.name)
        .replace("<APP_NAME_CAMELIZED>", &format_app_name(&config.name))
        .replace("<APP_ORIENTATION>", &config.orientation)
        .replace("<APP_THEME_COLOR>", &config.theme_color)
        .replace("<APP_BG_COLOR>", &config.background_color);
    fs::write(format!("{}/src/manifest.json", target_dir), manifest_src)?;

    let mut js_src =
        String::from_utf8(include_bytes!("./assets/templates/index.js").to_vec()).unwrap();
    if config.wakelock {
        js_src.push_str("\n");
        js_src.push_str("import './js/wakelock.js';");
        fs::write(
            format!("{}/src/js/wakelock.js", target_dir),
            include_bytes!("./assets/wakelock.js"),
        )?;
    }
    fs::write(format!("{}/src/index.js", target_dir), js_src)?;

    let sw_src = std::str::from_utf8(include_bytes!("./assets/templates/service-worker.js"))
        .unwrap()
        .replace("<APP_NAME>", &config.name);
    fs::write(format!("{}/src/service-worker.js", target_dir), sw_src)?;

    // TODO replace this w vite based setup
    // TODO option to output zip
    // TODO add wasm-based frontend and enable page
    if config.nodejs_based {
        fs::write(
            format!("{}/gulpfile.js", target_dir),
            include_bytes!("./assets/gulpfile.js"),
        )?;
        let package_json = std::str::from_utf8(include_bytes!("./assets/templates/package.json"))
            .unwrap()
            .replace("<APP_NAME>", &config.name)
            .replace("<APP_DESCRIPTION>", &config.description);
        fs::write(format!("{}/package.json", target_dir), package_json)?;
        fs::write(
            format!("{}/.gitignore", target_dir),
            include_bytes!("./assets/.gitignore"),
        )?;
    }
    // TODO else add deno-based build script

    let readme_src = std::str::from_utf8(include_bytes!("./assets/templates/README.md"))
        .unwrap()
        .replace("<APP_NAME>", &config.name);
    fs::write(format!("{}/README.md", target_dir), readme_src)?;

    Ok(())
}

fn resize_and_save_icon(target_dir: &str, icon: &image::DynamicImage, size: u32) {
    imageops::resize(icon, size, size, imageops::FilterType::CatmullRom)
        .save(format!("{}/src/images/icons-{}.png", target_dir, size))
        .unwrap_or_else(|err| panic!("couldn't save icon: {}", err));
}

// sorta turn kebab-case into camel-case
fn format_app_name(app_name: &str) -> String {
    app_name
        .split("-")
        .enumerate()
        .map(|(index, part)| {
            if index == 0 {
                String::from(part)
            } else {
                some_kind_of_uppercase_first_letter(part)
            }
        })
        .collect()
}

// thx to https://stackoverflow.com/a/38406885/7732282
fn some_kind_of_uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

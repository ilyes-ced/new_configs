use std::{env, path::Path, process::Command};

mod templating;

const MESSAGE: &str = "
    --theme_name=<name of theme in setup/scripts/json | random>
    --wallpaper=<image name in the path ~/Pictures/wallpapers | random | none>
";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("2 args are required type, backend/theme_name wallpaper");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }

    process_theme(args);
}

fn process_theme(args: Vec<String>) {
    let theme_name = if &args[1][..12] == "--theme_name" {
        decide_theme(args[1].split("=").last().unwrap())
    } else if &args[2][..12] == "--theme_name" {
        decide_theme(args[2].split("=").last().unwrap())
    } else if &args[3][..12] == "--theme_name" {
        decide_theme(args[3].split("=").last().unwrap())
    } else {
        println!("theme name not specified is required");
        std::process::exit(1)
    };

    let wallpaper_path: String = if &args[1][..11] == "--wallpaper" {
        decide_wallpaper(args[1].split("=").last().unwrap())
    } else if &args[2][..11] == "--wallpaper" {
        decide_wallpaper(args[2].split("=").last().unwrap())
    } else if &args[3][..11] == "--wallpaper" {
        decide_wallpaper(args[3].split("=").last().unwrap())
    } else {
        println!("wallpaper not specified is required, will leave the current one");
        std::process::exit(1)
    };

    println!("for custom selected\n\twall:{:?}\n\tback:{:?}", wallpaper_path, theme_name);

    //set themes here
    templating::template(Some(theme_name)).unwrap();
    set_wallpaper(wallpaper_path);
}

fn decide_theme(theme_name: &str) -> String {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("new_configs/themes/").to_string_lossy().into_owned()
    });

    let theme = String::from_utf8_lossy(&[config_path.to_string().as_bytes(), theme_name.as_bytes()].concat()).to_string();

    println!("{}", theme);

    if Path::new(&theme).exists() {
        return theme;
    } else {
        println!("selected theme is unavaillable please select a valid theme: {}", theme);
        std::process::exit(1)
    }
}

fn decide_wallpaper(wallpaper: &str) -> String {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("Pictures/wallpapers/").to_string_lossy().into_owned()
    });

    let wall_path: String = String::from_utf8_lossy(&[config_path.as_bytes(), wallpaper.as_bytes()].concat()).to_string();

    if Path::new(&wall_path).exists() {
        wall_path
    } else {
        println!("selected image is unavaillable please select a valid image: {}", wall_path);
        std::process::exit(1)
    }
}

fn set_wallpaper(wallpaper_path: String) {
    let wall_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("Documents/Projects/new_configs/scripts/active/wallpaper").to_string_lossy().into_owned()
    });

    let output = Command::new("rm").arg(&wall_path).output().expect("Failed to execute command");
    println!("=======>>> rm old wallpaper path output");
    println!("status: {}", output.status);
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("=======>>> link new wallpaper path output");
    let output = Command::new("ln").arg(wallpaper_path).arg(&wall_path).output().expect("Failed to execute command");
    println!("status: {}", output.status);
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

use handlebars::Handlebars;
use rand::seq::{IteratorRandom, SliceRandom};
use serde_json::{from_str, json, Value};
use std::{
    env,
    fs::{self, read_to_string, File},
    io::Write,
    os::unix::prelude::OsStrExt,
    path::Path,
    process::Command,
};

// can be "all" or "favs"
// favs for my faviourt themes and all for all
const THEME_DIR: &str = "favs";
const BRIGHTER_VALUE: i64 = 30;

mod templating;

const MESSAGE: &str = "
    --type=<pywal|custom>
        pywal:
            --backend=<wal|colorz|colorthief|random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random>
        custom:
            --theme_name=<name of theme in setup/scripts/json | random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random | none>
";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("3 args are required type, backend/theme_name wallpaper");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }

    if &args[1][..6] == "--type" {
        decide_process(args, 1);
    } else if &args[2][..6] == "--type" {
        decide_process(args, 2);
    } else if &args[3][..6] == "--type" {
        decide_process(args, 3);
    } else {
        println!("type needs to be specified");
        std::process::exit(1)
    }
}

fn decide_process(args: Vec<String>, index: usize) {
    if args[index] == "--type=pywal" {
        process_pywal(args);
    } else if args[index] == "--type=custom" {
        process_custom(args);
    } else {
        println!("specifed wrong type");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }
}

fn process_pywal(args: Vec<String>) {
    let backend = if &args[1][..9] == "--backend" {
        decide_backend(&args[1])
    } else if &args[2][..9] == "--backend" {
        decide_backend(&args[2])
    } else if &args[3][..9] == "--backend" {
        decide_backend(&args[3])
    } else {
        println!("backend not specified is required");
        std::process::exit(1)
    };

    let wallpaper_path = if &args[1][..11] == "--wallpaper" {
        decide_wallpaper(&args[1])
    } else if &args[2][..11] == "--wallpaper" {
        decide_wallpaper(&args[2])
    } else if &args[3][..11] == "--wallpaper" {
        decide_wallpaper(&args[3])
    } else {
        println!("wallpaper not specified is required");
        std::process::exit(1)
    };

    println!("for pywal selected\n\twall:{}\n\tback:{}", wallpaper_path, backend);

    // apparantly putting the entire command in a string doesnt work yopu need to put each part of the command in an .arg
    //let output = Command::new(format!("wal --backend {} -i {} && ~/new_configs/scripts/target/release/gtk_theme", backend, wallpaper_path))
    //let cmd = format!("wal --backend {} -i {} && ~/new_configs/scripts/target/release/gtk_theme", backend, wallpaper_path);

    let output = Command::new("wal")
        //.arg(format!("--backend {}", backend))
        .arg("-s")
        .arg("--backend")
        .arg(backend)
        .arg("-i")
        .arg(&wallpaper_path)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    //    .output()
    //    .expect("Failed to execute command");
    //println!("status: {}", output.status);
    //println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    //println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // here we do the templating with the aquired theme ~/.cache/wal/colors.json
    pywal_json_to_json();
    templating::template(None).unwrap();

    // println!("setting the gtk themes\n");
    // gtk_theme::main();
    set_wallpaper(Some(wallpaper_path));
    //let output = Command::new("~/new_configs/scripts/target/release/gtk_theme")
}

fn decide_backend(backend: &String) -> &'static str {
    let backend = backend.split("=").last().unwrap();
    if backend == "wal" {
        "wal"
    } else if backend == "colorz" {
        "colorz"
    } else if backend == "colorthief" {
        "colorthief"
    } else if backend == "random" {
        let arr = vec!["colorz", "wal", "colorthief"];
        let random = arr.choose(&mut rand::thread_rng());
        random.unwrap()
    } else {
        println!("specified backend is not available required");
        println!("{}", MESSAGE);
        std::process::exit(1)
    }
}

fn decide_wallpaper(wallpaper: &String) -> String {
    let wallpaper = wallpaper.split("=").last().unwrap();
    if wallpaper == "random" {
        let mut rng = rand::thread_rng();
        let files = fs::read_dir("~/Pictures/wallpapers/").unwrap();
        let file = files.choose(&mut rng).unwrap().unwrap();

        String::from_utf8_lossy(&[b"~/Pictures/wallpapers/", file.file_name().as_bytes()].concat()).to_string()
    } else {
        let wall_path: String = String::from_utf8_lossy(&[b"~/Pictures/wallpapers/", wallpaper.as_bytes()].concat()).to_string();
        if Path::new(&wall_path).exists() {
            wall_path
        } else {
            println!("image names doesnt exist choose a valid image");
            std::process::exit(1)
        }
    }
}

fn pywal_json_to_json() {
    let binding = read_to_string("~/.cache/wal/colors.json").unwrap();
    let colors = binding.as_str();
    let s: Value = from_str(colors).expect("JSON was not well-formatted");

    let reg = Handlebars::new();
    let template = fs::read_to_string("~/new_configs/scripts/templates/json.json").unwrap();
    println!("{:#?}", s["colors"]["color8"].as_str().unwrap());
    let new_json = reg
        .render_template(
            &template,
            &json!({
                "color0": s["colors"]["color0"],
                "color1": s["colors"]["color1"],
                "color2": s["colors"]["color2"],
                "color3": s["colors"]["color3"],
                "color4": s["colors"]["color4"],
                "color5": s["colors"]["color5"],
                "color6": s["colors"]["color6"],
                "color7": s["colors"]["color7"],
                "color8": s["colors"]["color8"],
                "color9": s["colors"]["color9"],
                "color10": s["colors"]["color10"],
                "color11": s["colors"]["color11"],
                "color12": s["colors"]["color12"],
                "color13": s["colors"]["color13"],
                "color14": s["colors"]["color14"],
                "color15": s["colors"]["color15"],
                //? not needed as pywal supports 16 colors now
                // "color8": make_bright(s["colors"]["color8"].as_str().unwrap()).unwrap(),
                // "color9": make_bright(s["colors"]["color9"].as_str().unwrap()).unwrap(),
                // "color10": make_bright(s["colors"]["color10"].as_str().unwrap()).unwrap(),
                // "color11": make_bright(s["colors"]["color11"].as_str().unwrap()).unwrap(),
                // "color12": make_bright(s["colors"]["color12"].as_str().unwrap()).unwrap(),
                // "color13": make_bright(s["colors"]["color13"].as_str().unwrap()).unwrap(),
                // "color14": make_bright(s["colors"]["color14"].as_str().unwrap()).unwrap(),
                // "color15": make_bright(s["colors"]["color15"].as_str().unwrap()).unwrap(),
                "background": s["special"]["background"],
                "foreground": s["special"]["foreground"],
                "cursor": s["special"]["cursor"],
            }),
        )
        .unwrap();
    let mut file = File::create("~/new_configs/scripts/themes/active/active.json").unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
}

fn custom_wallpaper_selection(wallpaper: &String) -> Option<String> {
    let wall = wallpaper.split("=").last().unwrap();
    if wall == "none" {
        None
    } else {
        let gg = decide_wallpaper(wallpaper);
        Some(gg)
    }
}
fn decide_theme_name(theme_name: &str) -> String {
    if theme_name == "random" {
        //select random theme
        let mut rng = rand::thread_rng();
        let file = if THEME_DIR == "favs" {
            let files = fs::read_dir("~/new_configs/scripts/themes/favs/").unwrap();
            files.choose(&mut rng).unwrap().unwrap()
        } else if THEME_DIR == "favs" {
            let files = fs::read_dir("~/new_configs/scripts/themes/json/").unwrap();
            files.choose(&mut rng).unwrap().unwrap()
        } else {
            println!("wrong THEME_DIR",);
            std::process::exit(1)
        };
        String::from_utf8_lossy(&[b"~/new_configs/scripts/themes/json/", file.file_name().as_bytes()].concat()).to_string()
    } else {
        let theme = String::from_utf8_lossy(&[b"~/new_configs/scripts/themes/json/", theme_name.as_bytes(), b".json"].concat()).to_string();
        if Path::new(&theme).exists() {
            return theme;
        } else {
            println!("selected theme is unavaillable please select a valid theme:: {}", theme);
            std::process::exit(1)
        }
    }
}

fn process_custom(args: Vec<String>) {
    let theme_name = if &args[1][..12] == "--theme_name" {
        decide_theme_name(args[1].split("=").last().unwrap())
    } else if &args[2][..12] == "--theme_name" {
        decide_theme_name(args[2].split("=").last().unwrap())
    } else if &args[3][..12] == "--theme_name" {
        decide_theme_name(args[3].split("=").last().unwrap())
    } else {
        println!("wallpaper not specified is required");
        std::process::exit(1)
    };

    let wallpaper_path: Option<String> = if &args[1][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[1])
    } else if &args[2][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[2])
    } else if &args[3][..11] == "--wallpaper" {
        custom_wallpaper_selection(&args[3])
    } else {
        println!("wallpaper not specified is required, will leave the current one");
        None
        //std::process::exit(1)
    };

    println!("for custom selected\n\twall:{:?}\n\tback:{:?}", wallpaper_path, theme_name);

    //set themes here
    templating::template(Some(theme_name)).unwrap();
    // println!("setting the gtk themes\n");
    // gtk_theme::main();
    set_wallpaper(wallpaper_path);

    //copy the selected theme to pywal cache
}

fn set_wallpaper(wallpaper_path: Option<String>) {
    match wallpaper_path {
        Some(path) => {
            let output = Command::new("rm").arg("~/new_configs/scripts/themes/active/wallpaper").output().expect("Failed to execute command");
            println!("status: {}", output.status);
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            let output = Command::new("ln").arg(path).arg("~/new_configs/scripts/themes/active/wallpaper").output().expect("Failed to execute command");
            println!("status: {}", output.status);
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        None => {}
    }
}
//
// fn make_bright(color: &str) -> Result<String, ()> {
//     let r = &color[1..3];
//     let g = &color[3..5];
//     let b = &color[5..7];
//     //println!("{} {} {}",r,g,b);
//     let mut r_brt = i64::from_str_radix(r, 16).unwrap() + (BRIGHTER_VALUE);
//     let mut g_brt = i64::from_str_radix(g, 16).unwrap() + (BRIGHTER_VALUE);
//     let mut b_brt = i64::from_str_radix(b, 16).unwrap() + (BRIGHTER_VALUE);
//     if r_brt > 255 {
//         r_brt = 255;
//     }
//     if g_brt > 255 {
//         g_brt = 255;
//     }
//     if b_brt > 255 {
//         b_brt = 255;
//     }
//     //println!("result !::::: {}", String::from(format!("{:x}{:x}{:x}", r_brt, g_brt, b_brt)));
//
//     Ok(String::from(format!("#{:x}{:x}{:x}", r_brt, g_brt, b_brt)))
// }
//

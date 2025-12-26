use handlebars::Handlebars;
use serde_json::{from_str, json, Value};
use std::error::Error;
use std::path::Path;

use std::{
    env,
    fs::{self, read_to_string, File},
    io::Write,
};

/////////////////////////////////
/////////////////////////////////
/////////////////////////////////
//  add dunstrc
/////////////////////////////////
/////////////////////////////////
/////////////////////////////////

pub fn template(theme_name: Option<String>) -> Result<(), Box<dyn Error>> {
    // reads files from themes/json and create all the color scheme files for alacritty i3 polybar ......
    let json_values = match theme_name {
        Some(name) => read_scheme_json(&Path::new(&name)).unwrap(),
        None => {
            let home = home::home_dir().unwrap();
            let json_path = home.join("Documents/Projects/new_configs/scripts/themes/active/active.json").to_string_lossy().into_owned();
            read_scheme_json(&Path::new(&json_path)).unwrap()
        }
    };
    // let _ = create_json(&json_values).unwrap();
    // let _ = create_alacritty(&json_values).unwrap();
    // let _ = create_rofi(&json_values).unwrap();
    // let _ = create_polybar(&json_values).unwrap();
    // let _ = create_i3(&json_values).unwrap();
    // let _ = create_i3_bar(&json_values).unwrap();
    // let _ = create_zellij(&json_values).unwrap();
    // let _ = create_dunstrc(&json_values).unwrap();
    // let _ = create_dunstrc(&json_values).unwrap();

    let template_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("Documents/Projects/new_configs/scripts/templates/").to_string_lossy().into_owned()
    });
    let results_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("Documents/Projects/new_configs/scripts/active/").to_string_lossy().into_owned()
    });

    let _ = create_file(&json_values, format!("{}{}", template_path, "hyprland.conf"), format!("{}{}", results_path, "hyprland.conf")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "dunstrc"), format!("{}{}", results_path, "dunstrc")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "waybar.css"), format!("{}{}", results_path, "waybar.css")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "wlogout.css"), format!("{}{}", results_path, "wlogout.css")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "rofi.rasi"), format!("{}{}", results_path, "rofi.rasi")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "starship.toml"), format!("{}{}", results_path, "starship.toml")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "zellij.kdl"), format!("{}{}", results_path, "zellij.kdl")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "alacritty.toml"), format!("{}{}", results_path, "alacritty.toml")).unwrap();

    Ok(())
}

fn create_file(s: &Value, template: String, result: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();

    println!("{}", format!("{}ff", s["color5"].as_str().expect("Expected a string")));
    println!("{}", format!("{}ff", s["background"].as_str().expect("Expected a string")));
    println!("");

    let template = fs::read_to_string(template).unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": s["color0"],
            "color1": s["color1"],
            "color2": s["color2"],
            "color3": s["color3"],
            "color4": s["color4"],
            "color5": s["color5"],
            "color6": s["color6"],
            "color7": s["color7"],
            "color8": s["color8"],
            "color9": s["color9"],
            "color10": s["color10"],
            "color11": s["color11"],
            "color12": s["color12"],
            "color13": s["color13"],
            "color14": s["color14"],
            "color15": s["color15"],
            "shade0": make_bright(s["background"].as_str().expect("Expected a string"), 10).unwrap(),
            "shade1": make_bright(s["background"].as_str().expect("Expected a string"), 9).unwrap(),
            "shade2": make_bright(s["background"].as_str().expect("Expected a string"), 8).unwrap(),
            "shade3": make_bright(s["background"].as_str().expect("Expected a string"), 7).unwrap(),
            "shade4": make_bright(s["background"].as_str().expect("Expected a string"), 6).unwrap(),
            "shade5": make_bright(s["background"].as_str().expect("Expected a string"), 5).unwrap(),
            "shade6": make_bright(s["background"].as_str().expect("Expected a string"), 4).unwrap(),
            "shade7": make_bright(s["background"].as_str().expect("Expected a string"), 3).unwrap(),
            "shade8": make_bright(s["background"].as_str().expect("Expected a string"), 2).unwrap(),
            "shade9": make_bright(s["background"].as_str().expect("Expected a string"), 1).unwrap(),
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
            "hypr_active_border": format!("{}ff",s["color5"].as_str().expect("Expected a string").strip_prefix('#').unwrap()),
            "hypr_background": format!("{}ff",s["background"].as_str().expect("Expected a string").strip_prefix('#').unwrap()),

        }),
    )?;
    let mut file = File::create(result).unwrap();
    file.write_all(new_json.as_bytes()).unwrap();
    Ok(())
}

fn read_scheme_json(path: &Path) -> Result<Value, ()> {
    let binding = read_to_string(&path).unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}

const BRIGHTER_VALUE: i64 = 20;
fn make_bright(color: &str, level: i64) -> Result<String, ()> {
    let r = &color[1..3];
    let g = &color[3..5];
    let b = &color[5..7];
    //println!("{} {} {}",r,g,b);
    let mut r_brt = i64::from_str_radix(r, 16).unwrap() + (BRIGHTER_VALUE * level);
    let mut g_brt = i64::from_str_radix(g, 16).unwrap() + (BRIGHTER_VALUE * level);
    let mut b_brt = i64::from_str_radix(b, 16).unwrap() + (BRIGHTER_VALUE * level);
    if r_brt > 255 {
        r_brt = 255;
    }
    if g_brt > 255 {
        g_brt = 255;
    }
    if b_brt > 255 {
        b_brt = 255;
    }
    //println!("result !::::: {}", String::from(format!("{:x}{:x}{:x}", r_brt, g_brt, b_brt)));

    Ok(String::from(format!("#{:x}{:x}{:x}", r_brt, g_brt, b_brt)))
}

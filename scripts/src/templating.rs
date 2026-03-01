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
    let json_path = match theme_name {
        Some(name) => name,
        None => {
            let home = home::home_dir().unwrap();
            let json_path = home.join("new_configs/scripts/active/active.json").to_string_lossy().into_owned();
            json_path
        }
    };

    println!("================== {:?}", json_path);

    let json_values = read_scheme_json(Path::new(&json_path)).unwrap();

    println!("================== {:?}", json_values);

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
        home.join("new_configs/scripts/templates/").to_string_lossy().into_owned()
    });
    let results_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("new_configs/scripts/active/").to_string_lossy().into_owned()
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

    println!(".........{}", s["background"].as_str().expect("Expected a string"));
    println!(".........{}", hex_to_rgba(s["background"].as_str().expect("Expected a string")));
    println!(".........{}", hex_to_rgba(s["background"].as_str().expect("Expected a string")));
    println!(".........{}", hex_to_rgba(s["background"].as_str().expect("Expected a string")));

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
            "hypr_active_border": s["color5"].as_str().expect("Expected a string").strip_prefix('#').unwrap(),
            "hypr_background": s["background"].as_str().expect("Expected a string").strip_prefix('#').unwrap(),
            "wlogout_background": hex_to_rgba(s["background"].as_str().expect("Expected a string")),

            // !!! very lazy behavior
            "zellij_color0": hex_to_zellij_rgb(s["color0"].as_str().expect("Expected a string")),
            "zellij_color1": hex_to_zellij_rgb(s["color1"].as_str().expect("Expected a string")),
            "zellij_color2": hex_to_zellij_rgb(s["color2"].as_str().expect("Expected a string")),
            "zellij_color3": hex_to_zellij_rgb(s["color3"].as_str().expect("Expected a string")),
            "zellij_color4": hex_to_zellij_rgb(s["color4"].as_str().expect("Expected a string")),
            "zellij_color5": hex_to_zellij_rgb(s["color5"].as_str().expect("Expected a string")),
            "zellij_color6": hex_to_zellij_rgb(s["color6"].as_str().expect("Expected a string")),
            "zellij_color7": hex_to_zellij_rgb(s["color7"].as_str().expect("Expected a string")),
            "zellij_color8": hex_to_zellij_rgb(s["color8"].as_str().expect("Expected a string")),
            "zellij_color9": hex_to_zellij_rgb(s["color9"].as_str().expect("Expected a string")),
            "zellij_color10": hex_to_zellij_rgb(s["color10"].as_str().expect("Expected a string")),
            "zellij_color11": hex_to_zellij_rgb(s["color11"].as_str().expect("Expected a string")),
            "zellij_color12": hex_to_zellij_rgb(s["color12"].as_str().expect("Expected a string")),
            "zellij_color13": hex_to_zellij_rgb(s["color13"].as_str().expect("Expected a string")),
            "zellij_color14": hex_to_zellij_rgb(s["color14"].as_str().expect("Expected a string")),
            "zellij_color15": hex_to_zellij_rgb(s["color15"].as_str().expect("Expected a string")),
            "zellij_background": hex_to_zellij_rgb(s["background"].as_str().expect("Expected a string")),
            "zellij_foreground": hex_to_zellij_rgb(s["foreground"].as_str().expect("Expected a string")),
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

fn hex_to_rgba(hex: &str) -> String {
    if hex.len() != 7 || !hex.starts_with('#') {
        panic!("Invalid hex format. Use #RRGGBB");
    }
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    //? wlogout background alpha is determined here
    format!("rgba({}, {}, {}, 0.8)", r, g, b)
}

fn hex_to_zellij_rgb(hex: &str) -> String {
    if hex.len() != 7 || !hex.starts_with('#') {
        panic!("Invalid hex format. Use #RRGGBB");
    }
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    println!("zelllij RGB{}", format!("{} {} {}", r, g, b));
    format!("{} {} {}", r, g, b)
}

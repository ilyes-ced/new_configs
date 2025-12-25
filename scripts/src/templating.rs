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
        None => read_scheme_json(&Path::new("~/new_configs/scripts/themes/active/active.json")).unwrap(),
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
        home.join("new_configs/scripts/templates/").to_string_lossy().into_owned()
    });
    let results_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        let home = home::home_dir().unwrap();
        home.join("new_configs/scripts/active/").to_string_lossy().into_owned()
    });

    let _ = create_file(&json_values, format!("{}{}", template_path, "json.json"), format!("{}{}", results_path, "active.json")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "alacritty.toml"), format!("{}{}", results_path, "alacritty.toml")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "colors.ini"), format!("{}{}", results_path, "colors.ini")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "rofi.rasi"), format!("{}{}", results_path, "rofi.rasi")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "colors"), format!("{}{}", results_path, "colors")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "bar_config"), format!("{}{}", results_path, "bar_config")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "config.kdl"), format!("{}{}", results_path, "config.kdl")).unwrap();
    let _ = create_file(&json_values, format!("{}{}", template_path, "dunstrc"), format!("{}{}", results_path, "dunstrc")).unwrap();

    Ok(())
}

fn create_file(s: &Value, template: String, result: String) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
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
            "background": s["background"],
            "foreground": s["foreground"],
            "cursor": s["cursor"],
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

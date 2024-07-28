use serde_json::{from_str, Value};
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Write};
const BRIGHTER_VALUE: i64 = 30;

// background =====> make it brighter 4 times and take the first 5 colors
// foreground is text color5
// colors

pub fn main() {
    // change the colors.css files in ~/.config/gtk-3.0 and gtk-4.0
    let _ = std::process::Command::new("sleep")
        .arg("5")
        .spawn()
        .unwrap();

    let gtk3 = File::open("/home/dude/.config/gtk-3.0/colors.css").unwrap();

    let reader3 = BufReader::new(gtk3);

    let mut old_colors15: Vec<String> = Vec::new();
    let mut old_text: Vec<String> = Vec::new();
    let mut colors_pushed = 0;

    for line in reader3.lines() {
        let line = line.unwrap();
        if line.len() >= 13 {
            if &line[..13] == "@define-color" {
                let comps: Vec<&str> = line.split(" ").collect();
                let s = comps[2].replace("#", "").replace(";", "");
                if colors_pushed == 15 {
                    old_text.push(s)
                } else {
                    old_colors15.push(s);
                    colors_pushed += 1
                }
            }
        }
    }

    //let mut file = File::create("text.txt").unwrap();
    //file.write_all(format!("{:#?}", read_scheme()).as_bytes());

    let color_scheme = read_scheme().unwrap();

    // [1..] removes #
    let bg_color = &color_scheme["colors"]["color0"].as_str().unwrap()[1..];

    let color0 = bg_color;
    let color1 = make_bright(bg_color, 1).unwrap();
    let color2 = make_bright(bg_color, 2).unwrap();
    let color3 = make_bright(bg_color, 3).unwrap();
    let color4 = make_bright(bg_color, 4).unwrap();

    let color5 = &color_scheme["colors"]["color7"].as_str().unwrap()[1..];

    let color6 = &color_scheme["colors"]["color1"].as_str().unwrap()[1..];
    let color7 = &color_scheme["colors"]["color2"].as_str().unwrap()[1..];
    let color8 = &color_scheme["colors"]["color3"].as_str().unwrap()[1..];
    let color9 = &color_scheme["colors"]["color4"].as_str().unwrap()[1..];
    let color10 = &color_scheme["colors"]["color5"].as_str().unwrap()[1..];
    let color11 = &color_scheme["colors"]["color6"].as_str().unwrap()[1..];

    let color12 = make_bright(&color_scheme["colors"]["color8"].as_str().unwrap()[1..], 0).unwrap();
    let color13 = make_bright(&color_scheme["colors"]["color8"].as_str().unwrap()[1..], 1).unwrap();
    let color14 = make_bright(&color_scheme["colors"]["color8"].as_str().unwrap()[1..], 2).unwrap();

    let colors = format!("@define-color color0 #{};\n@define-color color1 #{};\n@define-color color2 #{};\n@define-color color3 #{};\n@define-color color4 #{};\n@define-color color5 #{};\n@define-color color6 #{};\n@define-color color7 #{};\n@define-color color8 #{};\n@define-color color9 #{};\n@define-color color10 #{};\n@define-color color11 #{};\n@define-color color12 #{};\n@define-color color13 #{};\n@define-color color14 #{};\n\n@define-color text7 #{}{};\n@define-color text5 #{}{};\n@define-color text3 #{}{};\n@define-color text12 #{}{};\n@define-color text1 #{}{};\n@define-color text08 #{}{};\n@define-color text04 #{}{};\n@define-color alt_text #{};", 
        color0,
        color1,
        color2,
        color3,
        color4,
        color5,
        color6,
        color7,
        color8,
        color9,
        color10,
        color11,
        color12,
        color13,
        color14,
        color5, 70,
        color5, 50,
        color5, 30,
        color5, 12,
        color5, 10,
        color5, 08,
        color5, 04,
        color5,
    );

    let path = "text.txt";
    let mut output = File::create(path).unwrap();
    let mut output3 = File::create("/home/dude/.config/gtk-3.0/colors.css").unwrap();
    let mut output4 = File::create("/home/dude/.config/gtk-4.0/colors.css").unwrap();
    let _ = write!(output, "{}", colors);
    let _ = write!(output3, "{}", colors);
    let _ = write!(output4, "{}", colors);
}

fn read_scheme() -> Result<Value, ()> {
    // "/home/dude/.cache/wal/colors.json" for pywall theming
    // "/home/dude/.json" for pywall theming
    let binding = read_to_string("/home/dude/.cache/wal/colors.json").unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}

fn make_bright(color: &str, brightness_lvl: i64) -> Result<String, ()> {
    let r = &color[0..2];
    let g = &color[2..4];
    let b = &color[4..6];

    let r_brt = i64::from_str_radix(r, 16).unwrap() + (BRIGHTER_VALUE * brightness_lvl);
    let g_brt = i64::from_str_radix(g, 16).unwrap() + (BRIGHTER_VALUE * brightness_lvl);
    let b_brt = i64::from_str_radix(b, 16).unwrap() + (BRIGHTER_VALUE * brightness_lvl);

    Ok(String::from(format!("{:x}{:x}{:x}", r_brt, g_brt, b_brt)))
}

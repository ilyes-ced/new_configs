use handlebars::Handlebars;
use serde_json::json;
use serde_json::{from_str, Value};
use std::error::Error;
use std::fs::File;
use std::fs::{self, read_to_string};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

fn main() {
    // jsons
    let paths = fs::read_dir("./jsons").unwrap();
    for path in paths {
        let _ = create_json(path.unwrap().path()).unwrap();
    }

    // propreties
    let paths = fs::read_dir("./propreties").unwrap();
    for path in paths {
        create_propreties(path.unwrap().path()).unwrap();
    }
}

fn read_scheme_json(path: &PathBuf) -> Result<Value, ()> {
    let binding = read_to_string(path).unwrap();
    let colors = binding.as_str();
    let json: Value = from_str(colors).expect("JSON was not well-formatted");
    Ok(json)
}

fn create_json(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let reg = Handlebars::new();
    // render without register
    let template = fs::read_to_string("templates/json.json").unwrap();
    let json_values = read_scheme_json(&path).unwrap();

    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": json_values["color_01"],
            "color1": json_values["color_02"],
            "color2": json_values["color_03"],
            "color3": json_values["color_04"],
            "color4": json_values["color_05"],
            "color5": json_values["color_06"],
            "color6": json_values["color_07"],
            "color7": json_values["color_08"],
            "color8": json_values["color_09"],
            "color9": json_values["color_10"],
            "color10": json_values["color_11"],
            "color11": json_values["color_12"],
            "color12": json_values["color_13"],
            "color13": json_values["color_14"],
            "color14": json_values["color_15"],
            "color15": json_values["color_16"],
            "background": json_values["background"],
            "foreground": json_values["foreground"],
            "cursor": json_values["cursor"],
        }),
    )?;

    let binding = [
        b"j_",
        path.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(" ", "_")
            .replace("-", "_")
            .to_lowercase()
            .as_str()
            .as_bytes(),
    ]
    .concat();
    let pp = String::from_utf8_lossy(&binding);
    let mut file = File::create(["themes/json/", &pp].iter().collect::<PathBuf>())?;
    file.write_all(new_json.as_bytes())?;

    Ok(())
}

fn create_propreties(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(&path).unwrap();
    let file_reader = BufReader::new(file);
    let mut temp_prop: [String; 19] = [
        String::from("color0"),
        String::from("color1"),
        String::from("color2"),
        String::from("color3"),
        String::from("color4"),
        String::from("color5"),
        String::from("color6"),
        String::from("color7"),
        String::from("color8"),
        String::from("color9"),
        String::from("color10"),
        String::from("color11"),
        String::from("color12"),
        String::from("color13"),
        String::from("color14"),
        String::from("color15"),
        String::from("foreground"),
        String::from("background"),
        String::from("cursor"),
    ];

    for line in file_reader.lines() {
        let line = line.unwrap();
        if line != "" {
            let mut col = line.split("=");
            if &line[..5] == "color" {
                let col_ind = col.next().unwrap();
                let hex_val = col.next().unwrap();
                let ind = col_ind[5..].parse::<usize>().unwrap().clone();
                temp_prop[ind] = String::from(hex_val);
            } else if &line[..4] == "back" {
                temp_prop[16] = String::from(col.last().unwrap());
            } else if &line[..4] == "fore" {
                temp_prop[17] = String::from(col.last().unwrap());
            } else if &line[..4] == "curs" {
                temp_prop[18] = String::from(col.last().unwrap());
            }
        }
    }

    let reg = Handlebars::new();
    let template = fs::read_to_string("templates/json.json").unwrap();
    let new_json = reg.render_template(
        &template,
        &json!({
            "color0": temp_prop[0],
            "color1": temp_prop[1],
            "color2": temp_prop[2],
            "color3": temp_prop[3],
            "color4": temp_prop[4],
            "color5": temp_prop[5],
            "color6": temp_prop[6],
            "color7": temp_prop[7],
            "color8": temp_prop[8],
            "color9": temp_prop[9],
            "color10": temp_prop[10],
            "color11": temp_prop[11],
            "color12": temp_prop[12],
            "color13": temp_prop[13],
            "color14": temp_prop[14],
            "color15": temp_prop[15],
            "background": temp_prop[16],
            "foreground": temp_prop[17],
            "cursor": temp_prop[18],
        }),
    )?;

    // i dont even know it got like this
    // to create the new file with the .json extention
    let file_name = path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(" ", "_")
        .to_lowercase();
    let pp = String::from_utf8_lossy(&[b"p_", file_name.as_bytes(), b".json"].concat()).to_string();
    let mut file = File::create(["themes/json/", pp.as_str()].iter().collect::<PathBuf>())?;
    file.write_all(new_json.as_bytes())?;

    Ok(())
}

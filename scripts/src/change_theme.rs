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
use tabled::{
    builder::Builder,
    settings::Style,
    Table,
};

// can be "all" or "favs"
// favs for my faviourt themes and all for all
const THEME_DIR: &str = "favs";
const BRIGHTER_VALUE: i64 = 30;

mod gtk_theme;
mod templating;

const MESSAGE: &str = "
    --type=<pywal|custom>
        pywal:
            --backend=<wal|colorz|colorthief|random>
            --wallpaper=<image name in the path ~/Pictures/wallpapers | random>
        custom
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

    println!(
        "for pywal selected\n\twall:{}\n\tback:{}",
        wallpaper_path, backend
    );

    // apparantly putting the entire command in a string doesnt work yopu need to put each part of the command in an .arg
    //let output = Command::new(format!("wal --backend {} -i {} && /home/dude/setup/scripts/target/release/gtk_theme", backend, wallpaper_path))
    //let cmd = format!("wal --backend {} -i {} && /home/dude/setup/scripts/target/release/gtk_theme", backend, wallpaper_path);

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
    //let output = Command::new("/home/dude/setup/scripts/target/release/gtk_theme")
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
        let files = fs::read_dir("/home/dude/Pictures/wallpapers/").unwrap();
        let file = files.choose(&mut rng).unwrap().unwrap();

        String::from_utf8_lossy(
            &[
                b"/home/dude/Pictures/wallpapers/",
                file.file_name().as_bytes(),
            ]
            .concat(),
        )
        .to_string()
    } else {
        let wall_path: String = String::from_utf8_lossy(
            &[b"/home/dude/Pictures/wallpapers/", wallpaper.as_bytes()].concat(),
        )
        .to_string();
        if Path::new(&wall_path).exists() {
            wall_path
        } else {
            println!("image names doesnt exist choose a valid image");
            std::process::exit(1)
        }
    }
}

fn pywal_json_to_json() {
    let binding = read_to_string("/home/dude/.cache/wal/colors.json").unwrap();
    let colors = binding.as_str();
    let s: Value = from_str(colors).expect("JSON was not well-formatted");

    let reg = Handlebars::new();
    let template = fs::read_to_string("/home/dude/setup/scripts/templates/json.json").unwrap();
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
                "color8": make_bright(s["colors"]["color8"].as_str().unwrap()).unwrap(),
                "color9": make_bright(s["colors"]["color9"].as_str().unwrap()).unwrap(),
                "color10": make_bright(s["colors"]["color10"].as_str().unwrap()).unwrap(),
                "color11": make_bright(s["colors"]["color11"].as_str().unwrap()).unwrap(),
                "color12": make_bright(s["colors"]["color12"].as_str().unwrap()).unwrap(),
                "color13": make_bright(s["colors"]["color13"].as_str().unwrap()).unwrap(),
                "color14": make_bright(s["colors"]["color14"].as_str().unwrap()).unwrap(),
                "color15": make_bright(s["colors"]["color15"].as_str().unwrap()).unwrap(),
                "background": s["special"]["background"],
                "foreground": s["special"]["foreground"],
                "cursor": s["special"]["cursor"],
            }),
        )
        .unwrap();
    let mut file = File::create("/home/dude/setup/scripts/themes/active/active.json").unwrap();
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
        let file = if THEME_DIR == "favs"{
            let files = fs::read_dir("/home/dude/setup/scripts/themes/favs/").unwrap();
            files.choose(&mut rng).unwrap().unwrap()
        }else if THEME_DIR == "favs"{
            let files = fs::read_dir("/home/dude/setup/scripts/themes/json/").unwrap();
            files.choose(&mut rng).unwrap().unwrap()
        }else{
            println!("wrong THEME_DIR",);
            std::process::exit(1)
        };
        String::from_utf8_lossy(
            &[
                b"/home/dude/setup/scripts/themes/json/",
                file.file_name().as_bytes(),
            ]
            .concat(),
        )
        .to_string()
    } else {
        let theme = String::from_utf8_lossy(
            &[
                b"/home/dude/setup/scripts/themes/json/",
                theme_name.as_bytes(),
                b".json",
            ]
            .concat(),
        )
        .to_string();
        if Path::new(&theme).exists() {
            return theme;
        } else {
            print_table();
            println!(
                "selected theme is unavaillable please select a valid theme:: {}",
                theme
            );
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

    println!(
        "for custom selected\n\twall:{:?}\n\tback:{:?}",
        wallpaper_path, theme_name
    );

    //set themes here
    templating::template(Some(theme_name)).unwrap();
    // println!("setting the gtk themes\n");
    // gtk_theme::main();
    set_wallpaper(wallpaper_path);

    //copy the selected theme to pywal cache
}

fn print_table() {
    const ROWS: usize = 91;
    const COLS: usize = 6;
    let mut arr2d: [[&str; COLS]; ROWS] = [[""; COLS]; ROWS];

    for i in 0..ROWS {
        for j in 0..COLS {
            arr2d[i][j] = THEMES_NAMES[i * COLS + j];
        }
    }

    let mut builder = Builder::default();
    for line in THEMES_NAMES.iter() {
        if line.is_empty() {
            continue;
        }

        let words: Vec<_> = line.trim().split_terminator(' ').collect();
        builder.push_record(words);
    }

    let columns = (0..builder.count_columns()).map(|i| i.to_string());
    builder.set_header(columns);

    let table = Table::new(arr2d).with(Style::extended()).to_string();

    println!("{}", table);
    println!("NOTE: seeing all themes requires fullscreen mode (cant get it to display according to term width (annoying))");
}

fn set_wallpaper(wallpaper_path: Option<String>) {
    match wallpaper_path {
        Some(path) => {
            let output = Command::new("rm")
                .arg("/home/dude/setup/scripts/themes/active/wallpaper")
                .output()
                .expect("Failed to execute command");
            println!("status: {}", output.status);
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

            let output = Command::new("ln")
                .arg(path)
                .arg("/home/dude/setup/scripts/themes/active/wallpaper")
                .output()
                .expect("Failed to execute command");
            println!("status: {}", output.status);
            println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        None => {}
    }
}

fn make_bright(color: &str) -> Result<String, ()> {
    let r = &color[1..3];
    let g = &color[3..5];
    let b = &color[5..7];
    //println!("{} {} {}",r,g,b);
    let mut r_brt = i64::from_str_radix(r, 16).unwrap() + (BRIGHTER_VALUE);
    let mut g_brt = i64::from_str_radix(g, 16).unwrap() + (BRIGHTER_VALUE);
    let mut b_brt = i64::from_str_radix(b, 16).unwrap() + (BRIGHTER_VALUE);
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


/*
const FAV_THEMES_NAMES: [&str; 33] = [
    "j_asci",
    "j_adventure_time",
    "j_argonaut",
    "j_ayu_dark",
    "j_breath_darker",
    "j_cai",
    "j_clone_of_ubuntu",
    "j_dark_pastel",
    "j_gooey",
    "j_grape",
    "j_horizon_dark",
    "j_morada",
    "j_rose_pine",
    "j_spring",
    "j_sweet_terminal",
    "j_synthwave_alpha",
    "p_banana_blueberry",
    "p_blue_matrix",
    "p_challengerdeep",
    "p_cyberpunk",
    "p_duckbones",
    "p_floraverse",
    "p_materialocean",
    "p_lovelace",
    "p_lavandula",
    "p_laser",
    "nocturnal_winter",
    "p_neon",
    "p_shaman",
    "p_seashells",
    "p_sakura",
    "p_underthesea",
    "p_underthesea",
];
*/
const THEMES_NAMES: [&str; 546] = [
    "j_3024_day",
    "j_3024_night",
    "j_aci",
    "j_aco",
    "j_adventure_time",
    "j_afterglow",
    "j_alien_blood",
    "j_apprentice",
    "j_argonaut",
    "j_arthur",
    "j_atom",
    "j_aura",
    "j_ayu_dark",
    "j_ayu_light",
    "j_ayu_mirage",
    "j_azu",
    "j_belafonte_day",
    "j_belafonte_night",
    "j_bim",
    "j_birds_of_paradise",
    "j_blazer",
    "j_bluloco_light",
    "j_bluloco_zsh_light",
    "j_borland",
    "j_breath_darker",
    "j_breath_light",
    "j_breath_silverfox",
    "j_breath",
    "j_breeze",
    "j_broadcast",
    "j_brogrammer",
    "j_c64",
    "j_cai",
    "j_catppuccin_frappe",
    "j_catppuccin_latte",
    "j_catppuccin_macchiato",
    "j_catppuccin_mocha",
    "j_chalk",
    "j_chalkboard",
    "j_chameleon",
    "j_ciapre",
    "j_clone_of_ubuntu",
    "j_clrs",
    "j_cobalt_2",
    "j_cobalt_neon",
    "j_colorcli",
    "j_crayon_pony_fish",
    "j_dark_pastel",
    "j_darkside",
    "j_dehydration",
    "j_desert",
    "j_dimmed_monokai",
    "j_dissonance",
    "j_dracula",
    "j_earthsong",
    "j_elemental",
    "j_elementary",
    "j_elic",
    "j_elio",
    "j_espresso_libre",
    "j_espresso",
    "j_everblush",
    "j_everforest_dark",
    "j_everforest_light",
    "j_fairy_floss_dark",
    "j_fairy_floss",
    "j_fishtank",
    "j_flat_remix",
    "j_flat",
    "j_flatland",
    "j_foxnightly",
    "j_freya",
    "j_frontend_delight",
    "j_frontend_fun_forrest",
    "j_frontend_galaxy",
    "j_geohot",
    "j_github",
    "j_gogh",
    "j_gooey",
    "j_google_dark",
    "j_google_light",
    "j_gotham",
    "j_grape",
    "j_grass",
    "j_gruvbox_dark",
    "j_gruvbox_material",
    "j_gruvbox",
    "j_hardcore",
    "j_harper",
    "j_hemisu_dark",
    "j_hemisu_light",
    "j_highway",
    "j_hipster_green",
    "j_homebrew_light",
    "j_homebrew_ocean",
    "j_homebrew",
    "j_horizon_bright",
    "j_horizon_dark",
    "j_hurtado",
    "j_hybrid",
    "j_ibm_3270_high_contrast_",
    "j_ibm3270",
    "j_ic_green_ppl",
    "j_ic_orange_ppl",
    "j_idle_toes",
    "j_ir_black",
    "j_jackie_brown",
    "j_japanesque",
    "j_jellybeans",
    "j_jup",
    "j_kanagawa",
    "j_kibble",
    "j_kokuban",
    "j_laserwave",
    "j_later_this_evening",
    "j_lavandula",
    "j_liquid_carbon_transparent",
    "j_liquid_carbon",
    "j_lunaria_dark",
    "j_lunaria_eclipse",
    "j_lunaria_light",
    "j_maia",
    "j_man_page",
    "j_mar",
    "j_material",
    "j_mathias",
    "j_medallion",
    "j_misterioso",
    "j_molokai",
    "j_mona_lisa",
    "j_mono_amber",
    "j_mono_cyan",
    "j_mono_green",
    "j_mono_red",
    "j_mono_white",
    "j_mono_yellow",
    "j_monokai_dark",
    "j_monokai_pro_ristretto",
    "j_monokai_pro",
    "j_monokai_soda",
    "j_morada",
    "j_n0tch2k",
    "j_neon_night",
    "j_neopolitan",
    "j_nep",
    "j_neutron",
    "j_night_owl",
    "j_nightlion_v1",
    "j_nightlion_v2",
    "j_nighty",
    "j_nord_light",
    "j_nord",
    "j_novel",
    "j_obsidian",
    "j_ocean_dark",
    "j_oceanic_next",
    "j_ollie",
    "j_omni",
    "j_one_dark",
    "j_one_half_black",
    "j_one_light",
    "j_oxocarbon_dark",
    "j_palenight",
    "j_pali",
    "j_panda",
    "j_papercolor_dark",
    "j_papercolor_light",
    "j_paraiso_dark",
    "j_paul_millr",
    "j_pencil_dark",
    "j_pencil_light",
    "j_peppermint",
    "j_pixiefloss",
    "j_pnevma",
    "j_powershell",
    "j_predawn",
    "j_pro",
    "j_purple_people_eater",
    "j_red_alert",
    "j_red_sands",
    "j_relaxed",
    "j_rippedcasts",
    "j_rose_pine_dawn",
    "j_rose_pine_moon",
    "j_rose_pine",
    "j_royal",
    "j_sat",
    "j_sea_shells",
    "j_seafoam_pastel",
    "j_seti",
    "j_shaman",
    "j_shel",
    "j_slate",
    "j_smyck",
    "j_snazzy",
    "j_soft_server",
    "j_solarized_darcula",
    "j_solarized_dark_higher_contrast",
    "j_solarized_dark",
    "j_solarized_light",
    "j_sonokai",
    "j_spacedust",
    "j_spacegray_eighties_dull",
    "j_spacegray_eighties",
    "j_spacegray",
    "j_spring",
    "j_square",
    "j_srcery",
    "j_summer_pop",
    "j_sundried",
    "j_sweet_eliverlara",
    "j_sweet_terminal",
    "j_symphonic",
    "j_synthwave_alpha",
    "j_synthwave",
    "j_teerb",
    "j_tender",
    "j_terminal_basic",
    "j_terminix_dark",
    "j_thayer_bright",
    "j_tin",
    "j_tokyo_night_light",
    "j_tokyo_night_storm",
    "j_tokyo_night",
    "j_tomorrow_night_blue",
    "j_tomorrow_night_bright",
    "j_tomorrow_night_eighties",
    "j_tomorrow_night",
    "j_tomorrow",
    "j_toy_chest",
    "j_treehouse",
    "j_twilight",
    "j_ura",
    "j_urple",
    "j_vag",
    "j_vaughn",
    "j_vibrant_ink",
    "j_vs_code_dark_",
    "j_vs_code_light_",
    "j_warm_neon",
    "j_wez",
    "j_wild_cherry",
    "j_wombat",
    "j_wryan",
    "j_wzoreck",
    "j_zenburn",
    "p_3024_day",
    "p_3024_night",
    "p_abernathy",
    "p_adventure",
    "p_adventuretime",
    "p_afterglow",
    "p_alabaster",
    "p_alienblood",
    "p_andromeda",
    "p_apple_classic",
    "p_arcoiris",
    "p_argonaut",
    "p_arthur",
    "p_ateliersulphurpool",
    "p_atom",
    "p_atomonelight",
    "p_aurora",
    "p_ayu_light",
    "p_ayu_mirage",
    "p_ayu",
    "p_banana_blueberry",
    "p_batman",
    "p_belafonte_day",
    "p_belafonte_night",
    "p_birdsofparadise",
    "p_blazer",
    "p_blue_matrix",
    "p_blueberrypie",
    "p_bluedolphin",
    "p_blulocodark",
    "p_blulocolight",
    "p_borland",
    "p_breeze",
    "p_bright_lights",
    "p_broadcast",
    "p_brogrammer",
    "p_builtin_dark",
    "p_builtin_light",
    "p_builtin_pastel_dark",
    "p_builtin_solarized_dark",
    "p_builtin_solarized_light",
    "p_builtin_tango_dark",
    "p_builtin_tango_light",
    "p_c64",
    "p_calamity",
    "p_catppuccin-frappe",
    "p_catppuccin-latte",
    "p_catppuccin-macchiato",
    "p_catppuccin-mocha",
    "p_cga",
    "p_chalk",
    "p_chalkboard",
    "p_challengerdeep",
    "p_chester",
    "p_ciapre",
    "p_clrs",
    "p_cobalt_neon",
    "p_cobalt2",
    "p_coffee_theme",
    "p_crayonponyfish",
    "p_cyberdyne",
    "p_cyberpunk",
    "p_dark_pastel",
    "p_dark+",
    "p_darkermatrix",
    "p_darkmatrix",
    "p_darkside",
    "p_deep",
    "p_desert",
    "p_dimmedmonokai",
    "p_django",
    "p_djangorebornagain",
    "p_djangosmooth",
    "p_doom_peacock",
    "p_doomone",
    "p_dotgov",
    "p_dracula",
    "p_dracula+",
    "p_duckbones",
    "p_duotone_dark",
    "p_earthsong",
    "p_elemental",
    "p_elementary",
    "p_encom",
    "p_espresso_libre",
    "p_espresso",
    "p_everblush",
    "p_fahrenheit",
    "p_fairyfloss",
    "p_fideloper",
    "p_firefoxdev",
    "p_firewatch",
    "p_fishtank",
    "p_flat",
    "p_flatland",
    "p_floraverse",
    "p_forestblue",
    "p_framer",
    "p_frontenddelight",
    "p_funforrest",
    "p_galaxy",
    "p_galizur",
    "p_github_dark",
    "p_github",
    "p_glacier",
    "p_grape",
    "p_grass",
    "p_grey-green",
    "p_gruvbox_light",
    "p_gruvboxdark",
    "p_gruvboxdarkhard",
    "p_guezwhoz",
    "p_hacktober",
    "p_hardcore",
    "p_harper",
    "p_hax0r_blue",
    "p_hax0r_gr33n",
    "p_hax0r_r3d",
    "p_highway",
    "p_hipster_green",
    "p_hivacruz",
    "p_homebrew",
    "p_hopscotch.256",
    "p_hopscotch",
    "p_hurtado",
    "p_hybrid",
    "p_ic_green_ppl",
    "p_ic_orange_ppl",
    "p_iceberg-dark",
    "p_iceberg-light",
    "p_idea",
    "p_idletoes",
    "p_ir_black",
    "p_jackie_brown",
    "p_japanesque",
    "p_jellybeans",
    "p_jetbrains_darcula",
    "p_jubi",
    "p_kanagawabones",
    "p_kibble",
    "p_kolorit",
    "p_konsolas",
    "p_lab_fox",
    "p_laser",
    "p_later_this_evening",
    "p_lavandula",
    "p_liquidcarbon",
    "p_liquidcarbontransparent",
    "p_liquidcarbontransparentinverse",
    "p_lovelace",
    "p_man_page",
    "p_mariana",
    "p_material",
    "p_materialdark",
    "p_materialdarker",
    "p_materialdesigncolors",
    "p_materialocean",
    "p_mathias",
    "p_matrix",
    "p_medallion",
    "p_midnight-in-mojave",
    "p_mirage",
    "p_misterioso",
    "p_molokai",
    "p_monalisa",
    "p_monokai_remastered",
    "p_monokai_soda",
    "p_monokai_vivid",
    "p_n0tch2k",
    "p_neobones_dark",
    "p_neobones_light",
    "p_neon",
    "p_neopolitan",
    "p_neutron",
    "p_night_owlish_light",
    "p_nightlion_v1",
    "p_nightlion_v2",
    "p_niji",
    "p_nocturnal_winter",
    "p_nord-light",
    "p_nord",
    "p_novel",
    "p_obsidian",
    "p_ocean",
    "p_oceanic-next",
    "p_oceanicmaterial",
    "p_ollie",
    "p_onehalfdark",
    "p_onehalflight",
    "p_operator_mono_dark",
    "p_overnight_slumber",
    "p_palenighthc",
    "p_pandora",
    "p_paraiso_dark",
    "p_paulmillr",
    "p_pencildark",
    "p_pencillight",
    "p_peppermint",
    "p_piatto_light",
    "p_pnevma",
    "p_popping_and_locking",
    "p_primary",
    "p_pro_light",
    "p_pro",
    "p_purple_rain",
    "p_purplepeter",
    "p_rapture",
    "p_raycast_dark",
    "p_raycast_light",
    "p_rebecca",
    "p_red_alert",
    "p_red_planet",
    "p_red_sands",
    "p_relaxed",
    "p_retro",
    "p_rippedcasts",
    "p_rose-pine-dawn",
    "p_rose-pine-moon",
    "p_rose-pine",
    "p_rouge_2",
    "p_royal",
    "p_ryuuko",
    "p_sakura",
    "p_scarlet_protocol",
    "p_seafoam_pastel",
    "p_seashells",
    "p_seoulbones_dark",
    "p_seoulbones_light",
    "p_seti",
    "p_shades-of-purple",
    "p_shaman",
    "p_slate",
    "p_sleepyhollow",
    "p_smyck",
    "p_snazzy",
    "p_softserver",
    "p_solarized_darcula",
    "p_solarized_dark_-_patched",
    "p_solarized_dark_higher_contrast",
    "p_spacedust",
    "p_spacegray_eighties_dull",
    "p_spacegray_eighties",
    "p_spacegray",
    "p_spiderman",
    "p_spring",
    "p_square",
    "p_sublette",
    "p_subliminal",
    "p_sundried",
    "p_symfonic",
    "p_synthwave-everything",
    "p_synthwave",
    "p_synthwavealpha",
    "p_tango_adapted",
    "p_tango_half_adapted",
    "p_teerb",
    "p_terminal_basic",
    "p_thayer_bright",
    "p_the_hulk",
    "p_tinacious_design_(dark)",
    "p_tinacious_design_(light)",
    "p_tokyonight-day",
    "p_tokyonight-storm",
    "p_tokyonight",
    "p_tomorrow_night_blue",
    "p_tomorrow_night_bright",
    "p_tomorrow_night_burns",
    "p_tomorrow_night_eighties",
    "p_tomorrow_night",
    "p_tomorrow",
    "p_toychest",
    "p_treehouse",
    "p_twilight",
    "p_ubuntu",
    "p_ultradark",
    "p_ultraviolent",
    "p_underthesea",
    "p_unikitty",
    "p_urple",
    "p_vaughn",
    "p_vibrantink",
    "p_vimbones",
    "p_violet_dark",
    "p_violet_light",
    "p_warmneon",
    "p_wez",
    "p_whimsy",
    "p_wildcherry",
    "p_wilmersdorf",
    "p_wombat",
    "p_wryan",
    "p_zenbones_dark",
    "p_zenbones_light",
    "p_zenbones",
    "p_zenburn",
    "p_zenburned",
    "p_zenwritten_dark",
    "p_zenwritten_light",
    "",
    "",
];

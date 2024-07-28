use std::fs;

fn main() {
    let paths = fs::read_dir("../i3/pywal").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

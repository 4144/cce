use std::env;
use std::fs::{create_dir, File};
use std::io::Read;
use std::process::Command;

pub fn edit_snippet() -> String {
    let home = env::home_dir().expect("Fatal: unable to find home directory");
    let home_dir = home.as_path().join(".godboltc");
    let godboltc = home_dir.as_path();
    if !godboltc.exists() || !godboltc.is_dir() {
        create_dir(&godboltc).expect("Unable to create ~/.godboltc");
    }
    let main = godboltc.join("main");
    let godboltc = main.as_path();
    if !godboltc.exists() || !godboltc.is_file() {
        File::create(&godboltc).expect(&format!(
            "Could not create file {}",
            godboltc.to_str().unwrap()
        ));
    }
    let mut temp = File::open(&godboltc).expect("Unable to create temp file");
    let editor = match env::var("VISUAL").ok() {
        Some(edit) => edit,
        None => match env::var("EDITOR").ok() {
            Some(edit) => edit,
            None => String::from("nvim"),
        },
    };

    Command::new(editor)
        .arg(godboltc.to_str().unwrap())
        .status()
        .expect("Failed to open editor, please set $VISUAL or $EDITOR.");

    let mut buf = String::new();
    temp.read_to_string(&mut buf).unwrap();
    return buf;
}

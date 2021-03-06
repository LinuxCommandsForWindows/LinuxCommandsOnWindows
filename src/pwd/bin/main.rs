use std::env;

use linux_commands_on_windows::pwd::{
    PWD_HELP_MESSAGE,
    PWD_VERSION_MESSAGE
};

fn main() {
    let mut args = env::args();

    if let Some(arg) = args.nth(1) {
        match &*arg {
            "--help" => {
                println!("{}", PWD_HELP_MESSAGE);
                return;
            },
            "--version" => {
                println!("{}", PWD_VERSION_MESSAGE);
                return;
            },
            _ => ()
        }
    }
    
    let mut dir = env::current_dir().unwrap().to_str().unwrap().replace(":", "").replace(r"\", "/");
    let drive_letter = dir.remove(0);
    dir.insert(0, drive_letter.to_ascii_lowercase());

    println!("/mnt/{}", dir);
}

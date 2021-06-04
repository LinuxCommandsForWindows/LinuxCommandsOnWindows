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

    println!("/mnt/{}", env::current_dir().unwrap().to_str().unwrap().replace(":", "").replace(r"\", "/").to_ascii_lowercase());
}

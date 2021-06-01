use std::env;

use linux_commands_on_windows::whoami;

fn main() {
    let mut commandline_arguments = env::args();

    if let Some(argument) = commandline_arguments.nth(1) {
        match &*argument {
            "--help" => {
                println!("{}", whoami::WHOAMI_HELP_MESSAGE);
                return;
            },
            "--version" => {
                println!("{}", whoami::WHOAMI_VERSION_MESSAGE);
                return;
            }
            _ => {
                println!("whoami: option `{}` is unknown", argument);
                return;
            }
        }
    }

    println!("{}", whoami::GetUsername());
}

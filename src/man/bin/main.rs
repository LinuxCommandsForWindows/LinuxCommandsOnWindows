use std::env;

use linux_commands_on_windows::man;

fn main() {
    let mut command_arguments = env::args();

    let man_directory = man::GetManPagesDirectory();

    if !man_directory.exists() {
        println!(
            "cannot find man pages files at folder `/mnt/{}`; please make sure you extracted the man pages files to that directory",
            man_directory.to_str().unwrap().replace(":", "").replace("\\", "/").to_ascii_lowercase()
        );
        return;
    }

    if let Some(argument) = command_arguments.nth(1) {
        match man::GetManPagesFileContent(man_directory, &*argument) {
            Ok(content) => println!("{}", content),
            Err(error) => {
                if error.to_string() == String::from("The system cannot find the file specified. (os error 2)") {
                    println!("no manual entry for {}", argument);
                    return;
                }

                println!("could not retrieve man page due to an error; error: {}", error.to_string().to_ascii_lowercase())
            }
        }

        return;
    }

    println!("which manual page do you want?");
}

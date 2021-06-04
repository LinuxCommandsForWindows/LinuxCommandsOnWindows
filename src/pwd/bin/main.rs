use std::env;

fn main() {
    let mut args = env::args();

    if let Some(arg) = args.nth(1) {
        match &*arg {
            "--help" => {
                println!();
                return;
            },
            "--version" => {
                println!();
                return;
            },
            _ => ()
        }
    }

    println!("/mnt/{}", env::current_dir().unwrap().to_str().unwrap().replace(":", "").replace(r"\", "/").to_ascii_lowercase());
}

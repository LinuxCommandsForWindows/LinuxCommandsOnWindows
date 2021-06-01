use std::{
    env,
    fs,
    io::{
        self,
        Read
    },
    path::PathBuf
};

pub fn GetManPagesDirectory() -> PathBuf {
    if let Ok(value) = env::var("MAN_PAGES_DIR") {
        PathBuf::from(value)
    }
    else {
        let path = env::current_dir().unwrap();
        let chars = path.to_str().unwrap().chars().collect::<Vec<_>>();

        PathBuf::from(format!("{}shares\\man", chars[0..=2].into_iter().collect::<String>()))
    }
}

pub fn GetManPagesFileContent(directory: PathBuf, man_command: &str) -> io::Result<String> {
    let mut content = String::new();

    let mut file = fs::File::open(format!("{}/{}.manfile", directory.to_str().unwrap(), man_command))?;
    file.read_to_string(&mut content)?;

    Ok(content)
}

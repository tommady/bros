//use std::error::Error;
use dirs::home_dir;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bros-cli")]
enum Opt {
    #[structopt(name = "mybro")]
    MyBro {
        #[structopt(parse(from_str))]
        name: String,
        #[structopt(long = "zshrc-path")]
        zshrc_path: Option<String>,
    },
}

fn my_bro(name: String, zshrc_path: Option<String>) {
    let dir = String::from(home_dir().unwrap().to_str().unwrap());
    let default_zshrc_path = format!("{}/.zshrc", dir);
    let zshrc = match zshrc_path {
        Some(ref path) => Path::new(path),
        None => Path::new(&default_zshrc_path),
    };

    let mut file = OpenOptions::new().append(true).open(zshrc).unwrap();
    let mut data = format!("export BRO={}\n", name);
    file.write(data.as_bytes())
        .expect("write file export failed");
    data = "typeset -g POWERLEVEL9K_DIR_CLASSES=('*' DEFAULT ' ❤ '${BRO}' ❤ ')\n".to_string();
    file.write(data.as_bytes())
        .expect("write file POWERLEVEL failed");
    file.flush().expect("file flush failed");
}

fn main() {
    match Opt::from_args() {
        Opt::MyBro { name, zshrc_path } => my_bro(name, zshrc_path),
    }
}

use std::env::var;
use std::error::Error;
use std::path::Path;
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bros-cli")]
enum Opt {
    #[structopt(name = "mybro")]
    MyBro {
        #[structopt(parse(from_str))]
        name: String,
        #[structopt(long = "p10k-path")]
        p10k_path: Option<String>,
    },
}

fn my_bro(name: String, p10k_path: Option<String>) {
    let p10k_home = format!("{}/.p10k.zsh", var("HOME").unwrap());
    let p10k = match p10k_path {
        Some(path) => Path::new(&path),
        None => Path::new(&p10k_home),
    }

    let file = match OpenOptions::new().append(true).open(p10k) {
        Err(e) => println!("open file {} failed:{}", p10k.display(), e),
        Ok(_v) => {},
    };
}

fn main() {
    match Opt::from_args() {
        Opt::MyBro {
            name,
            p10k_path,
        } =>  my_bro(name, p10k_path),
    }
}

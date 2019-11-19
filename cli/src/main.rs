use std::env::var;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bros-cli")]
enum Opt {
    #[structopt(name = "mybro")]
    MyBro {
        #[structopt(parse(from_str))]
        name: String,
        #[structopt(long = "p10k-path", default_value = "")]
        p10k_path: String,
        #[structopt(long = "bro-config-path", default_value = "")]
        bro_config_path: String,
    },
}

fn my_bro(name: String, p10k_path: String, bro_config_path: String) {
    println!("{}", name);

    let p10k_home = format!("{}/.p10k.zsh", var("HOME").unwrap());
    let mut p10k = Path::new(&p10k_home);
    if !p10k_path.is_empty() {
        p10k = Path::new(&p10k_path);
    }

    let bro_config_home = format!("{}/.bro.toml", var("HOME").unwrap());
    let mut bro_conf = Path::new(&bro_config_home);
    if !bro_config_path.is_empty() {
        bro_conf = Path::new(&bro_config_path);
    }
}

fn main() {
    match Opt::from_args() {
        Opt::MyBro {
            name,
            p10k_path,
            bro_config_path,
        } => my_bro(name, p10k_path, bro_config_path),
    }
}

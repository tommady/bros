//use std::error::Error;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use structopt::StructOpt;
use dirs::home_dir;

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
    println!("{}", name);
    let dir = String::from(home_dir().unwrap().to_str().unwrap());
    let p10k_home = format!("{}/.p10k.zsh", dir);
    let p10k = match p10k_path {
        Some(ref path) => Path::new(path),
        None => Path::new(&p10k_home),
    };

    let mut file = OpenOptions::new().append(true).open(p10k).unwrap();
    // let mut w = BufWriter::new(file);
    let data = "typeset -g POWERLEVEL9K_DIR_CLASSES=('*' DEFAULT ' ❤'${{BRO}}'❤ ')";
    file.write_all(data.as_bytes()).expect("cannot write file")
    // w.write_all(data.as_bytes()).expect("unable to write");
    // w.flush().expect("unable to flush");

    // if let Err(e) = writeln!(file, "typeset -g POWERLEVEL9K_DIR_CLASSES=('*' DEFAULT ' ❤'${{BRO}}'❤ ')") {
    //     eprintln!("cant write to file: {}", e);
    // }
}

fn main() {
    match Opt::from_args() {
        Opt::MyBro {
            name,
            p10k_path,
        } =>  my_bro(name, p10k_path),
    }
}

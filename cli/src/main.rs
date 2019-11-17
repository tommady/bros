use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bros-cli")]
enum Opt {
    #[structopt(name = "mybro")]
    MyBro {
        #[structopt(parse(from_str))]
        name: String,
        #[structopt(long = "p10k-path", default_value = "~/.p10k.zsh")]
        p10k_path: String,
        #[structopt(long = "config-path", default_value = "~/.bros.toml")]
        config_path: String,
    },
}

fn main() {
    let matches = Opt::from_args();
    println!("{:?}", matches);
}

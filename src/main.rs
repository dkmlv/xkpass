use clap::Parser;
use xkpass::Args;

fn main() {
    let args = Args::parse();
    let xkcd_password = xkpass::generate_password(args);
    println!("{}", xkcd_password);
}

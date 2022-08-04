use clap::Parser;
use xkcdpass::Args;

fn main() {
    let args = Args::parse();
    let xkcd_password = xkcdpass::generate_password(args);
    println!("{}", xkcd_password);
}

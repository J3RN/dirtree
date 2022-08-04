use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}

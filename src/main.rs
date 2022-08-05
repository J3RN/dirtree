use clap::Parser;
use std::io;
use std::path::Path;
use serde::Serialize;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    #[clap(value_parser)]
    directory: String,
}

// #[derive(Debug)]
#[derive(Debug, Serialize)]
struct Node {
    name: String,
    children: Vec<Node>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("{:?}", serde_json::to_string(&process_dir(&Path::new(&args.directory)).unwrap()));

    Ok(())
}

fn process_dir(dirname: &Path) -> io::Result<Node> {
    let children = if dirname.is_dir() {
        dirname
            .read_dir()?
            .flatten()
            .map(|file| process_dir(&file.path()))
            .flatten()
            .collect()
    } else {
        vec![]
    };

    Ok(Node {
        name: dirname.file_name().unwrap().to_str().unwrap().to_string(),
        children,
    })
}

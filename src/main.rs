use std::io;
use std::path::Path;

use askama::Template;
use clap::Parser;
use serde::Serialize;

#[derive(Template)]
#[template(path = "tree.html", escape = "none")]
struct TreeTemplate {
    tree_data: String,
}

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    #[clap(value_parser)]
    directory: String,

    /// File to write generated output to (defaults to stdout)
    #[clap(short, value_parser)]
    output: Option<String>,

    /// Do not ignore entries that begin with '.'
    #[clap(short, long, value_parser)]
    all: bool
}

// #[derive(Debug)]
#[derive(Debug, Serialize)]
struct Node {
    name: String,
    children: Vec<Node>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let root_path = Path::new(&args.directory);
    let entries = enumerate_dir(&root_path, args.all).unwrap();
    let tree_data = serde_json::to_string(&entries).unwrap();
    let output = generate_output(tree_data);

    match args.output {
        Some(filename) => std::fs::write(&filename, output).expect(&format!("Failed to write output to {}", filename)),
        None => println!("{}", output),
    }

    Ok(())
}

fn enumerate_dir(dir: &Path, include_hidden: bool) -> io::Result<Node> {
    let children = if dir.is_dir() {
        dir.read_dir()?
            .flatten()
            .filter(|f| include_hidden || !f.file_name().into_string().unwrap().starts_with("."))
            .map(|file| enumerate_dir(&file.path(), include_hidden))
            .flatten()
            .collect()
    } else {
        vec![]
    };

    let name = match dir.file_name() {
        Some(name) => name.to_str().unwrap().to_string(),
        None => String::from("."),
    };

    Ok(Node { name, children })
}

fn generate_output(tree_data: String) -> String {
    let template = TreeTemplate { tree_data };
    template.render().unwrap()
}

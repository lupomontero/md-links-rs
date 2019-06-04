extern crate regex;
extern crate rustc_serialize;
extern crate structopt;


use std::fs;
use std::path::PathBuf;
use regex::Regex;
use rustc_serialize::json;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "md-links", about = "Check links in MarkDown files.")]
struct Opt {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Validate option
    #[structopt(short = "v", long = "validate")]
    validate: bool,
    /// Show stats instead of individual matches
    #[structopt(short = "s", long = "stats")]
    stats: bool,
    /// Show output in JSON format
    #[structopt(short = "j", long = "json")]
    json: bool,
}


#[derive(Debug, RustcEncodable)]
struct Link {
    url: String,
    text: String,
    file: String,
    line: u32,
}


fn get_links_from_file(path: &PathBuf) -> Vec<Link> {
    let mut links: Vec<Link> = vec![];
    let content = fs::read_to_string(path)
        .expect("could not read file");

    let mut line_number: u32 = 1;

    for line in content.lines() {
        let re = Regex::new(r"\[(.*)\]\((.*)\)").unwrap();
        for cap in re.captures_iter(line) {
            links.push(Link {
                url: cap[2].to_string(),
                text: cap[1].to_string(),
                file: path.to_str().unwrap().to_string(),
                line: line_number,
            });
        }
        line_number += 1;
    }

    links
}


fn get_links_from_dir(path: &PathBuf) -> Vec<Link> {
    let mut links: Vec<Link> = vec![];
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let child_path = entry.unwrap().path();
        let child_links = if child_path.is_dir() {
            get_links_from_dir(&child_path)
        } else if child_path.to_str().unwrap().ends_with("md") {
            get_links_from_file(&child_path)
        } else {
            vec![]
        };
        for child_link in child_links {
            links.push(child_link);
        }
    }

    links
}


fn main() {
    let opt = Opt::from_args();
    let result = fs::metadata(&opt.path);
    let metadata = match result {
        Ok(metadata) => { metadata },
        Err(error) => { panic!("ERROR: {}", error); }
    };

    let links = if metadata.is_dir() {
        get_links_from_dir(&opt.path)
    } else if opt.path.to_str().unwrap().ends_with("md") {
        get_links_from_file(&opt.path)
    } else {
        panic!("ERROR: Unsupported file extension.")
    };

    if opt.json {
        let encoded = json::encode(&links).unwrap();
        println!("{}", encoded);
        return;
    }

    for link in links {
        println!("{}:{} {} {}", link.file, link.line, link.url, link.text);
    }
}

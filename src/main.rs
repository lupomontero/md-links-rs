
extern crate rustc_serialize;
extern crate structopt;


mod links;


use std::path::PathBuf;
use rustc_serialize::json;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name = "md-links", about = "Check links in MarkDown files.")]
struct Opt {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Validate links (senf HTTP requests)
    #[structopt(short = "v", long = "validate")]
    validate: bool,
    /// Show stats instead of individual matches
    #[structopt(short = "s", long = "stats")]
    stats: bool,
    /// Show output in JSON format
    #[structopt(short = "j", long = "json")]
    json: bool,
}


fn main() {
    let opt = Opt::from_args();
    let links = links::from(&opt.path);

    if opt.json {
        let encoded = json::encode(&links).unwrap();
        println!("{}", encoded);
        return;
    }

    for link in links {
        println!("{}:{} {} {}", link.file, link.line, link.url, link.text);
    }
}

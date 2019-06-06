extern crate rustc_serialize;
extern crate structopt;
extern crate md_links;


use std::path::PathBuf;
use rustc_serialize::json;
use structopt::StructOpt;
use md_links::Link;


#[derive(Debug, StructOpt)]
#[structopt(name = "md-links", about = "Check links in MarkDown files.")]
struct Opt {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Validate links (send HTTP requests)
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
struct Stats {
    total: u32,
    uniq: u32,
    broken: Option<u32>,
}


fn print_stats(links: &Vec<Link>, validate: bool, json: bool) {
    let mut urls = vec![];
    let mut broken = 0;

    for link in links {
        urls.push(link.url.to_string());
        let valid = match link.valid {
            None => false,
            Some(x) => x,
        };
        if validate && link.target == "absolute" && !valid {
            broken += 1;
        }
    }

    urls.sort_unstable();
    urls.dedup();

    if json {
        let stats = Stats {
            total: links.len() as u32,
            uniq: urls.len() as u32,
            broken: if validate { Some(broken) } else { None },
        };
        let encoded = json::encode(&stats).unwrap();
        println!("{}", encoded);
        return;
    }

    println!("Total: {:?}", links.len());
    println!("Uniq: {:?}", urls.len());
    if validate {
        println!("Broken: {:?}", broken);
    }
}


fn print_links(links: &Vec<Link>, validate:bool, json: bool) {
    if json {
        let encoded = json::encode(&links).unwrap();
        println!("{}", encoded);
        return;
    }

    for link in links {
        if validate {
            println!("{}:{} {} {} {} {}", link.file, link.line, link.url, link.text, match link.valid {
                None => "",
                Some(x) => if x { "OK" } else { "INVALID" },
            }, match link.status {
                None => 0,
                Some(x) => x,
            });
        } else {
            println!("{}:{} {} {}", link.file, link.line, link.url, link.text);
        }
    }
}


fn main() {
    let opt = Opt::from_args();
    let links = md_links::from_path(&opt.path, opt.validate);

    if opt.stats {
        print_stats(&links, opt.validate, opt.json);
    } else {
        print_links(&links, opt.validate, opt.json);
    }
}

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


fn print_stats(links: &Vec<Link>, validate: bool, json: bool) -> String {
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
        return format!("{}", encoded);
    }

    if validate {
        return format!("Total: {}\nUniq: {}\nBroken: {}", links.len(), urls.len(), broken);
    }

    return format!("Total: {}\nUniq: {}", links.len(), urls.len());
}


fn print_links(links: &Vec<Link>, validate:bool, json: bool) -> String {
    if json {
        let encoded = json::encode(&links).unwrap();
        return format!("{}", encoded);
    }

    let mut out = "".to_string();

    for link in links {
        if out.len() > 0 { out = format!("{}\n", out); }
        if validate {
            out = format!(
                "{}{}:{} {} {} {} {}",
                out,
                link.file,
                link.line,
                link.url,
                link.text,
                match link.valid {
                    None => "",
                    Some(x) => if x { "OK" } else { "INVALID" },
                },
                match link.status {
                    None => 0,
                    Some(x) => x,
                }
            );
        } else {
            out = format!("{}{}:{} {} {}", out, link.file, link.line, link.url, link.text);
        }
    }

    return out;
}


fn main() {
    let opt = Opt::from_args();
    let links = md_links::from_path(&opt.path, opt.validate);
    let out = if opt.stats {
        print_stats(&links, opt.validate, opt.json)
    } else {
        print_links(&links, opt.validate, opt.json)
    };

    println!("{}", out);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_links_should_handle_one_link() {
        let links = vec![Link {
            file: "/README.md".to_string(),
            line: 1,
            url: "http://foo.bar".to_string(),
            text: "Foo".to_string(),
            target: "absolute".to_string(),
            valid: None,
            status: None,
        }];
        let out = print_links(&links, false, false);
        assert_eq!(out, "/README.md:1 http://foo.bar Foo");
    }

    #[test]
    fn print_links_should_handle_many_links() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://foo.bar/baz.html".to_string(),
                text: "Foo bar".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        let out = print_links(&links, false, false);
        assert_eq!(out, "/README.md:1 http://foo.bar Foo\n/README.pt-BR.md:2 http://foo.bar/baz.html Foo bar");
    }

    #[test]
    fn print_links_should_show_validation_stuff_when_appropriate() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: Some(true),
                status: Some(200),
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://foo.bar/baz.html".to_string(),
                text: "Foo bar".to_string(),
                target: "absolute".to_string(),
                valid: Some(false),
                status: Some(404),
            },
        ];
        let out = print_links(&links, true, false);
        assert_eq!(out, "/README.md:1 http://foo.bar Foo OK 200\n/README.pt-BR.md:2 http://foo.bar/baz.html Foo bar INVALID 404");
    }

    #[test]
    fn print_links_should_handle_json_option() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://foo.bar/baz.html".to_string(),
                text: "Foo bar".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        let out = print_links(&links, false, true);
        assert_eq!(out, "[{\"url\":\"http://foo.bar\",\"text\":\"Foo\",\"file\":\"/README.md\",\"line\":1,\"target\":\"absolute\",\"valid\":null,\"status\":null},{\"url\":\"http://foo.bar/baz.html\",\"text\":\"Foo bar\",\"file\":\"/README.pt-BR.md\",\"line\":2,\"target\":\"absolute\",\"valid\":null,\"status\":null}]");
    }

    #[test]
    fn print_stats_should_include_total_and_uniq() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        let out = print_stats(&links, false, false);
        assert_eq!(out, "Total: 2\nUniq: 1");
    }

    #[test]
    fn print_stats_should_include_broken_count_when_validate_option_enabled() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: Some(false),
                status: Some(404),
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://omg.com".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: Some(true),
                status: Some(200),
            },
        ];
        let out = print_stats(&links, true, false);
        assert_eq!(out, "Total: 2\nUniq: 2\nBroken: 1");
    }

    #[test]
    fn print_stats_should_handle_json_option() {
        let links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "http://omg.com".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        let out = print_stats(&links, false, true);
        assert_eq!(out, "{\"total\":2,\"uniq\":2,\"broken\":null}");
    }
}

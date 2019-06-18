extern crate regex;
extern crate reqwest;


use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, RustcEncodable)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub file: String,
    pub line: u32,
    pub target: String, // "absolute", "relative" or "fragment"
    pub valid: Option<bool>,
    pub status: Option<u16>,
}


fn validate_links(links: &mut Vec<Link>) {
    let client = reqwest::Client::new();
    let mut cache = HashMap::new();
    for link in links {
        if link.target != "absolute" {
            continue;
        }

        let key = link.url.to_string();

        if cache.contains_key(&key) {
            let status = *cache.get(&key).unwrap();
            link.valid = if status == 200 { Some(true) } else { Some(false) };
            link.status = Some(status);
            continue;
        }

        // TODO: Handle errors!!!
        let resp = client.get(&link.url).send().unwrap();
        let status = resp.status().as_u16();
        link.valid = if status == 200 { Some(true) } else { Some(false) };
        link.status = Some(status);
        cache.insert(key, status);
    }
}


fn from_file(path: &PathBuf) -> Vec<Link> {
    let mut links: Vec<Link> = vec![];
    let content = fs::read_to_string(path)
        .expect("could not read file");

    let mut line_number: u32 = 1;
    let re = Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();

    for line in content.lines() {
        for cap in re.captures_iter(line) {
            let url = cap[2].to_string();
            let target = if url.starts_with("#") {
                "fragment"
            } else if url.starts_with("http") {
                "absolute"
            } else {
                "relative"
            };
            links.push(Link {
                url: url,
                text: cap[1].to_string(),
                file: path.to_str().unwrap().to_string(),
                line: line_number,
                target: target.to_string(),
                valid: None,
                status: None,
            });
        }
        line_number += 1;
    }

    links
}


fn from_dir(path: &PathBuf) -> Vec<Link> {
    let mut links: Vec<Link> = vec![];
    let entries = fs::read_dir(path).unwrap();

    for entry in entries {
        let child_path = entry.unwrap().path();
        let child_links = if child_path.is_dir() {
            from_dir(&child_path)
        } else if child_path.to_str().unwrap().ends_with("md") {
            from_file(&child_path)
        } else {
            vec![]
        };
        for child_link in child_links {
            links.push(child_link);
        }
    }

    links
}


pub fn from_path(path: &PathBuf, validate: bool) -> Vec<Link> {
    let result = fs::metadata(path);
    let metadata = match result {
        Ok(metadata) => { metadata },
        Err(error) => { panic!("ERROR: {}", error); }
    };

    let mut links = if metadata.is_dir() {
        from_dir(path)
    } else if path.to_str().unwrap().ends_with("md") {
        from_file(path)
    } else {
        panic!("ERROR: Unsupported file extension.")
    };

    if validate {
        validate_links(&mut links);
    }

    links
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn from_path_should_panic_when_bad_path() {
        let p = PathBuf::from("__%%__");
        from_path(&p, false);
    }

    #[test]
    fn from_path_should_get_links_from_file() {
        let p = PathBuf::from("./README.md");
        let links = from_path(&p, false);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links");
        assert_eq!(links[0].text, "Laboratoria\'s bootcamp project `md-links`");
        assert_eq!(links[0].file, "./README.md");
        assert_eq!(links[0].line, 3);
        assert_eq!(links[0].valid, None);
        assert_eq!(links[0].status, None);
    }

    #[test]
    fn from_path_should_get_links_from_dir() {
        let p = PathBuf::from("./");
        let links = from_path(&p, false);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links");
        assert_eq!(links[0].text, "Laboratoria\'s bootcamp project `md-links`");
        assert_eq!(links[0].file, "./README.md");
        assert_eq!(links[0].line, 3);
        assert_eq!(links[0].valid, None);
        assert_eq!(links[0].status, None);
    }

    #[test]
    fn from_path_should_get_links_from_dir_and_validate() {
        let p = PathBuf::from("./");
        let links = from_path(&p, true);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links");
        assert_eq!(links[0].text, "Laboratoria\'s bootcamp project `md-links`");
        assert_eq!(links[0].file, "./README.md");
        assert_eq!(links[0].line, 3);
        assert_eq!(links[0].valid.unwrap(), true);
        assert_eq!(links[0].status.unwrap(), 200);
    }

    #[test]
    fn validate_links_should_handle_http_errors() {
        let mut links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "http://foo.bar".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        validate_links(&mut links);
        println!("{:?}", links);
    }

    #[test]
    fn validate_links_should_use_cache() {
        let mut links = vec![
            Link {
                file: "/README.md".to_string(),
                line: 1,
                url: "https://api.github.com".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
            Link {
                file: "/README.pt-BR.md".to_string(),
                line: 2,
                url: "https://api.github.com".to_string(),
                text: "Foo".to_string(),
                target: "absolute".to_string(),
                valid: None,
                status: None,
            },
        ];
        validate_links(&mut links);
        assert_eq!(links[0].valid, links[1].valid);
        assert_eq!(links[0].status, links[1].status);
        // TODO: Mock HTTP so we can verify only one request was sent???
    }
}

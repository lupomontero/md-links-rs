extern crate regex;


use std::fs;
use std::path::PathBuf;
use regex::Regex;


#[derive(Debug, RustcEncodable)]
pub struct Link {
    pub url: String,
    pub text: String,
    pub file: String,
    pub line: u32,
}


fn from_file(path: &PathBuf) -> Vec<Link> {
    let mut links: Vec<Link> = vec![];
    let content = fs::read_to_string(path)
        .expect("could not read file");

    let mut line_number: u32 = 1;
    let re = Regex::new(r"\[(.+)\]\((.+)\)").unwrap();

    for line in content.lines() {
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


pub fn from_path(path: &PathBuf) -> Vec<Link> {
    let result = fs::metadata(path);
    let metadata = match result {
        Ok(metadata) => { metadata },
        Err(error) => { panic!("ERROR: {}", error); }
    };

    let links = if metadata.is_dir() {
        from_dir(path)
    } else if path.to_str().unwrap().ends_with("md") {
        from_file(path)
    } else {
        panic!("ERROR: Unsupported file extension.")
    };

    links
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn from_should_panic_when_bad_path() {
        let p = PathBuf::from("__%%__");
        from(&p);
    }

    #[test]
    fn from_should_get_links_from_file() {
        let p = PathBuf::from("./README.md");
        let links = from(&p);
        println!("{:?}", links[0]);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links");
        assert_eq!(links[0].text, "Laboratoria\'s bootcamp project `md-links`");
        assert_eq!(links[0].file, "./README.md");
        assert_eq!(links[0].line, 3);
    }

    #[test]
    fn from_should_get_links_from_dir() {
        let p = PathBuf::from("./");
        let links = from(&p);
        println!("{:?}", links[0]);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://github.com/Laboratoria/curricula-js/tree/master/projects/04-md-links");
        assert_eq!(links[0].text, "Laboratoria\'s bootcamp project `md-links`");
        assert_eq!(links[0].file, "./README.md");
        assert_eq!(links[0].line, 3);
    }
}

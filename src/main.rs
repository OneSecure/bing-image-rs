use html_parser::{Dom, Node, Element};
use std::{fs::File, result::Result, io::{self, Read, Write}, path::PathBuf, path::Path, str::Bytes};
use structopt::StructOpt;
use url::Url;

mod app_config;

#[derive(Debug, StructOpt)]
/// A simple and general purpose bing wallpaper extractor.
struct Opt {
    #[structopt(short, long)]
    /// Pretty-print the output.
    pretty_print: bool,

    #[structopt(short, long)]
    /// Debug the parser, this will print errors to the console.
    debug: bool,

    /// Path to config json file, you must assin it.
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<(), u32> {
    let opt = Opt::from_args();

    let mut content = String::with_capacity(100_000);

    if let Some(path) = opt.input {
        // If input is provided then use that as a path
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut content).unwrap();
    } else {
        panic!("Please set json config file");
    }

    let config: app_config::AppConfig = serde_json::from_str(&content).unwrap();

    print!("{:?}\n", config);

    for url in config.bing_urls.iter() {
        do_business(url, &config.twitter_params)?;
    }

    Ok(())
}

fn do_business(url: &String, config: & app_config::TwitterParams) -> Result<(), u32> {
    let baseurl: Url;
    if let Ok(r) = Url::parse(url) {
        baseurl = base_url(r)?;
    } else {
        return Err(u32::MAX);
    }

    let mut content = String::with_capacity(100_000);

    if let Ok(c) = download_web_page_sync(url) {
        content = c;
    }

    let dom = Dom::parse(&content).unwrap();

    for error in &dom.errors {
        println!("# {}", error);
    }

    // println!("{}", dom.to_json_pretty()?);

    if let Some(r) = extract_image_title(&dom) {
        print!("{}\n", r);
    }

    if let Some(r) = extract_image_copyright(&dom) {
        print!("{}\n", r);
    }

    if let Some(r) = extract_image_path(&dom) {
        print!("{}\n", r);

        let joined = baseurl.join(&r).unwrap();

        if let Ok(c) = download_image_sync(joined.as_str()) {
            let path = Path::new("./test.jpg");
            let display = path.display();
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}: {}", display, why),
                Ok(file) => file,
            };
            match file.write_all(&c) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => print!("successfully wrote ot {}\n", display),
            }
        }
    }

    Ok(())
}

fn _extract_image_path(node: &Node) -> Option<String> {
    match node {
        Node::Element(element) => {
            let attrs = &element.attributes;

            if let Some(download) = attrs.get("download") {
                if download.as_deref() == Some("BingWallpaper.jpg") {
                    return attrs["href"].clone();
                }
            }
            for iter in element.children.iter() {
                if let Some(r) = _extract_image_path(iter) {
                    return Some(r);
                }
            }
            return None;
        },
        _ => return None,
    }
}

fn extract_image_path(dom: &Dom) -> Option<String> {
    assert!(dom.children.len() == 1);
    for iter in dom.children.iter() {
        if let Some(r) = _extract_image_path(iter) {
            return Some(r);
        }
    }
    return None;
}

fn _extract_image_title(node: &Node, info: &str) -> Option<String> {
    match node {
        Node::Element(element) => {
            for iter0 in element.classes.iter() {
                if iter0 == info {
                    for iter2 in element.children.iter() {
                        if let Node::Text(r) = iter2 {
                            return Some(r.to_string());
                        }
                    }
                }
            }
            for iter in element.children.iter() {
                if let Some(r) = _extract_image_title(iter, info) {
                    return Some(r);
                }
            }
            return None;
        },
        _ => return None,
    }
}

fn extract_image_title(dom: &Dom) -> Option<String> {
    assert!(dom.children.len() == 1);
    for iter in dom.children.iter() {
        if let Some(r) = _extract_image_title(iter, "title") {
            return Some(r);
        }
    }
    return None;
}

fn extract_image_copyright(dom: &Dom) -> Option<String> {
    assert!(dom.children.len() == 1);
    for iter in dom.children.iter() {
        if let Some(r) = _extract_image_title(iter, "copyright") {
            return Some(r);
        }
    }
    return None;
}

fn download_web_page_sync(url: &str) -> reqwest::Result<String> {
    return reqwest::blocking::get(url)?.text();
}

fn download_image_sync(url: &str) -> reqwest::Result<bytes::Bytes> {
    return reqwest::blocking::get(url)?.bytes();
}

fn base_url(mut url: Url) -> Result<Url, u32> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(u32::MAX);
        }
    }

    url.set_query(None);

    Ok(url)
}

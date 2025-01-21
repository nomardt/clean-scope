use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

const CITY_LIST: &str = include_str!("../data/city_list.txt");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let domains_path = &args[1];

    let city_list: HashSet<String> = CITY_LIST.lines().map(String::from).collect();

    if let Ok(lines) = read_lines(domains_path) {
        for line in lines {
            if let Ok(domain) = line {
                if let Some(subdomain) = get_subdomain(domain.as_str()) {
                    if !city_list.contains(subdomain) {
                        println!("{}", domain);
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_subdomain(domain: &str) -> Option<&str> {
    let mut parts = domain.split('.');

    if parts.clone().count() > 2 {
        parts.next()
    } else {
        None
    }
}

use package_version::{Source, Sources};
use serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
struct Config {
	sources: Vec<Sources>
}

fn main() {
	let config = read_to_string("config.toml").unwrap();
	let config: Config = basic_toml::from_str(&config).unwrap();
	for entry in config.sources {
		println!("########## {:#?} ##########", entry.name());
		println!("{:#?}", entry.get_tags().unwrap());
	}
}

/*
 * @Descripttion: 
 * @version: 1.0.0
 * @Author: CYKS
 * @Date: 2021-01-09 20:11:46
 * @LastEditors: CYKS
 * @LastEditTime: 2021-01-21 21:26:40
 */

extern crate clap;
use kvs::{KvStore, CommandResult, CommandError};
use std::path::PathBuf;
use clap::{App, load_yaml};

fn main() -> CommandResult<()> {
	let yaml = load_yaml!("cli.yml");
	let args = App::from_yaml(yaml);
	let matches = args.get_matches();
	let mut kvs = KvStore::open(PathBuf::from(r"J:\rust\block1\kvs\target\data")).unwrap();
	if matches.is_present("version") {
		let version = env!("CARGO_PKG_VERSION");
		println!("{}", version);
	} else if let Some(matches) = matches.subcommand_matches("set") {
		if !matches.is_present("KEY") || !matches.is_present("VALUE") {
			return Err(CommandError::ArgumenInvalidErorr(2));
		}
		let key = matches.value_of("KEY").unwrap();
		let value = matches.value_of("VALUE").unwrap();
		kvs.set(key.to_string(), value.to_string()).unwrap();
	} else if let Some(ref matches) = matches.subcommand_matches("get") {
		if !matches.is_present("KEY") {
			return Err(CommandError::ArgumenInvalidErorr(1));
		} else {
			let key = matches.value_of("KEY").unwrap();
			let value = kvs.get(key.to_string()).unwrap();
			match value {
				Some(value) => {
					println!("{}", value);
				},
				None => {
					println!("None");
				}
			};
		}
	} else if let Some(ref matches) = matches.subcommand_matches("rm") {
		if !matches.is_present("KEY") {
			return Err(CommandError::ArgumenInvalidErorr(1));
		} else {
			let key = matches.value_of("KEY").unwrap();
			kvs.remove(key.to_string()).unwrap();
		}
	} else {
		return Err(CommandError::CommandInvalidError("".to_string()));
	}
	Ok(())
}
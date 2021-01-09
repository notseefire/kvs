/*
 * @Descripttion: 
 * @version: 1.0.0
 * @Author: CYKS
 * @Date: 2021-01-09 20:11:46
 * @LastEditors: CYKS
 * @LastEditTime: 2021-01-09 22:45:51
 */

extern crate clap;
use core::panic;

use clap::{App, load_yaml};

fn main() {
	let yaml = load_yaml!("cli.yml");
	let args = App::from_yaml(yaml);
	let matches = args.get_matches();
	if matches.is_present("version") {
		let version = env!("CARGO_PKG_VERSION");
		println!("{}", version);
	} else if let Some(matches) = matches.subcommand_matches("set") {
		if !matches.is_present("KEY") || !matches.is_present("VALUE") {
			panic!("No Arguments Or Arguments required is NULL");
		}
		panic!("unimplemented");
	} else if let Some(ref matches) = matches.subcommand_matches("get") {
		if !matches.is_present("KEY") {
			panic!("No Arguments Or Arguments required is NULL");
		} else {
			let key = matches.value_of("KEY").unwrap();
			println!("{:?}", key);
		}
	} else if let Some(ref matches) = matches.subcommand_matches("rm") {
		if !matches.is_present("KEY") {
			panic!("No Arguments Or Arguments required is NULL");
		} else {
			let key = matches.value_of("KEY").unwrap();
			println!("{:?}", key);
		}
	} else {
		panic!("No Arguments");
	}
}
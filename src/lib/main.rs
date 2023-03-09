extern crate clap;
extern crate crossbeam;
extern crate prql_compiler;
extern crate rayon;

use clap::{Arg, ArgAction, Command as ClapCommand};
use prql_compiler::{compile, sql::Dialect, Options, Target};
use rayon::prelude::*;

pub trait PrqlSchema {
	fn schema() -> Vec<(&'static str, &'static str)>;
}

fn parse_query(query: &str) -> (&str, String) {
	let query_str = query.trim();
	let query = compile(
		&query_str,
		Options {
			format: false,
			target: Target::Sql(Some(Dialect::SQLite)),
			signature_comment: false,
		},
	)
	.unwrap();

	(query_str, query)
}

pub fn run() {
	let matches = ClapCommand::new("Innkeeper")
		.version("0.0.1")
		.about("Runs a command in all directories having a certain pattern.")
		.arg(
			Arg::new("parallel")
				.short('p')
				.long("parallel")
				.action(ArgAction::SetTrue)
				.display_order(1)
				.value_name("PARALLEL")
				.required(false)
				.help("Execute code in parallel."),
		)
		.arg(
			Arg::new("file")
				.short('f')
				.display_order(2)
				.value_name("FILE")
				.required(true)
				.help("File to run."),
		)
		.get_matches();

	let parallel = matches.get_flag("parallel");
	let file = matches.get_one::<String>("file").unwrap();

	let prql = std::fs::read_to_string(file).unwrap();

	let queries: Vec<(&str, String)> =
		prql.split(';').filter(|query| !query.trim().is_empty()).map(parse_query).collect();

	for (sql, query) in queries {
		println!("{}", query);
		println!("{}", sql);
	}
}

use vodozemac::olm::{Account, OlmMessage, SessionConfig};
use redb::ReadableTable;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
	about = "Worker prototype implementation",
	version,
	author
)]
pub struct Args {
	#[arg(
		default_value = ".",
		long = "work-path",
		help = "Work path"
	)]
	work_path: String,
}

const TABLE_SECRETS: redb::TableDefinition<&str, &str> = redb::TableDefinition::new("secrets");

pub fn run() -> anyhow::Result<()> {
	let args = Args::parse();

	let work_path = parse_work_path(&args.work_path)?;
	println!("Work path: {}", work_path.to_str().unwrap());

	Ok(())
}

fn init_db(work_path: &std::path::PathBuf) -> anyhow::Result<redb::Database> {
	let db_path = work_path.join("state.redb");
	Ok(redb::Database::create(db_path)?)
}

fn parse_work_path(raw_path: &String) -> anyhow::Result<std::path::PathBuf> {
	let work_path = std::path::PathBuf::from(raw_path);
	std::fs::canonicalize(&work_path).map_err(|e| e.into())
}

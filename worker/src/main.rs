mod tracing;
mod service;
mod cli;

mod command;

fn main() -> cli::Result<()> {
	command::run()
}

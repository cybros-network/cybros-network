mod tracing;
mod service;
mod cli;

mod command;
mod chain;

fn main() -> cli::Result<()> {
	command::run()
}

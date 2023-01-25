mod framework;

mod services;
mod command;
mod chain;

fn main() -> framework::cli::Result<()> {
	command::run()
}

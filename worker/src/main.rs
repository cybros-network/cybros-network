mod framework;

mod command;
mod chain;

fn main() -> framework::cli::Result<()> {
	command::run()
}

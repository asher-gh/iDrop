use drop_gui::app;
use std::process;

fn main() {
	if let Err(e) = app::App::launch() {
		eprintln!("{e}");
		process::exit(1);
	};
}

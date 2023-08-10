use drop_gui::app;
use std::process;

// Following examples on
// https://github.com/iced-rs/iced/tree/master/examples

fn main() {
	if let Err(e) = app::App::launch() {
		eprintln!("{e}");
		process::exit(1);
	};
}

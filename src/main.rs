pub mod styling;

use drop_gui::App;
use iced::pure::Sandbox;
use iced::Settings;
use std::process;

fn main() {
	if let Err(e) = App::run(Settings {
		default_font: Some(include_bytes!(
			"../assets/fonts/Poppins/Poppins-Regular.ttf"
		)),
		antialiasing: true,
		default_text_size: 24,
		..Settings::default()
	}) {
		eprintln!("{e}");
		process::exit(1);
	};
}

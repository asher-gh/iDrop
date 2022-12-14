use iced::{
	pure::{
		column, horizontal_rule, row, text_input,
		widget::{Column, Row, Text},
	},
	Color, Length, Space,
};

use crate::{
	ml::create_model,
	styling::{btn, drop_down, tglr, Theme, BOLD},
	views::SceneMessage,
};

use native_dialog::FileDialog;
use std::{error::Error, path::PathBuf, thread};

// -------------------------------------------------- TRAINING UI
pub struct TrainingUI {
	pub selected_model: Option<UserModel>,
	pub model_name: String,
	pub _new_model: bool,
	pub data_path: Option<PathBuf>,
	pub models: Vec<UserModel>,
	pub creation_toggle: bool,
	model_save_path: Option<PathBuf>,
	error: Result<(), Box<dyn Error>>,
}
impl TrainingUI {
	pub fn new() -> Self {
		TrainingUI {
			selected_model: None,
			model_name: String::new(),
			_new_model: false,
			data_path: None,
			models: Vec::new(), // add persistance later
			creation_toggle: true,
			model_save_path: None,
			error: Ok(()),
		}
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::ModelSelected(model) => {
				self.selected_model = Some(model);
			}
			SceneMessage::SelectModelSavePath => {
				self.model_save_path = FileDialog::new().show_open_single_dir().unwrap();
			}

			SceneMessage::InputChanged(value) => self.model_name = value,

			SceneMessage::SelectCSV => {
				self.data_path = FileDialog::new()
					.add_filter("", &["csv"])
					.show_open_single_file()
					.unwrap();
			}

			SceneMessage::CreateToggled(value) => self.creation_toggle = value,

			SceneMessage::GoPressed => {
				if let Some(pathbuf) = &self.data_path {
					let csv_path = pathbuf
						.to_str()
						.expect("CSV path not valid or empty")
						.to_string();
					// let _x = create_model(String::from(path), &self.model_name);
					// let model_name = format!("{}", self.model_name);

					let model_name_path: String = if let Some(save_path) = &self.model_save_path {
						save_path
							.join(&self.model_name)
							.to_str()
							.expect("Model save path faulty")
							.to_string()
					} else {
						String::from(&self.model_name)
					};

					dbg!(&csv_path, &model_name_path);

					let x = thread::spawn(move || {
						create_model(&csv_path, &model_name_path).expect("create model failed");
					});

					if x.join().is_err() {
						self.error = Err(String::from("Model creation failed").into());
					}
				} else {
					println!("path to training data not set");
				};
			}
			_ => {}
		};
	}

	pub fn view(&self) -> Column<SceneMessage> {
		// --------------------COMPONENTS--------------------
		let pick_list = drop_down(
			&self.models,
			self.selected_model.clone(),
			SceneMessage::ModelSelected,
		)
		.style(Theme::Light)
		.placeholder("Pick a model")
		.padding(10);

		let toggle_create: Row<SceneMessage> = row().push(
			tglr(
				"New model",
				self.creation_toggle,
				SceneMessage::CreateToggled,
			)
			.width(Length::Shrink),
		);

		let text_input = text_input(
			"Provide name for new Model",
			&self.model_name,
			SceneMessage::InputChanged,
		)
		.padding(10)
		.width(Length::Units(250));

		let mut file_selection: Row<SceneMessage> = row()
			.push(btn("Load data", SceneMessage::SelectCSV))
			.push(Space::with_width(Length::Fill));

		let mut save_path: Row<SceneMessage> = row()
			.push(btn(
				"Model save location",
				SceneMessage::SelectModelSavePath,
			))
			.push(Space::with_width(Length::Fill));

		// --------------------ASSEMBLING--------------------

		let mut controls = row()
			.push(toggle_create)
			.push(Space::with_width(Length::Fill));

		if self.creation_toggle {
			controls = controls.push(text_input);
		} else {
			controls = controls.push(pick_list);
		}

		if let Some(file_path) = &self.data_path {
			let file_name = file_path.file_name().unwrap().to_str().unwrap();
			file_selection = file_selection.push(Text::new("CSV file: "));
			file_selection = file_selection.push(Text::new(file_name).font(BOLD));
		}

		if let Some(path) = &self.model_save_path {
			let path = path.to_str().unwrap();
			save_path = save_path.push(Text::new(format!("Saving to {}", path)));
		}

		let create_model_btn = btn("Create model", SceneMessage::GoPressed);

		let mut view = column()
			.height(Length::Fill)
			.spacing(25)
			.push(controls)
			.push(file_selection)
			.push(save_path)
			.spacing(20)
			.push(horizontal_rule(10));

		if *&self.data_path.is_some() && !*&self.model_name.is_empty() {
			view = view.push(create_model_btn);
		}

		if let Err(e) = &self.error {
			view = view.push(Text::new(e.to_string()).color(Color::from_rgb(255., 0., 0.)));
		};

		view.height(Length::Shrink)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserModel {
	pub name: String,
	pub path: Option<String>,
}

impl std::fmt::Display for UserModel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

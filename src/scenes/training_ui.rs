use std::path::PathBuf;

use iced::{
	pure::{
		button, horizontal_rule, pick_list, row, text_input, toggler,
		widget::{Column, PickList, Row, Text},
	},
	ContentFit, Length, Space,
};
use native_dialog::FileDialog;

const MESSAGE: &str = "Welcome to the model creation and training facility. Please select one of the previously created models. If you want to create a new model, check the toggle and provide a name for the model. The model will be created and trained with the provided data (CSV) and saved in the database. You can then select the model in the prediction screen and it will be available to retrain with new data next time.";

use crate::logo;

use super::SceneMessage;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserModel {
	pub name: String,
	pub path: Option<String>,
}

impl std::fmt::Display for UserModel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

pub struct TrainingUI {
	pub selected_model: Option<UserModel>,
	pub model_name: String,
	pub new_model: bool,
	pub data_path: Option<PathBuf>,
	pub models: Vec<UserModel>,
	pub creation_toggle: bool,
}

impl<'a> TrainingUI {
	pub fn new() -> Self {
		TrainingUI {
			selected_model: None,
			model_name: String::new(),
			new_model: false,
			data_path: None,
			models: vec![], // add persistance later
			creation_toggle: false,
		}
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::ModelSelected(model) => {
				self.selected_model = Some(model);
			}

			SceneMessage::InputChanged(value) => self.model_name = value,

			SceneMessage::GoPressed => {}

			SceneMessage::SelectCSV => {
				// let path = FileDialog::new()
				// 	.set_location("~/Desktop")
				// 	.add_filter("PNG Image", &["png"])
				// 	.add_filter("JPEG Image", &["jpg", "jpeg"])
				// 	.show_open_single_file()
				// 	.unwrap();
				//
				// if let Some(path) = path_buf {
				// 	println!("{:?}", path);
				//                 self.data_path = Some(path.to_str);
				// }
				self.data_path = FileDialog::new()
					.show_open_single_file()
					.unwrap()
					.to_owned();
			}

			SceneMessage::CreateToggled(value) => self.creation_toggle = value,

			_ => {}
		}
	}

	pub fn view(&'a self) -> Column<'a, SceneMessage> {
		// --------------------COMPONENTS--------------------
		let pick_list: PickList<UserModel, SceneMessage> = pick_list(
			&self.models,
			self.selected_model.clone(),
			SceneMessage::ModelSelected,
		)
		.placeholder("Pick a model")
		.padding(10);

		let toggle_create: Row<SceneMessage> = row()
			.push(toggler(
				"Create new model".to_owned(),
				self.creation_toggle,
				SceneMessage::CreateToggled,
			))
			.width(Length::Units(200));

		let text_input = text_input(
			"Provide name for new Model",
			&self.model_name,
			SceneMessage::InputChanged,
		)
		.padding(10)
		.width(Length::Units(250));

		// let select_csv_btn: Button<SceneMessage> = Button::new("Select CSV file");

		// --------------------ASSEMBLING--------------------

		let mut content = Column::new().height(Length::Fill);

		let mut controls = row()
			.push(toggle_create)
			.push(Space::with_width(Length::Fill));
		// .push(Space::new(Length::Fill, Length::Fill));

		if self.creation_toggle {
			controls = controls.push(text_input);
		} else {
			controls = controls.push(pick_list);
		}

		content = content
			.push(
				row()
					.push(logo(40, ContentFit::Contain))
					.push(Space::with_width(Length::Fill)),
			)
			.push(Text::new(MESSAGE))
			.spacing(25)
			.push(controls)
			.spacing(20)
			.push(button("CSV").on_press(SceneMessage::SelectCSV))
			.push(horizontal_rule(10));

		content
	}
}

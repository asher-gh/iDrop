mod prediction_ui;
mod training_ui;

use iced::{
	pure::{column, text, widget::Column, Element},
	Length,
};

use drop_gui::prediction::Device;
use prediction_ui::PredictionUI;
use training_ui::{TrainingUI, UserModel};

#[derive(Clone, Debug)]
pub enum SceneMessage {
	DeviceSelected(Device),
	ModelSelected(UserModel),
	SliderChanged(f32),
	InputChanged(String),
	DebugToggled(bool),
	CreateToggled(bool),
	GoPressed,
	SelectCSV,
}

pub enum Scene {
	Greeter,
	Prediction(PredictionUI),
	Training(TrainingUI),
}

impl<'a> Scene {
	pub fn all_scenes() -> Vec<Scene> {
		vec![
			Scene::Greeter,
			Scene::Prediction(PredictionUI::new()),
			Scene::Training(TrainingUI::new()),
		]
	}

	pub fn update(&mut self, msg: SceneMessage, _debug: &mut bool) {
		match self {
			Scene::Greeter => {}
			Scene::Prediction(ui) => ui.update(msg),
			Scene::Training(ui) => ui.update(msg),
		}
	}

	pub fn title(&self) -> &str {
		match self {
			Scene::Greeter => "Welcome",
			Scene::Prediction { .. } => "Prediction",
			Scene::Training(_) => "Model creation and Training",
			// Scene::End => "End",
		}
	}

	fn _can_transition(&self) -> bool {
		todo!("plan when to block transitions")
	}

	pub fn view(&self, _debug: bool) -> Element<SceneMessage> {
		match self {
			Scene::Greeter => Self::welcome(),
			Scene::Prediction(prediction_ui) => {
				Self::container(self.title()).push(prediction_ui.view())
			}
			Scene::Training(ui) => {
				Self::container(self.title()).push(ui.view())
            }
			// Scene::Training => Self::training(),
			// Scene::End => Self::end(),
		}
		.into()
	}

	fn welcome() -> Column<'a, SceneMessage> {
		Self::container("Welcome!")
			.push(
				"Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore
            culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat
            excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate
            voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit
            irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem
            duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi
            laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non
            excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea
            consectetur et est culpa et culpa duis.",
			)
			.push("Created by Ashish")
	}

	fn container(title: &str) -> Column<'a, SceneMessage> {
		column()
			.push(text(title).size(50))
			.spacing(20)
			.height(Length::Units(600))
	}
}

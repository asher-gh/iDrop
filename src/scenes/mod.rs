mod prediction_ui;

use iced::pure::{column, text, widget::Column, Element};

use prediction::Device;
use prediction_ui::PredictionUI;

#[derive(Clone, Debug)]
pub enum SceneMessage {
	DeviceSelected(Device),
	SliderChanged(f32),
	InputChanged(String),
	DebugToggled(bool),
	GoPressed,
}

pub enum Scene {
	Greeter,
	Prediction(PredictionUI),
}

impl<'a> Scene {
	pub fn all_scenes() -> Vec<Scene> {
		vec![Scene::Greeter, Scene::Prediction(PredictionUI::new())]
	}

	pub fn update(&mut self, msg: SceneMessage, _debug: &mut bool) {
		match self {
			Scene::Greeter => {}
			Scene::Prediction(prediction_ui) => prediction_ui.update(msg),
		}
	}

	pub fn title(&self) -> &str {
		match self {
			Scene::Greeter => "Welcome",
			Scene::Prediction { .. } => "Prediction",
			// Scene::Training => "Training",
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
                Self::container(self.title()).push(prediction_ui.view())}
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
		column().spacing(20).push(text(title).size(50))
	}
}

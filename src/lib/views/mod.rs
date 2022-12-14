#![allow(unreachable_patterns)]
pub mod prediction_ui;
pub mod training_ui;

// use crate::views::{
// 	prediction_ui::PredictionUI,
// 	training_ui::{TrainingUI, UserModel},
// };
use iced::{
	pure::{column, text, widget::Column, Element},
	Length,
};

use prediction_ui::{Device, PredictionInput, PredictionUI};
use training_ui::{TrainingUI, UserModel};

#[derive(Clone, Debug)]
pub enum Message {
	BackPressed,
	NextPressed,
	SceneMessage(SceneMessage),
}

#[derive(Clone, Debug)]
pub enum SceneMessage {
	GoPressed,
	CreateToggled(bool),
	InputChanged(String),
	SelectCSV,
	SelectModelSavePath,
	ModelSelected(UserModel),
	DeviceSelected(Device),
	PredictionInputChanged(PredictionInput),
	SelectModel,
	UserModelToggled(bool),
}

// To add a view, declare it here and define it in
// a new file. Propogate all the messages through `update`
// method.
pub enum Scene {
	Training(TrainingUI),
	Prediction(PredictionUI),
}

impl Scene {
	pub fn all_scenes() -> Vec<Scene> {
		vec![
			Scene::Prediction(PredictionUI::new()),
			Scene::Training(TrainingUI::new()),
		]
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match self {
			Scene::Prediction(ui) => ui.update(msg),
			Scene::Training(ui) => ui.update(msg),
			_ => {}
		}
	}

	pub fn title(&self) -> &str {
		match self {
			Scene::Prediction { .. } => "Prediction",
			Scene::Training(_) => "Model creation and Training",
		}
	}

	fn _can_transition(&self) -> bool {
		false
	}

	pub fn view(&self) -> Element<SceneMessage> {
		match self {
			Scene::Prediction(ui) => Self::container(self.title()).push(ui.view()),
			Scene::Training(ui) => Self::container(self.title()).push(ui.view()),
		}
		.into()
	}

	fn container(title: &str) -> Column<SceneMessage> {
		column()
			.push(text(title).size(50))
			// .spacing(20)
			.height(Length::Shrink)
	}
}

pub(crate) struct Scenes {
	pub current: usize,
	pub list: Vec<Scene>,
}

impl Scenes {
	pub fn new() -> Scenes {
		Scenes {
			list: Scene::all_scenes(),
			current: 0,
		}
	}

	pub fn title(&self) -> &str {
		self.list[self.current].title()
	}

	pub fn update(&mut self, event: SceneMessage) {
		self.list[self.current].update(event)
	}

	pub fn has_previous(&self) -> bool {
		self.current > 0
	}

	pub fn can_continue(&self) -> bool {
		self.current < self.list.len() - 1
	}

	pub fn view(&self) -> Element<SceneMessage> {
		self.list[self.current].view()
	}
}

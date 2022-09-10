mod graphics;
mod scenes;

use iced::pure::widget::{Container, Image};
use iced::pure::{button, column, container, horizontal_space, row, scrollable, Element, Sandbox};
use iced::{ContentFit, Font, Length, Settings};
use scenes::{Scene, SceneMessage};
fn main() -> iced::Result {
	App::run(Settings {
		default_font: Some(include_bytes!("../assets/fonts/Montserrat-Regular.ttf")),
		antialiasing: true,
		..Settings::default()
	})
}

#[derive(Default)]
pub struct App {
	_can_continue: bool,
	debug: bool,
	view: Scenes,
}

#[derive(Clone, Debug)]
pub enum Message {
	BackPressed,
	NextPressed,
	SceneMessage(SceneMessage),
}

impl Sandbox for App {
	type Message = Message;

	fn new() -> Self {
		App {
			_can_continue: false,
			debug: true,
			view: Scenes::new(),
		}
	}

	fn title(&self) -> String {
		format!("{} - iDrop", self.view.title())
	}

	fn update(&mut self, event: Message) {
		match event {
			Message::BackPressed => {
				if self.view.has_previous() {
					self.view.active_scene -= 1;
				}
			}
			Message::NextPressed => {
				if self.view.can_continue() {
					self.view.active_scene += 1;
				}
			}
			Message::SceneMessage(scene_event) => self.view.update(scene_event, &mut self.debug),
		}
	}

	fn view(&self) -> iced::pure::Element<'_, Self::Message> {
		let App { view: scenes, .. } = self;

		let mut controls = row();

		if scenes.has_previous() {
			controls = controls.push(button("Back").on_press(Message::BackPressed));
		}

		controls = controls.push(horizontal_space(Length::Fill));

		if scenes.can_continue() {
			controls = controls.push(button("Next").on_press(Message::NextPressed));
		}

		let content: Element<_> = column()
			.push(scenes.view(self.debug).map(Message::SceneMessage))
			.push(controls)
			.max_width(540)
			.spacing(20)
			.padding(20)
			.into();

		let scrollable = scrollable(
			// container(if self.debug {
			// 	content.explain(Color::BLACK)
			// } else {
			// 	content
			// })
			container(content).width(Length::Fill).center_x(),
		);

		container(scrollable).height(Length::Fill).center_y().into()
	}
}

#[derive(Default)]
struct Scenes {
	scenes: Vec<Scene>,
	active_scene: usize,
}

impl Scenes {
	fn new() -> Scenes {
		Scenes {
			// scenes: vec![Scene::Greeter, Scene::Prediction(PredictionUI::new())],
			scenes: Scene::all_scenes(),
			active_scene: 0,
		}
	}

	fn title(&self) -> &str {
		self.scenes[self.active_scene].title()
	}

	fn update(&mut self, event: SceneMessage, debug: &mut bool) {
		self.scenes[self.active_scene].update(event, debug)
	}

	fn has_previous(&self) -> bool {
		if self.active_scene > 0 {
			true
		} else {
			false
		}
	}

	fn can_continue(&self) -> bool {
		if self.active_scene < self.scenes.len() - 1 {
			true
		} else {
			false
		}
	}

	fn view(&self, debug: bool) -> Element<SceneMessage> {
		self.scenes[self.active_scene].view(debug)
	}
}

fn logo<'a>(height: u16, content_fit: ContentFit) -> Container<'a, SceneMessage> {
	Container::new(
		// This should go away once we unify resource loading on native platforms
		if cfg!(target_arch = "wasm32") {
			Image::new("assets/images/logo.jpg")
		} else {
			Image::new(format!(
				"{}/assets/images/logo.jpg",
				env!("CARGO_MANIFEST_DIR"),
			))
		}
		.height(Length::Units(height))
		.content_fit(content_fit),
	)
	.width(Length::Fill)
}

// Fonts
const ICONS: Font = Font::External {
	name: "Icons",
	bytes: include_bytes!("../assets/fonts/fa-solid-900.otf"),
};

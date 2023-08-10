use iced::{
	pure::{button, column, container, horizontal_space, row, scrollable, Element, Sandbox},
	ContentFit, Length, Settings,
};

use super::{
	styling::{btn, logo, Theme},
	views::{Message, Scenes},
};

pub struct App {
	scenes: Scenes,
	theme: Theme,
}

impl App {
	pub fn launch() -> Result<(), iced::Error> {
		App::run(Settings {
			default_font: Some(include_bytes!(
				"../../assets/fonts/Poppins/Poppins-Regular.ttf"
			)),
			antialiasing: true,
			default_text_size: 24,
			..Settings::default()
		})
	}
}

impl Sandbox for App {
	type Message = Message;

	fn new() -> Self {
		App {
			scenes: Scenes::new(),
			theme: Theme::Light,
		}
	}

	fn title(&self) -> String {
		format!("{} - iDrop", self.scenes.title())
	}

	fn update(&mut self, event: Message) {
		match event {
			Message::BackPressed => {
				if self.scenes.has_previous() {
					self.scenes.current -= 1;
				}
			}
			Message::NextPressed => {
				if self.scenes.can_continue() {
					self.scenes.current += 1;
				}
			}
			Message::SceneMessage(scene_event) => self.scenes.update(scene_event),
		}
	}

	fn view(&self) -> iced::pure::Element<'_, Self::Message> {
		let App { scenes, .. } = self;

		let theme = self.theme;

		let mut controls = row().push(horizontal_space(Length::Fill));

		if scenes.has_previous() {
			controls = controls
				.push(btn("Back", Message::BackPressed))
				.push(horizontal_space(Length::Units(15)));
		}

		if scenes.can_continue() {
			controls = controls.push(button("Next").style(theme).on_press(Message::NextPressed));
		}

		controls = controls
			.push(horizontal_space(Length::Fill))
			.push(logo(40, ContentFit::Contain));
		// .push(horizontal_space(Length::Fill));

		let content: Element<_> = column()
			.push(scenes.view().map(Message::SceneMessage))
			.push(controls)
			.max_width(540)
			.spacing(20)
			.padding(20)
			.into();

		let scrollable = scrollable(container(content).width(Length::Fill).center_x());

		container(scrollable).center_y().into()
	}
}

#![allow(unused)]
use iced::pure::widget::{
	canvas, container, Button, Canvas, Column, Container, Image, Row, Slider, Text, TextInput,
};
use iced::pure::{
	button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
	text_input, Element, Sandbox,
};
use iced::{alignment, window, Color, ContentFit, Font, Length, Settings, Space};
use tf_prediction::predict;
use tf_prediction::Device;

use crate::graphics::Droplet;

mod graphics;

// Fonts
const ICONS: Font = Font::External {
	name: "Icons",
	bytes: include_bytes!("../assets/fonts/fa-solid-900.otf"),
};

fn main() -> iced::Result {
	App::run(Settings {
		default_font: Some(include_bytes!("../assets/fonts/Montserrat-Regular.ttf")),
		antialiasing: true,
		window: window::Settings {
			// size: (640, 480),
			..window::Settings::default()
		},
		..Settings::default()
	})
}

#[derive(Default)]
pub struct App {
	can_continue: bool,
	debug: bool,
	scenes: Scenes,
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
			can_continue: false,
			debug: false,
			scenes: Scenes::new(),
		}
	}

	fn title(&self) -> String {
		format!("{} - iDrop", self.scenes.title())
	}

	fn update(&mut self, event: Message) {
		match event {
			Message::BackPressed => {
				if self.scenes.has_previous() {
					self.scenes.active_scene -= 1;
				}
			}
			Message::NextPressed => {
				if self.scenes.can_continue() {
					self.scenes.active_scene += 1;
				}
			}
			Message::SceneMessage(scene_event) => self.scenes.update(scene_event, &mut self.debug),
		}
	}

	fn view(&self) -> iced::pure::Element<'_, Self::Message> {
		let App { scenes, .. } = self;

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

	fn background_color(&self) -> Color {
		Color::WHITE
	}

	fn scale_factor(&self) -> f64 {
		1.0
	}

	fn should_exit(&self) -> bool {
		false
	}

	fn run(settings: Settings<()>) -> Result<(), iced::Error>
	where
		Self: 'static + Sized,
	{
		<Self as iced::pure::Application>::run(settings)
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
			scenes: vec![
				Scene::Greeter,
				Scene::Prediction {
					selection: None,
					slider_value_f: 0.0,
					text_input_val: "".to_string(),
					can_continue: false,
					computed: false,
					result: (0.0, 0.0, 0.0, 0.0),
				},
			],
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

#[derive(Clone, Debug)]
pub enum SceneMessage {
	DeviceSelected(Device),
	SliderChangedF(f32),
	InputChanged(String),
	DebugToggled(bool),
	GoPressed,
}

enum Scene {
	Greeter,
	Prediction {
		selection: Option<Device>, // keep this and discard the other state (selected_state)
		slider_value_f: f32,
		text_input_val: String,
		can_continue: bool,
		computed: bool,
		result: (f32, f32, f32, f32), // minor,pbs,fluoSurf, freq
	},
	// Training,
	// End,
}

impl<'a> Scene {
	fn update(&mut self, msg: SceneMessage, debug: &mut bool) {
		match msg {
			SceneMessage::DebugToggled(value) => {
				// if let Scene::Training = self {
				// 	*debug = value;
				// }
			}
			SceneMessage::DeviceSelected(device) => {
				if let Scene::Prediction {
					selection,
					can_continue,
					computed,
					..
				} = self
				{
					*selection = Some(device);
					*can_continue = true;
					*computed = false;
				}
			}
			SceneMessage::SliderChangedF(value) => {
				if let Scene::Prediction { slider_value_f, .. } = self {
					*slider_value_f = value;
				}
			}
			SceneMessage::InputChanged(value) => {
				if let Scene::Prediction {
					text_input_val,
					can_continue,
					computed,
					..
				} = self
				{
					*text_input_val = value;
					*can_continue = true;
					*computed = false;
				}
			}
			SceneMessage::GoPressed => {
				if let Scene::Prediction {
					text_input_val,
					selection,
					result,
					computed,
					can_continue,
					..
				} = self
				{
					// let major_axis = text_input_val.parse::<f32>().unwrap();
					let major_axis = 40.0;
					if let Some(channel) = selection {
						// TODO: modularise the prediction for each characteristic
						*result = match predict(&channel, vec![major_axis]) {
							Ok((a, b, c, d)) => (a, b, c, d),
							Err(_) => (0.0, 0.0, 0.0, 0.0),
						};
					}
					*computed = true;
					*can_continue = false;
				}
			}
		}
	}

	fn title(&self) -> &str {
		match self {
			Scene::Greeter => "Welcome",
			Scene::Prediction { .. } => "Prediction",
			// Scene::Training => "Training",
			// Scene::End => "End",
		}
	}

	fn can_transition(&self) -> bool {
		todo!("plan when to block transitions")
	}

	fn view(&self, debug: bool) -> Element<SceneMessage> {
		match self {
			Scene::Greeter => Self::welcome(),
			Scene::Prediction {
				selection,
				// pick_list,
				// selected_device,
				slider_value_f,
				// slider_state_f,
				text_input_val,
				// text_input_state,
				// go_button,
				can_continue,
				computed,
				result,
				// droplet,
			} => self.prediction(
				*selection,
				*slider_value_f,
				text_input_val.to_owned(),
				*can_continue,
				*computed,
				*result,
				debug,
			),
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

	// fn training() -> Column<'a, SceneMessage> {
	// todo!()
	// }

	// fn debugger(debug: bool) -> Column<'a, SceneMessage> {
	// 	todo!()
	// }
	//
	// fn end() -> Column<'a, SceneMessage> {
	// 	todo!()
	// }

	fn prediction(
		&self,
		selection: Option<Device>,
		slider_value_f: f32,
		text_input_val: String,
		can_continue: bool,
		computed: bool,
		result: (f32, f32, f32, f32),
		debug: bool,
	) -> Column<'a, SceneMessage> {
		let pick_list = pick_list(&Device::ALL[..], selection, SceneMessage::DeviceSelected)
			.placeholder("Choose a device...")
			.padding(10);

		// let slider_f = Slider::new(
		// 	&mut self.slider_state_f,
		// 	0.0..=100.0,
		// 	self.slider_value_f,
		// 	Message::SliderChangedF,
		// );

		fn icon(unicode: char) -> Text {
			Text::new(unicode.to_string())
				.font(ICONS)
				.width(Length::Units(20))
				.horizontal_alignment(alignment::Horizontal::Center)
				.size(20)
		}

		let text_input = text_input(
			"Major Dimension",
			&text_input_val,
			SceneMessage::InputChanged,
		)
		.padding(10)
		.width(Length::Units(150));

		// Controls Row
		let mut controls: Row<SceneMessage> = row();

		if can_continue {
			controls =
				// controls.push(button(&mut self.go_button, "Go").on_press(Message::GoPressed));
                controls.push(button("Go").on_press(SceneMessage::GoPressed));
		}

		// let mut result_display = Column::new().padding(0).width(Length::Units(500));
		let mut result_display = column();

		let major_axis = if let Ok(x) = text_input_val.parse::<f32>() {
			x
		} else {
			0.0
		};

		let canvas: Canvas<SceneMessage, Droplet> = Canvas::new(Droplet {
			radii: (major_axis, result.0),
		});

		if let (true, Some(device)) = (computed, selection) {
			// let input: f32 = text_input_val.parse().unwrap();
			let max_val = device.max_value();

			let out_msg: (Text, Text, Text) = if major_axis <= max_val as f32 {
				(
					Text::new(format!(
						"{:.2} µL/min",
						if result.1.is_sign_negative() {
							0.0
						} else {
							result.1
						}
					)),
					Text::new(format!(
						"{:.2} µL/min",
						if result.2.is_sign_negative() {
							0.0
						} else {
							result.2
						}
					)),
					Text::new(format!(
						"{:.2} ",
						if result.3.is_sign_negative() {
							0.0
						} else {
							result.3
						}
					)),
				)
			} else {
				(
					Text::new("INVALID".to_owned()),
					Text::new("INVALID".to_owned()),
					Text::new("INVALID".to_owned()),
				)
			};

			result_display = result_display.push(
				row()
					.push(
						container(canvas.width(Length::Units(200)).height(Length::Units(200)))
							.center_x(), // .padding(10),
					)
					.push(horizontal_space(Length::Fill))
					.push(
						column()
							.max_width(250)
							.push(
								row()
									.push(icon('\u{f043}'))
									.push(Text::new(" PBS"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.0),
							)
							.push(horizontal_rule(5))
							.push(
								row()
									.push(icon('\u{F043}'))
									.push(Text::new(" FluoSurf"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.1),
							)
							.push(horizontal_rule(5))
							.push(
								row()
									.push(icon('\u{f83e}'))
									.push(Text::new(" Frequency"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.2),
							),
					),
			);
		}

		let mut content: Column<SceneMessage> = column();

		content = content
			.push(
				row()
					.push(logo(75, ContentFit::Contain))
					.push(Space::with_width(Length::Fill))
					.push(
						Text::new("Mode 1")
							.size(30)
							.height(Length::Units(45))
							.vertical_alignment(alignment::Vertical::Center),
					),
			)
			.push(Text::new(
				"Please select a channel and enter the major dimension.",
			))
			.spacing(14)
			.push(
				Row::new()
					.push(pick_list)
					.spacing(20)
					.push(
						Row::new().spacing(5).push(text_input).push(
							Text::new("µm")
								.height(Length::Units(40))
								.vertical_alignment(alignment::Vertical::Bottom),
						),
					)
					.push(controls),
			)
			.spacing(20)
			// .push(
			// 	Row::new()
			// 		.push(Text::new("Frequency").width(Length::Units(140)))
			// 		.spacing(10)
			// 		.push(slider_f.width(Length::Units(140)))
			// 		.spacing(10)
			// 		.push(Text::new(format!("{}", self.slider_value_f))),
			// )
			.push(horizontal_rule(10))
			.push(result_display);

		// let content: Element<_> = content.into();

		// let content = if debug {
		// 	content.explain(Color::BLACK)
		//               content.explain
		// } else {
		// 	content
		// };

		// Container::new(content)
		// 	.padding(20)
		// 	.width(Length::Fill)
		// 	.height(Length::Fill)
		// 	.into()

		content
	}

	fn container(title: &str) -> Column<'a, SceneMessage> {
		column().spacing(20).push(text(title).size(50))
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
	// .center_x()
}

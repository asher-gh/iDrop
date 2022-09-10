use iced::pure::widget::{Canvas, Column, Container, Image, Row, Slider, Text};
use iced::pure::{
	button, column, container, horizontal_rule, horizontal_space, pick_list, row, scrollable, text,
	text_input, Element, Sandbox,
};
use iced::{alignment, Color, ContentFit, Font, Length, Settings, Space};
use prediction::{compute, load_model, Device, Model};

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
		..Settings::default()
	})
}

#[derive(Default)]
pub struct App {
	_can_continue: bool,
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
			_can_continue: false,
			debug: true,
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
					models: None,
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
	SliderChanged(f32),
	InputChanged(String),
	DebugToggled(bool),
	GoPressed,
}

enum Scene {
	Greeter,
	Prediction {
		selection: Option<Device>,
		slider_value_f: f32,
		text_input_val: String,
		can_continue: bool,
		computed: bool,
		models: Option<(Model, Model, Model)>, // (sec_dim, flow, freq)
		result: (f32, f32, f32, f32),          // (minor, pbs, fluoSurf, freq)
	},
	// Training,
	// End,
}

impl<'a> Scene {
	fn update(&mut self, msg: SceneMessage, _debug: &mut bool) {
		match msg {
			SceneMessage::DebugToggled(_value) => {
				// if let Scene::Training = self {
				// 	*debug = value;
				// }
			}
			SceneMessage::DeviceSelected(device) => {
				if let Scene::Prediction {
					selection,
					can_continue,
					computed,
					models,
					..
				} = self
				{
					// let models_location = "assets/models/trained_models/";

					// let sec_dim_model_dir = "model_secondnumberdevice2";
					// let flow_model_dir = "newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHOUTFREQ";
					// let freq_model_dir =
					// 	"newermodelrr170510256WITHDROPOUT0.3MOSTFINALWITHFREQPREDICT";

					let model_path = device.model_path();

					let sec_dim_model_dir = model_path.sec_dim;
					let flow_model_dir = model_path.flow;
					let freq_model_dir = model_path.freq;

					let sec_dim_model = load_model(&sec_dim_model_dir);
					let flow_model = load_model(&flow_model_dir);
					let freq_model = load_model(&freq_model_dir);

					*selection = Some(device);
					*can_continue = true;
					*computed = false;
					*models = Some((sec_dim_model, flow_model, freq_model));
				}
			}
			SceneMessage::SliderChanged(value) => {
				if let Scene::Prediction {
					selection,
					slider_value_f,
					text_input_val,
					models,
					result,
					..
				} = self
				{
					*slider_value_f = value;
					*text_input_val = value.to_string();

					if let (Some(models), Some(_channel)) = (models, selection) {
						// TODO: modularise the prediction for each characteristic
						// *result = match predict(&channel, vec![major_axis]) {
						// 	Ok((a, b, c, d)) => (a, b, c, d),
						// 	Err(_) => (0.0, 0.0, 0.0, 0.0),
						// };
						*result = match compute(value, &models.0, &models.1, &models.2) {
							Ok(metrics) => {
								println!("ratio: {}", value / metrics.0);
								metrics
							}
							Err(_) => (0.0, 0.0, 0.0, 0.0),
						}
					}
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
					models,
					..
				} = self
				{
					let major_axis = text_input_val.parse::<f32>().unwrap();

					if let (Some(_channel), Some(models)) = (selection, models) {
						// TODO: modularise the prediction for each characteristic
						// *result = match predict(&channel, vec![major_axis]) {
						// 	Ok((a, b, c, d)) => (a, b, c, d),
						// 	Err(_) => (0.0, 0.0, 0.0, 0.0),
						// };
						*result = match compute(major_axis, &models.0, &models.1, &models.2) {
							Ok(metrics) => metrics,
							Err(_) => (0.0, 0.0, 0.0, 0.0),
						}
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

	fn _can_transition(&self) -> bool {
		todo!("plan when to block transitions")
	}

	fn view(&self, debug: bool) -> Element<SceneMessage> {
		match self {
			Scene::Greeter => Self::welcome(),
			Scene::Prediction {
				selection,
				slider_value_f,
				text_input_val,
				can_continue,
				computed,
				result,
				..
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
		_computed: bool,
		result: (f32, f32, f32, f32),
		_debug: bool,
	) -> Column<'a, SceneMessage> {
		let pick_list = pick_list(&Device::ALL[..], selection, SceneMessage::DeviceSelected)
			.placeholder("Choose a device...")
			.padding(10);

		let slider_f = Slider::new(
			// &mut self.slider_state_f,
			10.0..=250.0,
			slider_value_f,
			SceneMessage::SliderChanged,
		);

		// let slider_f = slider(50..=150, slider_value_f as i32, SceneMessage::SliderChanged(f32 as i32));

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

		if let Some(device) = selection {
			// let input: f32 = text_input_val.parse().unwrap();
			let max_val = device.max_value();

			let out_msg: (Text, Text, Text) = if major_axis <= max_val as f32 {
				let (.., pbs, fluo, freq) = result;

				let pbs = if pbs.is_sign_negative() {
					Text::new("NEG VAL".to_string()).color(Color::from_rgb8(255, 0, 0))
				} else {
					Text::new(format!("{:2}", pbs))
				};

				let fluo = if fluo.is_sign_negative() {
					Text::new("NEG VAL".to_string()).color(Color::from_rgb8(255, 0, 0))
				} else {
					Text::new(format!("{:2}", fluo))
				};

				let freq = if freq.is_sign_negative() {
					Text::new("NEG VAL".to_string()).color(Color::from_rgb8(255, 0, 0))
				} else {
					Text::new(format!("{:2}", freq))
				};

				(pbs, fluo, freq)
			} else {
				(
					Text::new("INVALID INPUT".to_owned()).color(Color::from_rgb8(255, 0, 0)),
					Text::new("INVALID INPUT".to_owned()).color(Color::from_rgb8(255, 0, 0)),
					Text::new("INVALID INPUT".to_owned()).color(Color::from_rgb8(255, 0, 0)),
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
							.max_width(350)
							.push(
								row()
									.push(icon('\u{f043}'))
									.push(Text::new("PBS (µL/min)"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.0),
							)
							.push(horizontal_rule(5))
							.push(
								row()
									.push(icon('\u{F043}'))
									.push(Text::new("FluoSurf (µL/min)"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.1),
							)
							.push(horizontal_rule(5))
							.push(
								row()
									.push(icon('\u{f83e}'))
									.push(Text::new("Frequency (Hz)"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.2),
							),
					),
			);
		}

		let mut content = Column::new();

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
			.push(slider_f)
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
}

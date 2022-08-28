use iced::pure::widget::{Button, Column, Container, Row, Slider, Text, TextInput};
use iced::pure::{button, column, pick_list, row, text, text_input, Element, Sandbox};
use iced::{alignment, window, Canvas, Color, Font, Length, Renderer, Settings, Vector};

use prediction::device::Device;
mod graphics;

fn main() -> iced::Result {
	<Application as Sandbox>::run(Settings {
		default_font: Some(include_bytes!("../assets/fonts/Poppins-Regular.ttf")),
		antialiasing: true,
		window: window::Settings {
			size: (540, 480),
			..window::Settings::default()
		},
		..Settings::default()
	})
}

// moving mode1 -> Scene::Prediction and Appliction as the main loader
#[derive(Default)]
pub struct Application {
	pick_list: pick_list::State<Device>,
	selected_device: Option<Device>,
	slider_value_f: f32,
	slider_state_f: slider::State,
	text_input_val: String,
	text_input_state: text_input::State,
	go_button: button::State,
	can_continue: bool,
	computed: bool,
	result: (f32, f32, f32, f32), // minor,pbs,fluoSurf, freq
	droplet: graphics::Droplet,
	debug: bool,
	scenes: SceneInfo,
}

#[derive(Default)]
struct SceneInfo {
	scenes: Vec<Scene>,
	active_scene: usize,
}

#[derive(Clone)]
enum SceneMessage {
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
		pick_list: pick_list::State<Device>,
		selected_device: Option<Device>,
		slider_value_f: f32,
		slider_state_f: slider::State,
		text_input_val: String,
		text_input_state: text_input::State,
		go_button: button::State,
		can_continue: bool,
		computed: bool,
		result: (f32, f32, f32, f32), // minor,pbs,fluoSurf, freq
		droplet: graphics::Droplet,
	},
	Training,
	End,
}

impl<'a> Scene {
	fn _update(&mut self, msg: SceneMessage, debug: &mut bool) {
		match msg {
			SceneMessage::DebugToggled(value) => {
				if let Scene::Training = self {
					*debug = value;
				}
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
					droplet,
					computed,
					can_continue,
					..
				} = self
				{
					let major_axis = text_input_val.parse::<f32>().unwrap();
					if let Some(channel) = selection {
						*result = match prediction::compute(major_axis, *channel) {
							Ok((a, b, c, d)) => (a, b, c, d),
							Err(_) => (0.0, 0.0, 0.0, 0.0),
						};

						(*droplet).radii = Vector {
							x: major_axis,
							y: result.0,
						};
					}
					*computed = true;
					*can_continue = false;
				}
			}
		}
	}

	fn _title(&self) -> &str {
		match self {
			Scene::Greeter => "Welcome",
			Scene::Prediction { .. } => "Prediction",
			Scene::Training => "Training",
			Scene::End => "End",
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
				pick_list,
				selected_device,
				slider_value_f,
				slider_state_f,
				text_input_val,
				text_input_state,
				go_button,
				can_continue,
				computed,
				result,
				droplet,
			} => Self::prediction(
				*selection,
				// *pick_list,
				*selected_device,
				*slider_value_f,
				*slider_state_f,
				// *text_input_val,
				*text_input_state,
				*go_button,
				// *can_continue,
				// *computed,
				*result,
				*droplet,
			),
			Scene::Training => Self::training(),
			Scene::End => Self::end(),
		}
		.into()
	}

	fn welcome() -> Column<'a, SceneMessage> {
		Self::container("Welcome!").push(
            "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis."
            ).push("Created by Ashish")
	}

	fn training() -> Column<'a, SceneMessage> {
		todo!()
	}

	fn debugger(debug: bool) -> Column<'a, SceneMessage> {
		todo!()
	}

	fn end() -> Column<'a, SceneMessage> {
		todo!()
	}

	fn prediction(
		selection: Option<Device>,
		// pick_list: pick_list::State<Device>,
		selected_device: Option<Device>,
		slider_value_f: f32,
		// slider_state_f: slider::State,
		text_input_val: String,
		// text_input_state: text_input::State,
		// go_button: button::State,
		can_continue: bool,
		computed: bool,
		result: (f32, f32, f32, f32),
		droplet: graphics::Droplet,
	) -> Column<'a, SceneMessage> {
		let pick_list = pick_list(
			&Device::ALL[..],
			selected_device,
			SceneMessage::DeviceSelected,
		)
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
		let mut controls = row();

		if can_continue {
			controls =
				// controls.push(button(&mut self.go_button, "Go").on_press(Message::GoPressed));
                controls.push(iced::pure::button("Go").on_press(SceneMessage::GoPressed));
		}

		let mut result_display = Column::new().padding(0).width(Length::Units(500));

		// graphical element
		let canvas: Canvas<SceneMessage, &mut graphics::Droplet> = Canvas::new(&mut droplet)
			.width(Length::Fill)
			.height(Length::Fill);

		if let (true, Some(device)) = (computed, selected_device) {
			let input: f32 = text_input_val.parse().unwrap();
			let max_val = device as u32;

			let out_msg: (Text, Text, Text) = if input <= max_val as f32 {
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

			// println!(
			// 	"
			// input: {}
			// max: {}
			// device: {}",
			// 	input, max_val, device
			// );

			result_display = result_display.push(
				Row::new()
					.push(Column::new().width(Length::Units(150)).push(canvas))
					.push(
						Column::new()
							.push(
								Row::new()
									.push(icon('\u{f043}'))
									.push(Text::new(" PBS"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.0),
							)
							.push(Rule::horizontal(10))
							.push(
								Row::new()
									.push(icon('\u{F043}'))
									.push(Text::new(" FluoSurf"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.1),
							)
							.push(Rule::horizontal(10))
							.push(
								Row::new()
									.push(icon('\u{f83e}'))
									.push(Text::new(" Frequency"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.2),
							),
					),
			);
		}

		let content: _ = Column::new()
			.push(
				Row::new()
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
			.spacing(20)
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
			.push(
				Row::new()
					.push(Text::new("Frequency").width(Length::Units(140)))
					.spacing(10)
					.push(slider_f.width(Length::Units(140)))
					.spacing(10)
					.push(Text::new(format!("{}", self.slider_value_f))),
			)
			.push(Rule::horizontal(20))
			.push(result_display);

		let content: Element<_> = content.into();

		let content = if *&self.debug {
			content.explain(Color::BLACK)
		} else {
			content
		};

		Container::new(content)
			.padding(20)
			.width(Length::Fill)
			.height(Length::Fill)
			.into()
	}

	fn container(title: &str) -> Column<'a, SceneMessage> {
		column().spacing(20).push(text(title).size(50))
	}
}

// ----------OLD IMPLEMENTATION STARTS HERE----------

// Fonts
const ICONS: Font = Font::External {
	name: "Icons",
	bytes: include_bytes!("../assets/fonts/fa-solid-900.ttf"),
};

#[derive(Debug, Clone)]
pub enum Message {
	DeviceSelected(Device),
	SliderChangedF(f32),
	InputChanged(String),
	DebugToggled(bool),
	GoPressed,
}

impl Sandbox for Application {
	type Message = Message;

	fn new() -> Self {
		Self { ..Self::default() }
	}

	fn title(&self) -> String {
		String::from("Mode 1")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::DeviceSelected(device) => {
				self.selected_device = Some(device);
				self.can_continue = true;
				self.computed = false;
			}
			Message::SliderChangedF(value) => self.slider_value_f = value,
			Message::InputChanged(value) => {
				self.text_input_val = value;
				self.can_continue = true;
				self.computed = false;
			}
			Message::GoPressed => {
				let major_axis = &self.text_input_val.parse::<f32>().unwrap();
				if let Some(channel) = self.selected_device {
					self.result = match prediction::compute(*major_axis, channel) {
						Ok((a, b, c, d)) => {
							println!("minor: {}", a);
							(a, b, c, d)
						}
						Err(_) => (0.0, 0.0, 0.0, 0.0),
					};

					self.droplet.radii = Vector {
						x: *major_axis,
						y: self.result.0,
					};
				}
				self.computed = true;
				self.can_continue = false;
			}
			Message::DebugToggled(_) => todo!(),
		}
	}

	fn view(&mut self) -> Element<Message> {
		let pick_list = PickList::new(
			&mut self.pick_list,
			&Device::ALL[..],
			self.selected_device,
			Message::DeviceSelected,
		)
		.placeholder("Choose a device...")
		.padding(10);

		let slider_f = Slider::new(
			&mut self.slider_state_f,
			0.0..=100.0,
			self.slider_value_f,
			Message::SliderChangedF,
		);

		fn icon(unicode: char) -> Text {
			Text::new(unicode.to_string())
				.font(ICONS)
				.width(Length::Units(20))
				.horizontal_alignment(alignment::Horizontal::Center)
				.size(20)
		}

		let text_input = TextInput::new(
			&mut self.text_input_state,
			"Major Dimension",
			&self.text_input_val,
			Message::InputChanged,
		)
		.padding(10)
		.width(Length::Units(150));

		// Controls Row
		let mut controls = Row::new();

		if self.can_continue {
			controls =
				controls.push(button(&mut self.go_button, "Go").on_press(Message::GoPressed));
		}

		let mut result_display = Column::new().padding(0).width(Length::Units(500));

		// graphical element
		let canvas: Canvas<Message, &mut graphics::Droplet> = Canvas::new(&mut self.droplet)
			.width(Length::Fill)
			.height(Length::Fill);

		if let (true, Some(selected_device)) = (self.computed, self.selected_device) {
			let input: f32 = self.text_input_val.parse().unwrap();
			let max_val = selected_device as u32;

			let out_msg: (Text, Text, Text) = if input <= max_val as f32 {
				(
					Text::new(format!(
						"{:.2} µL/min",
						if self.result.1.is_sign_negative() {
							0.0
						} else {
							self.result.1
						}
					)),
					Text::new(format!(
						"{:.2} µL/min",
						if self.result.2.is_sign_negative() {
							0.0
						} else {
							self.result.2
						}
					)),
					Text::new(format!(
						"{:.2} ",
						if self.result.3.is_sign_negative() {
							0.0
						} else {
							self.result.3
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

			println!(
				"
                     input: {}
                     max: {}
                     device: {}",
				input, max_val, selected_device
			);

			result_display = result_display.push(
				Row::new()
					.push(Column::new().width(Length::Units(150)).push(canvas))
					.push(
						Column::new()
							.push(
								Row::new()
									.push(icon('\u{f043}'))
									.push(Text::new(" PBS"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.0),
							)
							.push(Rule::horizontal(10))
							.push(
								Row::new()
									.push(icon('\u{F043}'))
									.push(Text::new(" FluoSurf"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.1),
							)
							.push(Rule::horizontal(10))
							.push(
								Row::new()
									.push(icon('\u{f83e}'))
									.push(Text::new(" Frequency"))
									.push(Space::with_width(Length::Fill))
									.push(out_msg.2),
							),
					),
			);
		}

		let content: _ = Column::new()
			.push(
				Row::new()
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
			.spacing(20)
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
			.push(
				Row::new()
					.push(Text::new("Frequency").width(Length::Units(140)))
					.spacing(10)
					.push(slider_f.width(Length::Units(140)))
					.spacing(10)
					.push(Text::new(format!("{}", self.slider_value_f))),
			)
			.push(Rule::horizontal(20))
			.push(result_display);

		let content: Element<_> = content.into();

		let content = if *&self.debug {
			content.explain(Color::BLACK)
		} else {
			content
		};

		Container::new(content)
			.padding(20)
			.width(Length::Fill)
			.height(Length::Fill)
			.into()
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
		<Self as iced::Application>::run(settings)
	}
}

// Device to populate the picklist

fn logo<'a>(height: u16, content_fit: ContentFit) -> Container<'a, Message> {
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

fn button<'a, Message: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Message> {
	Button::new(
		state,
		Text::new(label).horizontal_alignment(alignment::Horizontal::Center),
	)
	.padding(12)
	.width(Length::Units(100))

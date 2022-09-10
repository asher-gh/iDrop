use iced::{
	alignment,
	pure::{
		button, column, container, horizontal_rule, horizontal_space, pick_list, row, text_input,
		widget::{Canvas, Column, Row, Slider, Text},
	},
	Color, ContentFit, Length, Space,
};
use prediction::{compute, load_model, Device, Model};

use crate::{graphics::Droplet, logo, ICONS};

use super::SceneMessage;

pub struct PredictionUI {
	pub selection: Option<Device>,
	pub slider_value_f: f32,
	pub text_input_val: String,
	pub can_continue: bool,
	pub computed: bool,
	pub models: Option<(Model, Model, Model)>, // (sec_dim, flow, freq)
	pub result: (f32, f32, f32, f32),          // (minor, pbs, fluoSurf, freq)
}

impl PredictionUI {
	pub fn new() -> Self {
		PredictionUI {
			selection: None,
			slider_value_f: 0.0,
			text_input_val: "".to_string(),
			can_continue: false,
			computed: false,
			result: (0.0, 0.0, 0.0, 0.0),
			models: None,
		}
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::DeviceSelected(device) => {
				let model_path = device.model_path();

				let sec_dim_model = load_model(&model_path.sec_dim);
				let flow_model = load_model(&model_path.flow);
				let freq_model = load_model(&model_path.freq);

				self.selection = Some(device);
				self.can_continue = true;
				self.computed = false;
				self.models = Some((sec_dim_model, flow_model, freq_model));
			}

			SceneMessage::SliderChanged(value) => {
				self.slider_value_f = value;
				self.text_input_val = value.to_string();

				if let (Some(models), Some(_channel)) = (self.models.as_ref(), self.selection) {
					self.result = match compute(value, &models.0, &models.1, &models.2) {
						Ok(metrics) => {
							println!("ratio: {}", value / metrics.0);
							metrics
						}
						Err(_) => (0.0, 0.0, 0.0, 0.0),
					}
				}
			}

			SceneMessage::InputChanged(value) => {
				self.text_input_val = value;
				self.can_continue = true;
				self.computed = false;
			}

			SceneMessage::GoPressed => {
				let major_axis = self.text_input_val.parse::<f32>().unwrap();

				if let Some(models) = self.models.as_ref() {
					self.result = match compute(major_axis, &models.0, &models.1, &models.2) {
						Ok(metrics) => metrics,
						Err(_) => (0.0, 0.0, 0.0, 0.0),
					}
				}
				self.computed = true;
				self.can_continue = false;
			}

			_ => {}
		}
	}

	pub fn view<'a>(&self) -> Column<'a, SceneMessage> {
		let pick_list = pick_list(
			&Device::ALL[..],
			self.selection,
			SceneMessage::DeviceSelected,
		)
		.placeholder("Choose a device...")
		.padding(10);

		let slider_f = Slider::new(
			// &mut self.slider_state_f,
			10.0..=250.0,
			self.slider_value_f,
			SceneMessage::SliderChanged,
		);

		// let slider_f = slider(50..=150, slider_value_f as i32, SceneMessage::SliderChanged(f32 as i32));

		let text_input = text_input(
			"Major Dimension",
			&self.text_input_val,
			SceneMessage::InputChanged,
		)
		.padding(10)
		.width(Length::Units(150));

		// Controls Row
		let mut controls: Row<SceneMessage> = row();

		if self.can_continue {
			controls =
				// controls.push(button(&mut self.go_button, "Go").on_press(Message::GoPressed));
                controls.push(button("Go").on_press(SceneMessage::GoPressed));
		}

		// let mut result_display = Column::new().padding(0).width(Length::Units(500));
		let mut result_display = column();

		let major_axis = if let Ok(x) = self.text_input_val.parse::<f32>() {
			x
		} else {
			0.0
		};

		let canvas: Canvas<SceneMessage, Droplet> = Canvas::new(Droplet {
			radii: (major_axis, self.result.0),
		});

		if let Some(device) = self.selection {
			// let input: f32 = text_input_val.parse().unwrap();
			let max_val = device.max_value();

			let out_msg: (Text, Text, Text) = if major_axis <= max_val as f32 {
				let (.., pbs, fluo, freq) = self.result;

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
}

fn icon(unicode: char) -> Text {
	Text::new(unicode.to_string())
		.font(ICONS)
		.width(Length::Units(20))
		.horizontal_alignment(alignment::Horizontal::Center)
		.size(20)
}

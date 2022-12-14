#![allow(unused)]
use iced::pure::{container, horizontal_rule, horizontal_space, row};
use std::error::Error;
use std::path::PathBuf;

use super::super::styling::{btn, drop_down, tglr, tinput, BOLD};
use super::super::Droplet;
use super::SceneMessage;
use crate::colors::{Extended, Palette, EXTENDED_DARK, EXTENDED_LIGHT};

use iced::pure::widget::{Canvas, PickList, Row, Text};
use iced::{
	alignment,
	canvas::{Cursor, Frame, Geometry, Path, Stroke},
	pure::{
		column, text, vertical_space,
		widget::{
			canvas::{self, Program},
			Column,
		},
		Element, Sandbox,
	},
	Color, Length, Point, Rectangle, Vector,
};

use iced_style::{button, menu, pick_list, text_input, toggler};
use native_dialog::FileDialog;
use pyo3::{prelude::*, types::PyString};
use tract_onnx::prelude::*;

// -------------------------------------------------- PREDICTION UI
pub struct PredictionUI {
	selection: Option<Device>,
	input_data: PredictionInputs,
	prediction_data: Option<(f32, f32)>, // pbs,fluosurf
	user_model_path: Option<PathBuf>,
	user_model_toggle: bool,
	error: Result<(), Box<dyn Error>>,
}

/*
* Defining types of inputs on prediction view
* needed to sort out which input to update
* when a new text_input even fires.
*/
#[derive(Clone, Debug)]
pub enum PredictionInput {
	DimA(String),
	DimB(String),
	Freq(String),
}

// For encapsulating all the inputs on the page
#[derive(Default)]
struct PredictionInputs {
	dim_a: Option<String>,
	dim_b: Option<String>,
	freq: Option<String>,
}

impl PredictionUI {
	pub fn new() -> Self {
		PredictionUI {
			selection: None,
			input_data: PredictionInputs::default(),
			prediction_data: None,
			user_model_path: None,
			user_model_toggle: false,
			error: Ok(()),
		}
	}

	fn title(&self) -> String {
		String::from("Prediction")
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::SelectModel => {
				self.user_model_path = FileDialog::new()
					.add_filter("ONNX File", &["onnx"])
					.show_open_single_file()
					.unwrap();
			}
			SceneMessage::UserModelToggled(value) => self.user_model_toggle = value,
			SceneMessage::DeviceSelected(device) => {
				self.user_model_path = Some(PathBuf::from(device.path()));
				self.selection = Some(device);
			}
			SceneMessage::PredictionInputChanged(input) => {
				let PredictionInputs { dim_a, dim_b, freq } = &mut self.input_data;

				match input {
					PredictionInput::DimA(value) => {
						if value.parse::<f32>().is_ok() || value.is_empty() {
							*dim_a = Some(value)
						};
					}
					PredictionInput::DimB(value) => {
						if value.parse::<f32>().is_ok() || value.is_empty() {
							*dim_b = Some(value);
						};
					}
					PredictionInput::Freq(value) => {
						if value.parse::<f32>().is_ok() {
							*freq = Some(value);
						};
					}
				}
			}
			SceneMessage::GoPressed => {
				if let Err(e) = self.get_inference() {
					self.error = Err(Box::from(e));
				};
			}
			_ => {}
		}
	}

	pub fn view(&self) -> Column<SceneMessage> {
		// Dropdown
		let pick_list: PickList<'_, Device, SceneMessage> = drop_down(
			&Device::ALL[..],
			self.selection,
			SceneMessage::DeviceSelected,
		)
		.placeholder("Choose a device...")
		.width(Length::Units(200));

		let mut model_selection = row()
			.push(
				tglr(
					"Load own model?",
					self.user_model_toggle,
					SceneMessage::UserModelToggled,
				)
				.width(Length::Shrink), // .spacing(10),
			)
			.align_items(iced::Alignment::Center)
			.push(horizontal_space(Length::Fill));

		model_selection = if self.user_model_toggle {
			model_selection
				.push(btn("Select model", SceneMessage::SelectModel).width(Length::Units(200)))
		} else {
			model_selection.push(pick_list)
		};

		// Text inputs

		let (dim_a, dim_b, freq): (&str, &str, &str) = (
			self.input_data.dim_a.as_deref().unwrap_or(""),
			self.input_data.dim_b.as_deref().unwrap_or(""),
			self.input_data.freq.as_deref().unwrap_or(""),
		);

		let mut inputs = column()
			.push(horizontal_rule(1))
			.push(Self::input_row("Droplet length (µm)", dim_a, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::DimA(s))
			}))
			.push(Self::input_row("Droplet height (µm)", dim_b, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::DimB(s))
			}))
			.push(Self::input_row("Frequency (Hz)", freq, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::Freq(s))
			}))
			// .push(Self::input_row("Capillary number", capillary, move |s| {
			// 	SceneMessage::PredictionInputChanged(PredictionInput::Capillary(s))
			// }))
			// .push(Self::input_row(
			// 	"Interfacial tension (mN/m)",
			// 	interfacial,
			// 	move |s| SceneMessage::PredictionInputChanged(PredictionInput::Interfacial(s)),
			// ))
			.spacing(10);

		if self.user_model_path.is_some()
			&& self.input_data.dim_a.is_some()
			&& self.input_data.dim_b.is_some()
			&& self.input_data.freq.is_some()
		{
			inputs = inputs.push(btn("Predict Flow", SceneMessage::GoPressed));
		}

		inputs = inputs.push(horizontal_rule(1));

		// -------------------- CANVAS
		let (dim_a, dim_b) = (dim_a.trim().parse::<f32>(), dim_b.trim().parse::<f32>());

		let graphical_element: Canvas<SceneMessage, Droplet> = Canvas::new(Droplet {
			radii: (
				*dim_b.as_ref().unwrap_or(&0.),
				*dim_a.as_ref().unwrap_or(&0.),
			),
		});

		// -------------------- RESULT
		let mut inference_res: Column<SceneMessage> = column();
		if let Some((pbs, flu)) = self.prediction_data {
			inference_res = inference_res
				.push(
					row()
						.push(text("PBS:"))
						.push(horizontal_space(Length::Fill))
						.push(text(format!("{pbs} µL/min"))),
				)
				.push(
					row()
						.push(text("FluoSurf:"))
						.push(horizontal_space(Length::Fill))
						.push(text(format!("{flu} µL/min"))),
				);
		};

		if let Err(e) = &self.error {
			inference_res = column().push(Text::new(e.to_string()));
		};

		let result = row()
			.spacing(20)
			.push(
				container(
					graphical_element
						.width(Length::Units(200))
						.height(Length::Units(200)),
				)
				.center_x(),
			)
			.push(inference_res);

		// -------------------- FINAL
		/*
		 * Sadly no emoji support at the moment in iced
		 * however, I could use a explicit glyphs from fontawesome
		 */

		let mut view = column().spacing(10).push(model_selection);

		if self.user_model_path.is_some() {
			let file_name = self
				.user_model_path
				.as_ref()
				.unwrap()
				.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.to_owned();

			view = view.push(
				row()
					.push(text("Selected model"))
					.push(horizontal_space(Length::Fill))
					.push(text(file_name).font(BOLD)),
			);
		}

		view.push(Text::new("Please select the model and enter the following parameters. The button will not be available until all required parameters are provided.")).push(inputs).push(result)
	}

	// -------------------- UTILITY
	fn get_inference(&mut self) -> TractResult<()> {
		let path = self.user_model_path.as_ref();

		let (dim_a, dim_b, freq) = (
			self.input_data.dim_a.as_ref().unwrap().parse::<f32>(),
			self.input_data.dim_b.as_ref().unwrap().parse::<f32>(),
			self.input_data.freq.as_ref().unwrap().parse::<f32>(),
		);

		if path.is_some() && dim_a.is_ok() && dim_b.is_ok() && freq.is_ok() {
			let path = path.unwrap().to_str().unwrap();

			let model = tract_onnx::onnx()
				.model_for_path(path)?
				.with_input_fact(0, f32::fact(&[1, 3]).into())?
				.into_optimized()?
				.into_runnable()?;

			let vals: Vec<f32> = vec![dim_a.unwrap(), dim_b.unwrap(), freq.unwrap()];

			let input = tract_ndarray::arr1(&vals).into_shape((1, 3)).unwrap();

			// Input the generated data into the model
			let result = model.run(tvec!(input.into())).unwrap();

			let to_show = result[0].to_array_view::<f32>()?;

			let out = to_show.as_slice().unwrap();

			self.prediction_data = Some((out[0], out[1]));
		}
		Ok(())
	}

	fn input_row<'a>(
		label: &str,
		dis_val: &str,
		update: impl Fn(String) -> SceneMessage + 'a,
	) -> Row<'a, SceneMessage> {
		row()
			.push(text(label))
			.push(horizontal_space(Length::Fill))
			.push(
				tinput("", dis_val, update)
					.width(Length::Units(200))
					.padding(10),
			)
			.align_items(iced::Alignment::Center)
	}
}

// Microfluidic Devicede finitions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Device {
	CH100,
	CH190,
	CH275,
}

impl Device {
	const ALL: [Self; 3] = [Device::CH100, Device::CH190, Device::CH275];

	fn max_value(&self) -> f32 {
		match self {
			Device::CH100 => 100.0,
			Device::CH190 => 190.0,
			Device::CH275 => 275.0,
		}
	}

	fn path(&self) -> String {
		let model_file = match self {
			Device::CH100 => "100.onnx",
			Device::CH190 => "190.onnx",
			Device::CH275 => "275.onnx",
		};

		format!(
			"{}{model_file}",
			concat!(env!("CARGO_MANIFEST_DIR"), "/assets/models/")
		)
	}
}
impl std::fmt::Display for Device {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Device::CH100 => "100",
				Device::CH190 => "190",
				Device::CH275 => "275",
			}
		)
	}
}

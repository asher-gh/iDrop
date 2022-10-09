// #![allow(unused)]
#![allow(unused_must_use, dead_code)]

use iced::canvas::{Cursor, Frame, Geometry, Path, Stroke};
use iced::pure::widget::canvas::{self, Program};
use iced::pure::widget::{Button, Canvas, Column, PickList, Row, Text, TextInput, Toggler};
use iced::pure::{
	button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
	text_input, toggler, Element, Sandbox,
};
use iced::pure::{pick_list, vertical_space};
use iced::{alignment, Color, Font, Length, Point, Rectangle, Space, Vector};
use iced_style::{button, menu, pick_list, text_input, toggler};
use native_dialog::FileDialog;
use pyo3::prelude::*;
use pyo3::types::PyString;
use std::borrow::Cow;
use std::path::PathBuf;
use tract_onnx::prelude::*;

pub fn create_model(path: String, model_name: &str) -> PyResult<()> {
	let python_code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/create_model.py"));
	let from_py: PyResult<_> = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
		let script = PyModule::from_code(py, python_code, "", "")?;
		let path = PyString::new(py, &path);
		let new_model: Py<PyAny> = script.getattr("new_model")?.into();
		new_model.call1(py, (path, model_name))
	});

	if let Err(e) = from_py {
		println!("{e}");
	};

	Ok(())
}

// -------------------------------------------------- GRAPHICS
pub struct App {
	scenes: Scenes,
	theme: Theme,
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

		let mut controls = row();

		if scenes.has_previous() {
			controls = controls.push(btn("Back", Message::BackPressed));
		}

		controls = controls.push(horizontal_space(Length::Fill));

		if scenes.can_continue() {
			controls = controls.push(button("Next").style(theme).on_press(Message::NextPressed));
		}

		let content: Element<_> = column()
			.push(scenes.view().map(Message::SceneMessage))
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

		container(scrollable).center_y().into()
	}
}

struct Scenes {
	current: usize,
	list: Vec<Scene>,
}

impl Scenes {
	fn new() -> Scenes {
		Scenes {
			list: Scene::all_scenes(),
			current: 0,
		}
	}

	fn title(&self) -> &str {
		self.list[self.current].title()
	}

	fn update(&mut self, event: SceneMessage) {
		self.list[self.current].update(event)
	}

	fn has_previous(&self) -> bool {
		self.current > 0
	}

	fn can_continue(&self) -> bool {
		self.current < self.list.len() - 1
	}

	fn view(&self) -> Element<SceneMessage> {
		self.list[self.current].view()
	}
}

enum Scene {
	Greeter,
	Training(TrainingUI),
	Prediction(PredictionUI),
}

impl Scene {
	pub fn all_scenes() -> Vec<Scene> {
		vec![
			Scene::Prediction(PredictionUI::new()),
			Scene::Training(TrainingUI::new()),
			Scene::Greeter,
		]
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match self {
			// Scene::Greeter => {}
			Scene::Prediction(ui) => ui.update(msg),
			Scene::Training(ui) => ui.update(msg),
			_ => {}
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
		false
	}

	pub fn view(&self) -> Element<SceneMessage> {
		match self {
			Scene::Greeter => Self::welcome(),
			Scene::Prediction(ui) => Self::container(self.title()).push(ui.view()),
			Scene::Training(ui) => Self::container(self.title()).push(ui.view()),
			// Scene::Training => Self::training(), Scene::End => Self::end(),
		}
		.into()
	}

	fn container(title: &str) -> Column<SceneMessage> {
		column()
			.push(text(title).size(50))
			// .spacing(20)
			.height(Length::Shrink)
	}

	fn welcome() -> Column<'static, SceneMessage> {
		Self::container("Welcome!")
			.push("
Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.
            ")
            .push(vertical_space(Length::Units(20)))
			.push("Created by Ashish")
			.push("Contributions by Dr. Claire Barnes and Dr. Francesco Guidice")
	}
}

// -------------------------------------------------- TRAINING UI
struct TrainingUI {
	pub selected_model: Option<UserModel>,
	pub model_name: String,
	pub _new_model: bool,
	pub data_path: Option<PathBuf>,
	pub models: Vec<UserModel>,
	pub creation_toggle: bool,
	model_save_path: Option<PathBuf>,
}
impl TrainingUI {
	pub fn new() -> Self {
		TrainingUI {
			selected_model: None,
			model_name: String::new(),
			_new_model: false,
			data_path: None,
			models: Vec::new(), // add persistance later
			creation_toggle: false,
			model_save_path: None,
		}
	}

	pub fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::ModelSelected(model) => {
				self.selected_model = Some(model);
			}
			SceneMessage::SelectModelSavePath => {
				self.model_save_path = FileDialog::new().show_open_single_dir().unwrap();
			}

			SceneMessage::InputChanged(value) => self.model_name = value,

			SceneMessage::SelectCSV => {
				self.data_path = FileDialog::new()
					.add_filter("CSV File", &["csv"])
					.show_open_single_file()
					.unwrap();
			}

			SceneMessage::CreateToggled(value) => self.creation_toggle = value,

			SceneMessage::GoPressed => {
				if let Some(pathbuf) = &self.data_path {
					let path = pathbuf.to_str().unwrap();
					// let _x = create_model(String::from(path), &self.model_name);
					let model_name = format!("{}", self.model_name);

					let model_name_path = if let Some(save_path) = &self.model_save_path {
						format!("{}/{}", save_path.to_str().unwrap(), model_name)
					} else {
						model_name
					};

					create_model(String::from(path), &model_name_path);
				} else {
					println!("python script not called");
				};
			}
			_ => {}
		};
	}

	pub fn view(&self) -> Column<SceneMessage> {
		// --------------------COMPONENTS--------------------
		let pick_list = drop_down(
			&self.models,
			self.selected_model.clone(),
			SceneMessage::ModelSelected,
		)
		.style(Theme::Light)
		.placeholder("Pick a model")
		.padding(10);

		let toggle_create: Row<SceneMessage> = row().push(
			tglr(
				"New model",
				self.creation_toggle,
				SceneMessage::CreateToggled,
			)
			.width(Length::Shrink),
		);

		let text_input = text_input(
			"Provide name for new Model",
			&self.model_name,
			SceneMessage::InputChanged,
		)
		.padding(10)
		.width(Length::Units(250));

		let mut file_selection: Row<SceneMessage> = row()
			.push(btn("Load data", SceneMessage::SelectCSV))
			.push(Space::with_width(Length::Fill));

		let mut save_path: Row<SceneMessage> = row()
			.push(btn(
				"Model save location",
				SceneMessage::SelectModelSavePath,
			))
			.push(Space::with_width(Length::Fill));

		// --------------------ASSEMBLING--------------------

		let mut controls = row()
			.push(toggle_create)
			.push(Space::with_width(Length::Fill));

		if self.creation_toggle {
			controls = controls.push(text_input);
		} else {
			controls = controls.push(pick_list);
		}

		if let Some(file_path) = &self.data_path {
			let file_name = file_path.file_name().unwrap().to_str().unwrap();
			file_selection = file_selection.push(Text::new("CSV file: "));
			file_selection = file_selection.push(Text::new(file_name).font(BOLD));
		}

		if let Some(path) = &self.model_save_path {
			let path = path.to_str().unwrap();
			save_path = save_path.push(Text::new(format!("Saving to {}", path)));
		}

		let create_model_btn = btn("Create model", SceneMessage::GoPressed);

		let mut view = column()
			.height(Length::Fill)
			.spacing(25)
			.push(controls)
			.push(file_selection)
			.push(save_path)
			.spacing(20)
			.push(horizontal_rule(10));

		if *&self.data_path.is_some() && !*&self.model_name.is_empty() {
			view = view.push(create_model_btn);
		}

		view.height(Length::Shrink)
	}
}

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
	// --------------------
	DeviceSelected(Device),
	PredictionInputChanged(PredictionInput),
	SelectModel,
	UserModelToggled(bool),
}

#[derive(Clone, Debug)]
pub enum PredictionInput {
	DimA(String),
	DimB(String),
	Freq(String),
	Capillary(String),
	Interfacial(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserModel {
	pub name: String,
	pub path: Option<String>,
}

impl std::fmt::Display for UserModel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.name)
	}
}

const BOLD: Font = Font::External {
	name: "Poppins-Bold",
	bytes: include_bytes!("../assets/fonts/Poppins/Poppins-Bold.ttf"),
};

// -------------------------------------------------- PREDICTION UI
pub struct PredictionUI {
	selection: Option<Device>,
	input_data: PredictionInputs,
	prediction_data: Option<(f32, f32)>, // pbs,fluosurf
	user_model_path: Option<PathBuf>,
	user_model_toggle: bool,
}

#[derive(Default)]
struct PredictionInputs {
	dim_a: Option<String>,
	dim_b: Option<String>,
	freq: Option<String>,
	capillary: Option<String>,
	interfacial: Option<String>,
}

impl PredictionUI {
	fn new() -> Self {
		PredictionUI {
			selection: None,
			input_data: PredictionInputs::default(),
			prediction_data: None,
			user_model_path: None,
			user_model_toggle: false,
		}
	}

	fn title(&self) -> String {
		String::from("Prediction")
	}

	fn update(&mut self, msg: SceneMessage) {
		match msg {
			SceneMessage::SelectModel => {
				self.user_model_path = FileDialog::new()
					.add_filter("ONNX File", &["onnx"])
					.show_open_single_file()
					.unwrap();
			}
			SceneMessage::UserModelToggled(value) => self.user_model_toggle = value,
			SceneMessage::DeviceSelected(device) => {
				self.selection = Some(device);
			}
			SceneMessage::PredictionInputChanged(input) => {
				let PredictionInputs {
					dim_a,
					dim_b,
					capillary,
					freq,
					interfacial,
				} = &mut self.input_data;
				match input {
					PredictionInput::DimA(value) => {
						*dim_a = Some(value);
					}
					PredictionInput::DimB(value) => {
						*dim_b = Some(value);
					}
					PredictionInput::Capillary(value) => {
						*capillary = Some(value);
					}
					PredictionInput::Freq(value) => {
						*freq = Some(value);
					}
					PredictionInput::Interfacial(value) => {
						*interfacial = Some(value);
					}
				}
			}
			SceneMessage::GoPressed => {
				self.get_inference();
			}
			_ => {}
		}
	}

	fn view(&self) -> Column<SceneMessage> {
		// --------------------- DROPDOWN
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

		// -------------------- TEXT INPUTS

		let (dim_a, dim_b, freq, capillary, interfacial): (&str, &str, &str, &str, &str) = (
			self.input_data.dim_a.as_deref().unwrap_or(""),
			self.input_data.dim_b.as_deref().unwrap_or(""),
			self.input_data.freq.as_deref().unwrap_or(""),
			self.input_data.capillary.as_deref().unwrap_or(""),
			self.input_data.interfacial.as_deref().unwrap_or(""),
		);

		let inputs = column()
			.push(horizontal_rule(1))
			.push(Self::input_row("First dimension (µm)", dim_a, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::DimA(s))
			}))
			.push(Self::input_row("Second dimension (µm)", dim_b, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::DimB(s))
			}))
			.push(Self::input_row("Frequency", freq, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::Freq(s))
			}))
			.push(Self::input_row("Capillary number", capillary, move |s| {
				SceneMessage::PredictionInputChanged(PredictionInput::Capillary(s))
			}))
			.push(Self::input_row(
				"Interfacial tension (mN/m)",
				interfacial,
				move |s| SceneMessage::PredictionInputChanged(PredictionInput::Interfacial(s)),
			))
			.spacing(10)
			.push(btn("Predict Flow", SceneMessage::GoPressed))
			.push(horizontal_rule(1));

		// -------------------- CANVAS
		let (dim_a, dim_b) = (dim_a.trim().parse::<f32>(), dim_b.trim().parse::<f32>());

		let graphical_element: Canvas<SceneMessage, Droplet> = Canvas::new(Droplet {
			radii: (
				*dim_a.as_ref().unwrap_or(&0.),
				*dim_b.as_ref().unwrap_or(&0.),
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

		view.push(inputs).push(result)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Device {
	CH100,
	CH190,
	CH270,
}

impl Device {
	const ALL: [Self; 3] = [Device::CH100, Device::CH190, Device::CH270];

	fn max_value(&self) -> f32 {
		match self {
			Device::CH100 => 100.0,
			Device::CH190 => 190.0,
			Device::CH270 => 270.0,
		}
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
				Device::CH270 => "270",
			}
		)
	}
}

#[derive(Debug)]
pub struct Droplet {
	pub radii: (f32, f32),
}

impl Program<SceneMessage> for Droplet {
	type State = ();

	fn draw(&self, _state: &Self::State, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		let mut frame = Frame::new(bounds.size());
		let center = frame.center();
		let stroke_width = 1.0;
		let padding = 10.0;
		let drop_width = (frame.width() / 2.0) - padding - stroke_width;
		let drop_height = (frame.height() / 2.0) - padding - stroke_width;

		let drop_fill = Color::from_rgb8(63, 183, 250);
		let drop_outline = Color::from_rgb8(100, 100, 100);

		let (mut x, mut y) = self.radii;

		let aspect = x / y;

		if x.is_normal() & x.is_normal() {
			if aspect >= 1.0 {
				x = drop_width;
				y = drop_width / aspect;
			} else {
				y = drop_height;
				x = drop_height * aspect;
			}
		}

		let droplet_frame = Path::new(|path| {
			path.move_to(Point::ORIGIN);
			path.line_to(Point {
				y: frame.height(),
				..Point::ORIGIN
			});
			path.line_to(Point {
				x: frame.width(),
				y: frame.height(),
			})
		});

		let background = Path::new(|path| {
			path.ellipse(canvas::path::arc::Elliptical {
				center,
				radii: Vector { x, y },
				start_angle: 0.0,
				end_angle: 2.0 * std::f32::consts::PI,
				rotation: std::f32::consts::FRAC_PI_2,
			})
		});

		let stroke = Stroke {
			width: stroke_width,
			color: drop_outline,
			..Stroke::default()
		};

		let text = canvas::Text {
			horizontal_alignment: alignment::Horizontal::Left,
			vertical_alignment: alignment::Vertical::Top,
			size: 15.0,
			..canvas::Text::default()
		};

		frame.fill(&background, drop_fill);
		frame.stroke(&background, stroke);
		frame.stroke(&droplet_frame, stroke);

		// dim a (top left)
		frame.fill_text(canvas::Text {
			content: format!("{:2}", self.radii.0.to_string()),
			position: Point { x: 2.0, y: 0.0 },
			..text
		});

		// dim b (bottom right)
		frame.fill_text(canvas::Text {
			content: format!("{:2}", self.radii.1.to_string()),
			position: Point {
				x: frame.width(),
				y: frame.height() - 2.0,
			},
			horizontal_alignment: alignment::Horizontal::Right,
			vertical_alignment: alignment::Vertical::Bottom,
			..text
		});

		frame.fill_rectangle(
			Point::ORIGIN,
			frame.size(),
			Color::from_rgba8(0, 0, 0, 0.35),
		);

		vec![frame.into_geometry()]
	}
}

// ==========STYLING ==========
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
	Light,
	Dark,
}

mod styling;
use styling::palette::Palette;
use styling::palette::{Extended, EXTENDED_DARK, EXTENDED_LIGHT};

impl Theme {
	pub fn palette(self) -> Palette {
		match self {
			Self::Light => Palette::LIGHT,
			Self::Dark => Palette::DARK,
		}
	}

	pub fn extended_palette(&self) -> &Extended {
		match self {
			Self::Light => &EXTENDED_LIGHT,
			Self::Dark => &EXTENDED_DARK,
		}
	}
}

impl Default for Theme {
	fn default() -> Self {
		Self::Light
	}
}

/*
 * Button
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Brn {
	Primary,
	Secondary,
	Positive,
	Destructive,
	Text,
}

impl Default for Brn {
	fn default() -> Self {
		Self::Primary
	}
}

impl button::StyleSheet for Theme {
	fn active(&self) -> button::Style {
		let palette = self.extended_palette();

		let appearance = button::Style {
			border_radius: 2.0,
			..Default::default()
		};

		let from_pair = |pair: crate::styling::palette::Pair| button::Style {
			background: Some(pair.color.into()),
			text_color: pair.text,
			..appearance
		};

		from_pair(palette.primary.strong)
	}

	fn hovered(&self) -> button::Style {
		let active = self.active();
		let palette = self.extended_palette();

		button::Style {
			background: Some(iced::Background::Color(palette.primary.base.color)),
			..active
		}
	}
}

/*
 * Toggler
 */
impl toggler::StyleSheet for Theme {
	fn active(&self, is_active: bool) -> toggler::Style {
		let palette = self.extended_palette();

		toggler::Style {
			background: if is_active {
				palette.primary.strong.color
			} else {
				palette.background.strong.color
			},
			background_border: None,
			foreground: if is_active {
				palette.primary.strong.text
			} else {
				palette.background.base.color
			},
			foreground_border: None,
		}
	}

	fn hovered(&self, is_active: bool) -> toggler::Style {
		let palette = self.extended_palette();

		toggler::Style {
			background: if is_active {
				Color {
					a: 0.7,
					..palette.primary.weak.color
				}
			} else {
				palette.background.weak.text
			},
			..self.active(is_active)
		}
	}
}

/*
 * Pick List
 */
impl pick_list::StyleSheet for Theme {
	fn menu(&self) -> iced_style::menu::Style {
		let palette = self.extended_palette();
		menu::Style {
			text_color: palette.background.strong.text,
			background: palette.background.weak.color.into(),
			border_width: 1.0,
			border_color: palette.background.strong.color,
			selected_text_color: palette.primary.strong.text,
			selected_background: palette.primary.strong.color.into(),
		}
	}

	fn active(&self) -> pick_list::Style {
		let palette = self.extended_palette();

		pick_list::Style {
			text_color: palette.background.weak.text,
			background: palette.background.weak.color.into(),
			placeholder_color: palette.background.strong.color,
			border_radius: 2.,
			border_width: 1.,
			border_color: palette.background.strong.color,
			icon_size: 0.7,
		}
	}

	fn hovered(&self) -> pick_list::Style {
		let palette = self.extended_palette();

		pick_list::Style {
			text_color: palette.background.weak.text,
			background: palette.background.weak.color.into(),
			placeholder_color: palette.background.strong.color,
			border_radius: 2.0,
			border_width: 1.0,
			border_color: palette.primary.strong.color,
			icon_size: 0.7,
		}
	}
}

impl text_input::StyleSheet for Theme {
	fn active(&self) -> text_input::Style {
		let palette = self.extended_palette();

		text_input::Style {
			background: palette.background.base.color.into(),
			border_radius: 2.0,
			border_width: 1.0,
			border_color: palette.background.strong.color,
		}
	}

	fn hovered(&self) -> text_input::Style {
		let palette = self.extended_palette();

		text_input::Style {
			background: palette.background.base.color.into(),
			border_radius: 2.0,
			border_width: 1.0,
			border_color: palette.background.base.text,
		}
	}

	fn focused(&self) -> text_input::Style {
		let palette = self.extended_palette();

		text_input::Style {
			background: palette.background.base.color.into(),
			border_radius: 2.0,
			border_width: 1.0,
			border_color: palette.primary.strong.color,
		}
	}

	fn placeholder_color(&self) -> Color {
		let palette = self.extended_palette();

		palette.background.strong.color
	}

	fn value_color(&self) -> Color {
		let palette = self.extended_palette();

		palette.background.base.text
	}

	fn selection_color(&self) -> Color {
		let palette = self.extended_palette();

		palette.primary.weak.color
	}
}

fn btn<'a, T>(label: &'a str, msg: T) -> Button<'a, T> {
	button(Text::new(label).horizontal_alignment(alignment::Horizontal::Center))
		.on_press(msg)
		.style(Theme::Light)
}

fn tglr<'a, T>(label: &'a str, is_checked: bool, msg: impl Fn(bool) -> T + 'a) -> Toggler<'a, T> {
	toggler(label.to_owned(), is_checked, msg)
		.style(Theme::Light)
		.spacing(5)
}

fn drop_down<'a, M, T>(
	opt: impl Into<Cow<'a, [T]>>,
	selected: Option<T>,
	on_selected: impl Fn(T) -> M + 'a,
) -> PickList<'a, T, M>
where
	T: ToString + Eq + 'static,
	[T]: ToOwned<Owned = Vec<T>>,
{
	pick_list(opt, selected, on_selected).style(Theme::Light)
}

fn tinput<'a, M: Clone>(
	place_holder: &str,
	value: &str,
	on_change: impl Fn(String) -> M + 'a,
) -> TextInput<'a, M> {
	text_input(place_holder, value, on_change).style(Theme::Light)
}

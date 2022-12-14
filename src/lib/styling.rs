use iced::{
	alignment,
	pure::{
		button, container, pick_list, text_input, toggler,
		widget::{Button, Container, Image, PickList, Text, TextInput, Toggler},
	},
	Color, ContentFit, Font, Length,
};

use super::colors::{Extended, Palette, EXTENDED_DARK, EXTENDED_LIGHT};
use iced_style::{button, menu, pick_list, text_input, toggler};
use std::borrow::Cow;

// Loading custom font
pub(crate) const BOLD: Font = Font::External {
	name: "Poppins-Bold",
	bytes: include_bytes!("../../assets/fonts/Poppins/Poppins-Bold.ttf"),
};

// Theme profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
	Light,
	Dark,
}

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

// Button styles
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

		let from_pair = |pair: crate::colors::Pair| button::Style {
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

// Toggler styles
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

// Pick list styles
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

// Text input styles
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

// custom functions for button, toggler and text_input

pub fn btn<'a, T>(label: &'a str, msg: T) -> Button<'a, T> {
	button(Text::new(label).horizontal_alignment(alignment::Horizontal::Center))
		.on_press(msg)
		.style(Theme::Light)
}

pub fn tglr<'a, T>(
	label: &'a str,
	is_checked: bool,
	msg: impl Fn(bool) -> T + 'a,
) -> Toggler<'a, T> {
	toggler(label.to_owned(), is_checked, msg)
		.style(Theme::Light)
		.spacing(5)
}

pub fn tinput<'a, M: Clone>(
	place_holder: &str,
	value: &str,
	on_change: impl Fn(String) -> M + 'a,
) -> TextInput<'a, M> {
	text_input(place_holder, value, on_change).style(Theme::Light)
}

// function to place logo in the UI
pub fn logo<'a, M: Clone + 'a>(height: u16, content_fit: ContentFit) -> Container<'a, M> {
	container(
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
	.width(Length::Shrink)
}

pub fn drop_down<'a, M, T, U>(
	opt: impl Into<Cow<'a, [T]>>,
	selected: Option<T>,
	on_selected: U,
) -> PickList<'a, T, M>
where
	T: ToString + Eq + 'static,
	[T]: ToOwned<Owned = Vec<T>>,
	U: Fn(T) -> M + 'a,
{
	pick_list(opt, selected, on_selected).style(Theme::Light)
}

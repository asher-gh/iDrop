use iced::canvas::{Cursor, Frame, Geometry, Path, Stroke};
use iced::pure::widget::canvas::{self, Program};
use iced::{alignment, Color, Point, Rectangle, Vector};

use crate::SceneMessage;

// First, we define the data we need for drawing
#[derive(Debug)]
pub struct Droplet {
	pub radii: (f32, f32),
}

// Then, we implement the `Program` trait
impl Program<SceneMessage> for Droplet {
	type State = ();

	fn draw(&self, state: &Self::State, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		// We prepare a new `Frame`
		let mut frame = Frame::new(bounds.size());
		let center = frame.center();
		let stroke_width = 1.0;
		let padding = 10.0;
		let drop_width = (frame.width() / 2.0) - padding - stroke_width;
		let drop_height = (frame.height() / 2.0) - padding - stroke_width;

		let drop_fill = Color::from_rgb8(63, 183, 250);
		let drop_outline = Color::from_rgb8(100, 100, 100);

		let (mut x, mut y) = self.radii;
		// let (mut x, mut y) = (80.0f32, 70.0f32);

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
			// line_cap: LineCap::Round,
			..Stroke::default()
		};

		let mut text = canvas::Text {
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

		// frame.fill_rectangle(
		// 	Point::ORIGIN,
		// 	frame.size(),
		// 	Color::from_rgba8(0, 0, 0, 0.35),
		// );

		vec![frame.into_geometry()]
	}
}

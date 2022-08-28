use iced::canvas::{Cursor, Frame, Geometry, Path};
use iced::pure::widget::canvas::{self, Program};
use iced::{Color, Rectangle, Vector};

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
		let width = frame.width() / 2.0;
		let height = frame.height();

		// let (mut x, mut y) = self.radii;
		let (mut x, mut y) = (80.0f32, 70.0f32);

		let aspect = x / y; //1

		if x.is_normal() {
			if aspect > 1_f32 {
				y = width;
				x = width / aspect;
			} else {
				x = width;
				y = width / aspect;
			}
		}

		let background = Path::new(|path| {
			path.ellipse(canvas::path::arc::Elliptical {
				center,
				radii: Vector { x, y },
				start_angle: 0.0,
				end_angle: 2.0 * std::f32::consts::PI,
				rotation: std::f32::consts::FRAC_PI_2,
			})
		});

		frame.fill(&background, Color::from_rgb8(0x12, 0x93, 0xD8));

		// Finally, we produce the geometry
		vec![frame.into_geometry()]
	}
}

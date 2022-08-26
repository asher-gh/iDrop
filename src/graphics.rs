use iced::canvas::{self, Cache, Canvas, Cursor, Geometry, Path};
use iced::{mouse, Color, Container, Element, Length, Rectangle, Sandbox, Vector};

#[derive(Default)]
pub struct Droplet {
	droplet: Cache,
	pub radii: Vector,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {}

impl Sandbox for Droplet {
	type Message = Message;

	fn new() -> Self {
		Droplet {
			..Default::default()
		}
	}

	fn title(&self) -> String {
		String::from("Clock - Iced")
	}

	fn view(&mut self) -> Element<Message> {
		let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);

		Container::new(canvas)
			.width(Length::Fill)
			.height(Length::Fill)
			.padding(20)
			.into()
	}

	fn update(&mut self, _message: Self::Message) {}
}

impl<Message> canvas::Program<Message> for Droplet {
	fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		let geometry = self.droplet.draw(bounds.size(), |frame| {
			let center = frame.center();
			let Vector { mut x, mut y } = self.radii;
			let width = bounds.width / 2.0;
			let aspect = x / y;

			if x.is_normal() {
				if aspect >= 1.0 {
					x = width;
					y = width / aspect;
				} else {
					x = aspect * width;
					y = width;
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

			frame.translate(Vector::new(center.x, center.y));
		});

		vec![geometry]
	}

	fn update(
		&mut self,
		_event: canvas::Event,
		_bounds: Rectangle,
		_cursor: Cursor,
	) -> (canvas::event::Status, Option<Message>) {
		(canvas::event::Status::Ignored, None)
	}

	fn mouse_interaction(&self, _bounds: Rectangle, _cursor: Cursor) -> mouse::Interaction {
		mouse::Interaction::default()
	}
}

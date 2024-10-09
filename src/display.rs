use colored::{Color, Colorize};
use std::{
	i128,
	io::{BufWriter, Write},
	u128,
};

#[allow(dead_code)]
pub struct Display {
	view_x: i128,
	view_y: i128,
	width: i128,
	height: i128,
	out: BufWriter<std::io::Stdout>,
	life_color: colored::Color,
	dead_color: colored::Color,
	border_color: colored::Color,
	bg_color: colored::Color,
}

#[allow(dead_code)]
impl Display {
	pub fn new(
		view_x: i128, view_y: i128, width: i128, height: i128,
	) -> Display {
		Display {
			view_x,
			view_y,
			width,
			height,
			out: BufWriter::new(std::io::stdout()),
			life_color: Color::TrueColor { r: 183, g: 189, b: 248 },
			dead_color: Color::TrueColor { r: 36, g: 39, b: 58 },
			border_color: Color::TrueColor { r: 237, g: 135, b: 150 },
			// bg_color: Color::TrueColor { r: 49, g: 50, b: 68 },
			bg_color: Color::TrueColor { r: 24, g: 25, b: 38 },
		}
	}
}

#[allow(dead_code)]
impl Display {
	const LTC: &'static str = "╔"; //"┌┏╔╒╓";
	const RTC: &'static str = "╗"; //"┐┓╗╕╖";
	const MC: &'static str = "═"; //"─━═";
	const LBC: &'static str = "╚"; //"└┗╚╘╙";
	const RBC: &'static str = "╝"; //"┘┛╝╛╜";
	const MB: &'static str = "║"; //"│┃║";
	const LIFE: &'static str = "⏺"; // "⏺█○▢◈◉◌";
	const DEAD: &'static str = "∙"; // "｡⸱·∙◦";
	const SP: &'static str = " ";

	pub fn top_border(&mut self) {
		let lc = Self::LTC.color(self.border_color).on_color(self.bg_color);
		let rc = Self::RTC.color(self.border_color).on_color(self.bg_color);
		let mc = Self::MC.repeat(((self.width * 2) + 1) as usize);
		let mc = mc.color(self.border_color).on_color(self.bg_color);
		let format = format!("\x1B[1;1H\x1b[?25l{lc}{mc}{rc}\n");
		self.out.write_all(format.as_bytes()).unwrap();
	}

	pub fn bottom_border(&mut self) {
		let lc = Self::LBC.color(self.border_color).on_color(self.bg_color);
		let rc = Self::RBC.color(self.border_color).on_color(self.bg_color);
		let mc = Self::MC
			.repeat(((self.width * 2) + 1) as usize)
			.color(self.border_color)
			.on_color(self.bg_color);
		let format = format!("{lc}{mc}{rc}\n");
		self.out.write_all(format.as_bytes()).unwrap();
	}

	pub fn left_border(&mut self) {
		let lc = Self::MB.color(self.border_color).on_color(self.bg_color);
		let format = format!("{lc}{}", " ".on_color(self.bg_color));
		self.out.write_all(format.as_bytes()).unwrap();
	}

	pub fn right_border(&mut self) {
		let rc = Self::MB.color(self.border_color).on_color(self.bg_color);
		let format = format!("{}{rc}\n", " ".on_color(self.bg_color));
		self.out.write_all(format.as_bytes()).unwrap();
	}

	pub fn cell(&mut self, x: i128, is_alive: &bool) {
		let live = Self::LIFE.color(self.life_color).on_color(self.bg_color);
		let dead = Self::DEAD.color(self.dead_color).on_color(self.bg_color);

		let cell = if *is_alive { &live } else { &dead };
		let sp = ((x + 1) != (self.view_x + self.width)) as usize;
		let sp = Self::SP.repeat(sp).on_color(self.bg_color);
		self.out.write_all(format!("{cell}{sp}").as_bytes()).unwrap();
	}

	pub fn footer(&mut self, tc: usize, vc: usize, cc: u128) {
		let vcfpad = " ".repeat(20 - vc.to_string().len());
		let tcfpad = " ".repeat(20 - tc.to_string().len());

		let vcf = format!("{vc}{vcfpad}").color(self.life_color);
		let vc = "Live cells  (view): ".color(self.border_color);
		let tcf = format!("{tc}{tcfpad}").color(self.life_color);
		let tc = "Live cells (total): ".color(self.border_color);
		let ccf = format!("{cc}").color(self.life_color);
		let cc = "Cycle count       : ".color(self.border_color);

		let format = format!("{vc}{vcf}\n{tc}{tcf}\n{cc}{ccf}\n");
		self.out.write_all(format.as_bytes()).unwrap();
	}

	pub fn flush(&mut self) {
		self.out.flush().unwrap();
	}

	pub fn clear(&mut self) {
		self.out.write_all("\x1B[1;1H\x1B[2J".as_bytes()).unwrap();
		self.flush();
	}

	pub fn get_view_range(&self, is_x: bool) -> std::ops::Range<i128> {
		if is_x {
			self.view_x..self.view_x + self.width
		} else {
			self.view_y..self.view_y + self.height
		}
	}
}

#[allow(dead_code)]
impl Display {
	pub fn get_width(&self) -> i128 {
		self.width
	}

	pub fn get_height(&self) -> i128 {
		self.height
	}

	pub fn get_view_x(&self) -> i128 {
		self.view_x
	}

	pub fn get_view_y(&self) -> i128 {
		self.view_y
	}

	pub fn set_width(&mut self, width: i128) {
		self.width = width;
	}

	pub fn set_height(&mut self, height: i128) {
		self.height = height;
	}

	pub fn set_view_x(&mut self, view_x: i128) {
		self.view_x = view_x;
	}

	pub fn set_view_y(&mut self, view_y: i128) {
		self.view_y = view_y;
	}
}

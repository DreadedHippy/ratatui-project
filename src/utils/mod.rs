use ratatui::{style::{Style, Color}, text::Text};

pub const TABS: [&str; 8] = [
	"(1) Register",
	"(2) Deposit",
	"(3) Withdraw",
	"(4) Balance",
	"(5) Admin List",
	"(6) Close",
	"(7) Update",
	"(8) Exit",
];

pub enum CustomText {
}

impl CustomText {
	pub fn generate_prompt(string: &String) -> Text {
		Text::styled(string, Style::default().fg(Color::LightYellow))
	}
	
	pub fn generate_hint(string: &String) -> Text {
		Text::styled(string, Style::default().fg(Color::LightRed))
	}

	pub fn generate_input(string: &String) -> Text{
		Text::styled(string, Style::default())
	}
}

pub enum CustomInput {
	Char(char),
	Number(usize),
	Escape,
	Backspace,
	Left,
	Right,
	Enter
}

impl CustomInput {
	pub fn inner_value(&self) -> String {
		match self {
			CustomInput::Char(c) => c.to_string(),
			CustomInput::Number(n) => n.to_string(),
			_ => "".to_string()
		}
	}
}
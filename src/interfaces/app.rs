pub enum InputMode {
	Normal,
	Editing
}

pub struct AppInterface {
	pub tab_display: (Vec<String>, usize),
	pub input: String,
	pub input_mode: InputMode,
	pub display_text: Vec<String>,
	pub hint_text: String
}

impl AppInterface {
	pub fn select_tab(&mut self, tab_index: usize) {
		self.tab_display.1 = tab_index
	}
}
use std::io::Stdout;
use ratatui::{prelude::*, widgets::*, symbols::DOT};
pub mod app;
use app::AppInterface;

use crate::utils::CustomText;


pub fn ui(f: &mut ratatui::Frame<CrosstermBackend<Stdout>>, app: &AppInterface) {
	let selected = app.tab_display.1;
	let main_block = Block::default().title("Rust Bank").borders(Borders::ALL);
	let tabs = Interfaces::create_tab(app.tab_display.0.clone(), Some(selected));
	let output_block = Block::default().title("---------------").title_alignment(Alignment::Center).borders(Borders::ALL);
	let prompt_text = CustomText::generate_prompt(&app.display_text);
	let hint_text = CustomText::generate_hint(&app.hint_text);
	let input_text = CustomText::generate_input(&app.input);

	
	let in_main_block = main_block.inner(f.size());

	let (tabs_area, output_area, hint_area) = {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
			.split(in_main_block);
		(chunks[0], chunks[1], chunks[2])
	};

	
	let in_output_block = output_block.inner(output_area);
	let (text_display_area, input_area) = {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Length(1), Constraint::Percentage(50)])
			.split(in_output_block);
		(chunks[0], chunks[1])
	};

	f.render_widget(main_block, f.size());
	f.render_widget(tabs, tabs_area);
	f.render_widget(output_block, output_area);
	f.render_widget(Paragraph::new(prompt_text), text_display_area);
	f.render_widget(Paragraph::new(hint_text).alignment(Alignment::Right), hint_area);
	f.render_widget(Paragraph::new(input_text), input_area);
	// f.render_widget(User, area);
}


pub enum Interfaces {}

impl Interfaces {
	// pub fn paragaph() -> impl Widget {
	// 	let paragraph = Paragraph::new("This is a different paragraph");
	// 	paragraph
	// }

	// pub fn another_paragaph() -> impl Widget {
	// 	let paragraph = Paragraph::new("This is yet another different paragraph");
	// 	paragraph
	// }

	pub fn create_tab(tabs: Vec<String>, selected: Option<usize>) -> impl Widget {
		let selected = selected.unwrap_or_default();

		let tabs = Tabs::new(tabs)
		.style(Style::default().fg(Color::White))
		.select(selected)
		.highlight_style(Style::default().fg(Color::Green))
		.divider(DOT);
	
		tabs
	}
}
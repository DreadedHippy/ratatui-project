use std::io::Stdout;
use ratatui::{prelude::*, widgets::*, symbols::{block::ONE_EIGHTH, line::{TOP_RIGHT, BOTTOM_RIGHT}}};
pub mod app;
use app::AppInterface;

use crate::utils::CustomText;


pub fn ui(f: &mut ratatui::Frame<CrosstermBackend<Stdout>>, app: &mut AppInterface) {
	let selected = app.tab_display.1;
	let main_block = Block::default().title(Span::styled(
		"RUST BANK",
		Style::default().add_modifier(Modifier::BOLD).yellow(),
	)).title_alignment(Alignment::Center).borders(Borders::ALL);
	let tabs = Interfaces::create_tab(app.tab_display.0.clone(), Some(selected));
	let output_block = Block::default().title("---------------").title_alignment(Alignment::Center).borders(Borders::TOP);
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
			.constraints([Constraint::Percentage(80), Constraint::Length(1)])
			.split(in_output_block);
		(chunks[0], chunks[1])
	};

	let create_block = |title| {
		Block::default()
			.borders(Borders::ALL)
			.gray()
			.title(Span::styled(
				title,
				Style::default().add_modifier(Modifier::BOLD),
			))
	};

	let content_length = app.display_text.lines().enumerate().last().unwrap().0;

	app.scrollbar_state = app.scrollbar_state.content_length(content_length as u16);
	f.render_widget(main_block, f.size());
	f.render_widget(tabs, tabs_area);
	f.render_widget(output_block, output_area);
	f.render_widget(Paragraph::new(prompt_text).block(create_block("")).scroll((app.scroll as u16, 0)), text_display_area);
	f.render_stateful_widget(
    Scrollbar::default()
			.orientation(ScrollbarOrientation::VerticalRight)
			.begin_symbol(Some(TOP_RIGHT))
			.thumb_style(Style::default().light_magenta())
			.end_symbol(Some(BOTTOM_RIGHT)),
    text_display_area,
    &mut app.scrollbar_state,
	);
	f.render_widget(Paragraph::new(hint_text).alignment(Alignment::Right), hint_area);
	f.render_widget(Paragraph::new(input_text).block(create_block("").on_black()).fg(Color::Green), input_area);
}


pub enum Interfaces {}

impl Interfaces {
	pub fn create_tab(tabs: Vec<String>, selected: Option<usize>) -> impl Widget {
		let selected = selected.unwrap_or_default();

		let tabs = Tabs::new(tabs)
		.style(Style::default().fg(Color::White))
		.select(selected)
		.highlight_style(Style::default().fg(Color::LightGreen))
		.divider(ONE_EIGHTH) ;
	
		tabs
	}
}
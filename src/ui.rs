use ratatui::{
	layout::{Alignment, Layout, Direction, Constraint, Rect},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders, Paragraph},
	Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
	// This is where you add new widgets.
	// See the following resources:
	// - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
	// - https://github.com/ratatui-org/ratatui/tree/master/examples
	render_popup(app, frame);

	frame.render_widget(
		Paragraph::new(format!(
			"This is a tui template.\n\
				Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
				Press left and right to increment and decrement the counter respectively.\n\
				Counter: {:?}",
			app.words
		))
		.block(
			Block::default()
				.title("Template")
				.title_alignment(Alignment::Center)
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded),
		)
		.style(Style::default().fg(Color::Cyan).bg(Color::Black))
		.alignment(Alignment::Center),
		frame.size(),
	);
}

fn render_popup(app: &mut App, frame: &mut Frame) {
	if let Some(input_request) = app.get_input_request() {
		let popup_width = frame.size().width / 5;
		let popup_height = 3;

		let popup_rect = Rect::new((frame.size().width - popup_width) / 2, (frame.size().height - popup_height) / 2, popup_width, popup_height);

		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[
					Constraint::Length(1),
					Constraint::Length(2),
					Constraint::Min(0)
				]
				.as_ref(),
			).split(popup_rect);

		// TODO: Render help text frame.render_widget(help_message, chunks[0])
		let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor

		let scroll = app.input.visual_scroll(width as usize);
		let input = Paragraph::new(app.input.value())
			.style(Style::default().fg(Color::Yellow))
			.scroll((0, scroll as u16))
			.block(Block::default().borders(Borders::ALL).title("Input")); // TODO: Change input text depending on type of input
		frame.render_widget(input, popup_rect);
	}
}

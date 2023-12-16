use crate::app::{App, AppResult, InputRequest};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
	let code = key_event.code;
	if (code == KeyCode::Char('c') || code == KeyCode::Char('C')) && key_event.modifiers == KeyModifiers::CONTROL {
		app.quit();
		return Ok(());
	}

	if app.get_input_request().is_some() {
		input_key_events(key_event, app)
	} else {
		default_key_events(key_event, app)
	}
}

fn default_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
	match key_event.code {
		// Exit application on `ESC` or `q`
		KeyCode::Esc | KeyCode::Char('q') => {
			if key_event.kind == KeyEventKind::Press {
				// TODO: Popup to confirm this before actually quitting.
				app.quit();
			}
		}
		KeyCode::Char('l') => {
			if key_event.kind == KeyEventKind::Press {
				if key_event.modifiers == KeyModifiers::CONTROL {
					app.maybe_request_input(InputRequest::WordsLoad)
				}
			}
		}
		KeyCode::Char('s') => {
			if key_event.kind == KeyEventKind::Press {
				if key_event.modifiers == KeyModifiers::CONTROL {
					app.save_words()
				}
				if key_event.modifiers == KeyModifiers::CONTROL | KeyModifiers::SHIFT {
					app.maybe_request_input(InputRequest::WordsSave)
				}
			}
		}
		// Other handlers you could add here.
		_ => {}
	}
	Ok(())
}

fn input_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
	match key_event.code {
		KeyCode::Esc | KeyCode::Char('q') => app.reset_input(),
		KeyCode::Enter => app.finalise_input(),
		_ => app.input_key(key_event)
	}

	Ok(())
}
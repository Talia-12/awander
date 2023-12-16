use std::{error, path::PathBuf, str::FromStr};

use crossterm::event::{Event, KeyEvent};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::{entries::WordEntry, loader::{file_csv_reader, read_words}};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
	/// Is the application running?
	pub running: bool,
	/// words
	pub words: Vec<WordEntry>,
	pub words_loaded_from: Option<PathBuf>,
	input_request: Option<InputRequest>,
	pub input: Input
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputRequest {
	WordsLoad, WordsSave
}

impl Default for App {
	fn default() -> Self {
		Self {
			running: true,
			words: vec![],
			words_loaded_from: None,
			input_request: None,
			input: Input::default()
		}
	}
}

impl App {
	/// Constructs a new instance of [`App`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Handles the tick event of the terminal.
	pub fn tick(&self) {}

	/// Set running to false to quit the application.
	pub fn quit(&mut self) {
		self.running = false;
	}

	pub fn reset_input(&mut self) {
		self.input.reset();
		self.input_request = None;
	}

	pub fn input_key(&mut self, key: KeyEvent) {
		self.input.handle_event(&Event::Key(key));
	}

	pub fn finalise_input(&mut self) {
		if let Some(input_request) = self.input_request {
			let input = self.input.value().to_string();

			match input_request {
				InputRequest::WordsLoad => self.load_words(&input),
				InputRequest::WordsSave => {
					self.words_loaded_from = Some(PathBuf::from_str(&input).unwrap());
					self.save_words();
				},
			}

		}	else {
			println!("WARNING: called finalise_input without having requested input at some point.");
		}	

		self.reset_input();
	}

	pub fn maybe_request_input(&mut self, request: InputRequest) {
		if self.input_request.is_none() {
			self.input_request = Some(request);
		}
	}

	pub fn request_input(&mut self, request: InputRequest) -> Result<(), String> {
		if self.input_request.is_none() {
			self.input_request = Some(request);
			Ok(())
		} else {
			Err("Attempted to request input while input was aleady being requested.".to_string())
		}
	}

	pub fn get_input_request(&self) -> Option<InputRequest> {
		self.input_request
	}

	pub fn save_words(&self) {
		todo!()
	}

	pub fn load_words(&mut self, word_path: &str) {
		match file_csv_reader(word_path).and_then(read_words) {
			Ok(words) => {
				self.words = words;
				self.words_loaded_from = Some(PathBuf::from_str(word_path).unwrap());
			},
			Err(err) => println!("Error loading words from {}: {}", word_path, err.to_string()),
		}
	}
}

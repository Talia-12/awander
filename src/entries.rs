use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WordEntry {
	pub conlang_word: String,
	pub origlang_word: String,
	pub part_of_speech: String,
	pub definition: String
}

impl WordEntry {
	pub fn new(conlang_word: String, origlang_word: String, part_of_speech: String, definition: String) -> WordEntry {
		WordEntry { conlang_word, origlang_word, part_of_speech, definition }
	}

	pub fn new_str(conlang_word: &str, origlang_word: &str, part_of_speech: &str, definition: &str) -> WordEntry {
		WordEntry { conlang_word: conlang_word.to_string(), origlang_word: origlang_word.to_string(), part_of_speech: part_of_speech.to_string(), definition: definition.to_string() }
	}
}
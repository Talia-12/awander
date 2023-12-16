use std::{error::Error, fs::File, io::{Read, Write}, path::Path};

use csv::{Reader, Writer};

use crate::entries::WordEntry;

#[derive(Clone, Debug)]
pub struct LoadError {
	error_message: String
}
impl std::fmt::Display for LoadError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.error_message)
	}
}
impl Error for LoadError { }

pub fn file_csv_reader<P: AsRef<Path>>(path: P) ->  Result<Reader<File>, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = csv::Reader::from_reader(file);
	Ok(reader)
}

pub fn read_words<R: Read>(mut reader: Reader<R>) -> Result<Vec<WordEntry>, Box<dyn Error>> {
	let mut entries = Vec::<WordEntry>::new();

	for result in reader.records() {
		let record = result?;

		if record.len() < 4 {
			return Err(Box::new(LoadError { error_message: "Must have columns for conlang word, original language word, part of speech, and definition.".to_string() }));
		}

		entries.push(WordEntry::new(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string()));
	}

	Ok(entries)
}

pub fn file_csv_writer<P: AsRef<Path>>(path: P) ->  Result<Writer<File>, Box<dyn Error>> {
	let file = File::open(path)?;
	let writer = csv::Writer::from_writer(file);
	Ok(writer)
}

pub fn write_words<W: Write>(words: &Vec<WordEntry>, writer: &mut Writer<W>) -> Result<(), Box<dyn Error>> {
	writer.write_record(&["SYMPHAN WORD","ENGLISH WORD","PoS","DEFINITIONS"])?;

	for word in words {
		writer.write_record(&[
			word.conlang_word.to_string(),
			word.origlang_word.to_string(),
			word.part_of_speech.to_string(),
			word.definition.to_string()
		])?;
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use super::*;

	fn string_reader(str: &str) -> Reader<&[u8]> {
		Reader::from_reader(str.as_bytes())
	}

	#[test]
	fn test_read_words() {
		let input = "
SYMPHAN WORD,ENGLISH WORD,PoS,DEFINITIONS
(+1)(+1+5),how,Adverb,
(+1)(+4),long,Adjective,
(+1)(+5)(+3),when,Adverb,";

		assert_eq!(read_words(string_reader(input)).unwrap(), vec![
			WordEntry::new_str("(+1)(+1+5)", "how", "Adverb", ""),
			WordEntry::new_str("(+1)(+4)", "long", "Adjective", ""),
			WordEntry::new_str("(+1)(+5)(+3)", "when", "Adverb", "")
		]);

		let failing_input = "
SYMPHAN WORD,ENGLISH WORD,PoS,DEFINITIONS
(+1)(+1+5),how,Adverb,
(+1)(+4),long,
(+1)(+5)(+3),when,Adverb,";

		assert_eq!(read_words(string_reader(failing_input)).unwrap_err().to_string(), "CSV error: record 2 (line: 4, byte: 66): found record with 3 fields, but the previous record has 4 fields".to_string());

		let failing_input = "
SYMPHAN WORD,ENGLISH WORD,PoS
(+1)(+1+5),how,Adverb
(+1)(+4),long,
(+1)(+5)(+3),when,Adverb";

		assert_eq!(read_words(string_reader(failing_input)).unwrap_err().to_string(), "Must have columns for conlang word, original language word, part of speech, and definition.".to_string());
	}

	#[test]
	fn test_write_words() {
		let words = vec![
			WordEntry::new_str("A", "B", "C", ""),
			WordEntry::new_str("D", "E", "F", "G"),
			WordEntry::new_str("H", "I", "J", ""),
		];

		let mut wtr = Writer::from_writer(vec![]);

		write_words(&words,  &mut wtr).unwrap();

    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    assert_eq!(data, "SYMPHAN WORD,ENGLISH WORD,PoS,DEFINITIONS\nA,B,C,\nD,E,F,G\nH,I,J,\n");
	}
}
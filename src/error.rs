#[derive(Debug)]
pub enum Error {
    UnrecognizedPayload(BadPayloadDetails),
    PayloadMissingEntry(BadPayloadDetails),
    EmptyPayload(csv::StringRecord),
    ParseDateError(ParseErrorDetails<chrono::format::ParseError>),
    ParseTimeError(ParseErrorDetails<chrono::format::ParseError>),
    ParseIntError(ParseErrorDetails<std::num::ParseIntError>),
    Csv(csv::Error)
}

#[derive(Debug)]
pub struct BadPayloadDetails {
    pub record: csv::StringRecord,
    pub idx: usize,
    pub expected: Option<Vec<String>>
}

impl BadPayloadDetails {
    pub fn new(record: csv::StringRecord) -> Self {
        BadPayloadDetails {
            record,
            idx: 0,
            expected: None
        }
    }

    pub fn at_index(self, idx: usize) -> Self {
        BadPayloadDetails { idx, ..self }
    }

    pub fn expected_one_of(self, expected: Vec<String>) -> Self {
        BadPayloadDetails { expected: Some(expected), ..self }
    }
}


#[derive(Debug)]
pub struct ParseErrorDetails<E> {
    pub record: csv::StringRecord,
    pub idx: usize,
    pub error: E,
}

impl<E> ParseErrorDetails<E> {
    pub fn new(record: csv::StringRecord, idx: usize, error: E) -> Self {
        ParseErrorDetails {
            record, idx, error
        }
    }
}



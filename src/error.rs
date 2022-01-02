use std::fmt::{Display, Formatter, self};
use colored::Colorize;

#[derive(Debug)]
pub enum Error {
    UnrecognizedPayload(BadPayloadDetails),
    PayloadMissingEntry(BadPayloadDetails),
    EmptyPayload(csv::StringRecord),
    ParseDateError(ParseErrorDetails<chrono::format::ParseError>),
    ParseTimeError(ParseErrorDetails<chrono::format::ParseError>),
    ParseIntError(ParseErrorDetails<std::num::ParseIntError>),
    UnrecognizedPackage { report_type: String, report_subtype: String },
    Csv(csv::Error),
    Io(std::io::Error),
    Zip(zip::result::ZipError),
    Arrow(arrow::error::ArrowError),
    Parquet(parquet::errors::ParquetError),
    InvalidFilename(std::path::PathBuf),
    MissingColumnHeader(String),
    DatatypeMismatch { datatype: arrow::datatypes::DataType, value: Option<crate::flatfile::DataValue> },
    IndexError(usize),
    UnsupportedDataType(arrow::datatypes::DataType),
    NullError,
    UnsupportedFetchReport(crate::packages::Package),
    Reqwest(reqwest::Error),
    ScraperError,
    ZipUrlNoFilename(String),
    FailedToDownload { url: String, path: std::path::PathBuf, status: reqwest::StatusCode },
    InvalidYear(String),
    InvalidMonth(String),
    ManageError(crate::manage::state::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            UnrecognizedPayload(d) => 
                write!(f, "Unrecognized payload: {}", d),
            PayloadMissingEntry(d) => 
                write!(f, "Payload missing entry ({}): {}", d.idx, d),
            EmptyPayload(r) => 
                write!(f, "Empty payload:\n\n\t{}", r.iter().collect::<Vec<_>>().join(",")),
            ParseDateError(d) => 
                write!(f, "Parse date error: {}", d),
            ParseTimeError(d) => 
                write!(f, "Parse time error: {}", d),
            ParseIntError(d) => 
                write!(f, "Parse integer error: {}", d),
            UnrecognizedPackage { report_type, report_subtype } => 
                write!(f, "Unrecognized package:\n\tReport type:\t{}\n\tSubtype:\t{}", report_type, report_subtype), 
            Csv(e) => 
                write!(f, "CSV error: {}", e),
            Io(e) =>
                write!(f, "{}", e),
            Zip(e) => 
                write!(f, "{}", e),
            Arrow(e) =>
                write!(f, "{}", e),
            Parquet(e) =>
                write!(f, "{}", e),
            InvalidFilename(p) =>
                write!(f, "Invalid filename: {}", p.to_string_lossy()),
            MissingColumnHeader(c) =>
                write!(f, "Missing column header: {}", c),
            DatatypeMismatch { datatype, value } => 
                write!(f, "Datatype mismatch. Expected {} but got value {}", datatype, value.as_ref().map(|v| v.clone().as_string().unwrap()).unwrap_or("".to_string())), // TODO rm unwrap
            IndexError(i) => 
                write!(f, "Index error: {}", i),
            UnsupportedDataType(dt) =>
                write!(f, "Unsupported datatype: {}", dt),
            NullError =>
                write!(f, "Null found where not allowed"),
            UnsupportedFetchReport(p) =>
                write!(f, "Fetch action for package {} not supported", p.as_str()),
            Reqwest(e) =>
                write!(f, "{}", e),
            ScraperError =>
                write!(f, "Scraper error"), // TODO: more info
            ZipUrlNoFilename(s) =>
                write!(f, "No filename found for {}", s),
            FailedToDownload { url, path, status } =>
                write!(f, "Failed to download {} to {}. Got status {}", url, path.to_string_lossy(), status),
            InvalidYear(y) => 
                write!(f, "Invalid year (format is yyyy or yy): {}", y),
            InvalidMonth(m) =>
                write!(f, "Invalid month (format is mm): {}", m),
            ManageError(e) =>
                write!(f, "Manage error:\n{}", e),
        }
    }
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

fn write_underlined_string_record(record: &csv::StringRecord, idx: usize, f: &mut Formatter<'_>) -> fmt::Result {
    let record_str = record.iter().collect::<Vec<_>>().join(",");
    let underline = record.range(idx)
        .map(|r| {
            let pre: String = [' '].repeat(r.start + idx).into_iter().collect();
            let line: String = ['^'].repeat(r.end - r.start).into_iter().collect();
            format!("{}{}", pre, line.red())
        })
        .unwrap_or_else(|| {
            let pre: String = [' '].repeat(record_str.len()).into_iter().collect();
            let line: String = ['^'].repeat(3).into_iter().collect();
            format!("{}{}", pre, line.red())
        });
    write!(f, "\n\n\t{}\n", record_str)?;
    write!(f, "\t{}\n\n", underline)
}

impl Display for BadPayloadDetails {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_underlined_string_record(&self.record, self.idx, f)?;
        match &self.expected {
            Some(exp) => write!(f, "Expected one of [{}]\n", exp.into_iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(", ")),
            None => Ok(())
        }
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

impl<E: Display> Display for ParseErrorDetails<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_underlined_string_record(&self.record, self.idx, f)?;
        write!(f, "{}", self.error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_payload_details_display() {
        let record = csv::StringRecord::from(vec!["one", "two", "three"]);
        let details = BadPayloadDetails::new(record.clone()).at_index(1);
        let s = format!("{}", details);
        assert_eq!(
            s,
            format!("\n\n\tone,two,three\n\t    {}\n\n", "^^^".red())
        );
        let details = BadPayloadDetails::new(record.clone()).at_index(2);
        let s = format!("{}", details);
        assert_eq!(
            s,
            format!("\n\n\tone,two,three\n\t        {}\n\n", "^^^^^".red())
        );
        let details = BadPayloadDetails::new(record.clone()).at_index(2).expected_one_of(vec!["four".to_string(), "five".to_string()]);
        let s = format!("{}", details);
        assert_eq!(
            s,
            format!("\n\n\tone,two,three\n\t        {}\n\nExpected one of [\"four\", \"five\"]\n", "^^^^^".red())
        );
        let details = BadPayloadDetails::new(record.clone()).at_index(8);
        let s = format!("{}", details);
        assert_eq!(
            s,
            format!("\n\n\tone,two,three\n\t             {}\n\n", "^^^".red())
        );
        let details = BadPayloadDetails::new(record).at_index(8).expected_one_of(vec!["four".to_string(), "five".to_string()]);
        let s = format!("{}", details);
        assert_eq!(
            s,
            format!("\n\n\tone,two,three\n\t             {}\n\nExpected one of [\"four\", \"five\"]\n", "^^^".red())
        );
    }
}

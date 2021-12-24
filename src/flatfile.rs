use chrono::naive::{NaiveDate, NaiveTime, NaiveDateTime};
use crate::error::{
    Error,
    BadPayloadDetails,
    ParseErrorDetails,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FlatFile(Vec<Record>);

impl FlatFile {
    pub fn read_csv<R: std::io::Read>(rdr: csv::Reader<R>) -> Result<FlatFile, Error> {
        let records = rdr.into_records()
            .map(|r| r.map_err(Error::Csv).and_then(Record::from_csv_record))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(FlatFile(records))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Record {
    Comment(CommentRecord),
    Information(InformationRecord),
    Data(DataRecord)
}

impl Record {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        match record.get(0) {
            Some("C") => CommentRecord::from_csv_record(record).map(Record::Comment),
            Some("I") => InformationRecord::from_csv_record(record).map(Record::Information),
            Some("D") => DataRecord::from_csv_record(record).map(Record::Data),
            Some(_) => Err(
                Error::UnrecognizedPayload(
                    BadPayloadDetails::new(record)
                        .at_index(0)
                        .expected_one_of(
                            vec![
                                "C".to_string(), 
                                "I".to_string(), 
                                "D".to_string()
                            ]
                        )
                )
            ),
            None => Err(
                Error::PayloadMissingEntry(
                    BadPayloadDetails::new(record)
                        .at_index(0)
                        .expected_one_of(
                            vec![
                                "C".to_string(), 
                                "I".to_string(), 
                                "D".to_string()
                            ]
                        )
                )
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommentRecord {
    EMMS(CommentRecordEMMS),
    BUT(CommentRecordBUT),
    EOR(CommentRecordEndOfReport),
}

impl CommentRecord {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        match record.get(1) {
            Some("END OF REPORT") => CommentRecordEndOfReport::from_csv_record(record).map(CommentRecord::EOR),
            Some(_) => match record.get(2) {
                Some(r) if BlindUpdateReportId::from_str(r).is_some() => CommentRecordBUT::from_csv_record(record)
                    .map(CommentRecord::BUT),
                Some(r) if BlindUpdateReportId::from_str(r).is_none() => CommentRecordEMMS::from_csv_record(record)
                    .map(CommentRecord::EMMS),
                Some(_) => Err(
                    Error::UnrecognizedPayload(
                        BadPayloadDetails::new(record).at_index(2)
                    )
                ),
                None => Err(
                    Error::PayloadMissingEntry(
                        BadPayloadDetails::new(record).at_index(2)
                    )
                )
            },
            None => Err(
                Error::PayloadMissingEntry(
                    BadPayloadDetails::new(record)
                        .at_index(1)
                )
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlindUpdateReportId {
    BlindUpdateSubmission,
    BlindUpdateResponse,
}

impl BlindUpdateReportId {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "BLIND_UPDATE_SUBMISSION" => Some(BlindUpdateReportId::BlindUpdateSubmission),
            "BLIND_UPDATE_RESPONSE" => Some(BlindUpdateReportId::BlindUpdateResponse),
            _ => None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileId(String);

impl FileId {
    pub fn from_str(s: &str) -> Self {
        FileId(s.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentRecordEMMS {
    system: String,
    report_id: FileId,
    from: String,
    to: String,
    publish_date: NaiveDate,
    publish_time: NaiveTime,
    specific_payload_information: [String; 3],
}

impl CommentRecordEMMS {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        let system = record.get(1).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(1)))?;
        let report_id = record.get(2).map(FileId::from_str)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(2)))?;
        let from = record.get(3).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(3)))?;
        let to = record.get(4).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(4)))?;
        let publish_date = record.get(5)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(5)))
            .and_then(|s| NaiveDate::parse_from_str(s, "%Y/%m/%d")
                      .map_err(|e| Error::ParseDateError(ParseErrorDetails::new(record.clone(), 5, e))))?;
        let publish_time = record.get(6)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(6)))
            .and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S")
                      .map_err(|e| Error::ParseTimeError(ParseErrorDetails::new(record.clone(), 6, e))))?;
        let payload_info_1 = record.get(7)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(7)))?;
        let payload_info_2 = record.get(8)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(8)))?;
        let payload_info_3 = record.get(9)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(9)))?;
        let specific_payload_information = [payload_info_1, payload_info_2, payload_info_3];
        Ok(CommentRecordEMMS { system, report_id, from, to, publish_date, publish_time, specific_payload_information })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentRecordBUT {
    system: String,
    report_id: BlindUpdateReportId,
    from: String,
    to: String,
    publish_date: NaiveDate,
    publish_time: NaiveTime,
    market: String,
    payload_id: String,
    payload_response_id: String
}

impl CommentRecordBUT {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        let system = record.get(1).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(1)))?;
        let report_id = record.get(2).and_then(BlindUpdateReportId::from_str)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(2)))?;
        let from = record.get(3).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(3)))?;
        let to = record.get(4).map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(4)))?;
        let publish_date = record.get(5)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(5)))
            .and_then(|s| NaiveDate::parse_from_str(s, "%Y/%m/%d")
                      .map_err(|e| Error::ParseDateError(ParseErrorDetails::new(record.clone(), 5, e))))?;
        let publish_time = record.get(6)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(6)))
            .and_then(|s| NaiveTime::parse_from_str(s, "%H:%M:%S")
                      .map_err(|e| Error::ParseTimeError(ParseErrorDetails::new(record.clone(), 6, e))))?;
        let market = record.get(10)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(10)))?;
        let payload_id = record.get(11)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(11)))?;
        let payload_response_id = record.get(12)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(12)))?;
        Ok(CommentRecordBUT { system, report_id, from, to, publish_date, publish_time, market, payload_id, 
            payload_response_id })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentRecordEndOfReport {
    count_of_records: u32
}

impl CommentRecordEndOfReport {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        let count_of_records = record.get(2)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(2)))
            .and_then(|s| s.parse()
                      .map_err(|e| Error::ParseIntError(ParseErrorDetails::new(record.clone(), 2, e))))?;
        Ok(CommentRecordEndOfReport { count_of_records })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InformationRecord {
    report_type: String,
    report_subtype: String,
    report_version: u32,
    column_headers: Vec<String>,
}

impl InformationRecord {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        let report_type = record.get(1)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(1)))?;
        let report_subtype = record.get(2)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(2)))?;
        let report_version = record.get(3)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(3)))
            .and_then(|s| s.parse()
                      .map_err(|e| Error::ParseIntError(ParseErrorDetails::new(record.clone(), 3, e))))?;
        let column_headers = record.iter()
            .skip(4)
            .map(ToString::to_string)
            .collect();
        Ok(InformationRecord { report_type, report_subtype, report_version, column_headers })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataRecord {
    report_type: String,
    report_subtype: String,
    report_version: u32,
    data: Vec<DataValue>
}

impl DataRecord {
    pub fn from_csv_record(record: csv::StringRecord) -> Result<Self, Error> {
        let report_type = record.get(1)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(1)))?;
        let report_subtype = record.get(2)
            .map(ToString::to_string)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(2)))?;
        let report_version = record.get(3)
            .ok_or(Error::PayloadMissingEntry(BadPayloadDetails::new(record.clone()).at_index(3)))
            .and_then(|s| s.parse()
                      .map_err(|e| Error::ParseIntError(ParseErrorDetails::new(record.clone(), 3, e))))?;
        let data = record.iter()
            .skip(4)
            .map(DataValue::from_str)
            .collect();
        Ok(DataRecord { report_type, report_subtype, report_version, data })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Integer(i64),
    Float(f64),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    String(String)
}

impl DataValue {
    fn from_str(s: &str) -> Self {
        if let Ok(i) = s.parse::<i64>() {
            return DataValue::Integer(i)
        }
        if let Ok(f) = s.parse::<f64>() {
            return DataValue::Float(f)
        }
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y/%m/%d") {
            return DataValue::Date(d)
        }
        if let Ok(t) = NaiveTime::parse_from_str(s, "%H:%M:%S") {
            return DataValue::Time(t)
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S")
            .or(NaiveDateTime::parse_from_str(s, "%Y/%m/%d %H:%M")) {
            return DataValue::DateTime(dt)
        }
        DataValue::String(s.to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emms_comment_record() {
        let record = csv::StringRecord::from(vec!["C", "NEMP.WORLD", "BIDMOVE_SUMMARY", "AEMO", "PUBLIC", "2021/04/01", "04:43:39", 
                                             "339145123", "BIDMOVE_SUMMARY", "339145118"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Comment(
            CommentRecord::EMMS(
                CommentRecordEMMS {
                    system: "NEMP.WORLD".to_string(),
                    report_id: FileId("BIDMOVE_SUMMARY".to_string()),
                    from: "AEMO".to_string(),
                    to: "PUBLIC".to_string(),
                    publish_date: chrono::naive::NaiveDate::from_ymd(2021, 4, 1),
                    publish_time: chrono::naive::NaiveTime::from_hms(4, 43, 39),
                    specific_payload_information: ["339145123".to_string(), "BIDMOVE_SUMMARY".to_string(), "339145118".to_string()]
                }
            )
        );
        assert_eq!(parsed, expected);
    }
    
    #[test]
    fn but_comment_record() {
        let record = csv::StringRecord::from(vec!["C", "PRODUCTION", "BLIND_UPDATE_SUBMISSION", "PARTICIPANTID", "NEMMCO", "2021/09/03",
                                             "22:04:05", "", "", "", "NEM", "123ABC-002", "324-BB321"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Comment(
            CommentRecord::BUT(
                CommentRecordBUT {
                    system: "PRODUCTION".to_string(),
                    report_id: BlindUpdateReportId::BlindUpdateSubmission,
                    from: "PARTICIPANTID".to_string(),
                    to: "NEMMCO".to_string(),
                    publish_date: chrono::naive::NaiveDate::from_ymd(2021, 9, 3),
                    publish_time: chrono::naive::NaiveTime::from_hms(22, 4, 5),
                    market: "NEM".to_string(),
                    payload_id: "123ABC-002".to_string(),
                    payload_response_id: "324-BB321".to_string(),
                }
            )
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn eor_comment_record() {
        let record = csv::StringRecord::from(vec!["C", "END OF REPORT", "45917"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Comment(
            CommentRecord::EOR(
                CommentRecordEndOfReport {
                    count_of_records: 45917
                }
            )
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn emms_info_record() {
        let record = csv::StringRecord::from(vec!["I", "BID", "BIDDAYOFFER_D", "2", "SETTLEMENTDATE", "DUID", "BIDTYPE", "BIDSETTLEMENTDATE", "OFFERDATE", 
                                                  "VERSIONNO", "PARTICIPANTID", "DAILYENERGYCONSTRAINT", "REBIDEXPLANATION", "PRICEBAND1", "PRICEBAND2"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Information(
            InformationRecord {
                report_type: "BID".to_string(),
                report_subtype: "BIDDAYOFFER_D".to_string(),
                report_version: 2,
                column_headers: vec!["SETTLEMENTDATE".to_string(),
                                     "DUID".to_string(),
                                     "BIDTYPE".to_string(),
                                     "BIDSETTLEMENTDATE".to_string(),
                                     "OFFERDATE".to_string(),
                                     "VERSIONNO".to_string(),
                                     "PARTICIPANTID".to_string(),
                                     "DAILYENERGYCONSTRAINT".to_string(),
                                     "REBIDEXPLANATION".to_string(),
                                     "PRICEBAND1".to_string(),
                                     "PRICEBAND2".to_string()]
            }
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn but_info_record() {
        let record = csv::StringRecord::from(vec!["I", "BUS", "METER_REGISTER", "1", "NMI", "METER_SERIAL", "FIELDID", "VALUE"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Information(
            InformationRecord {
                report_type: "BUS".to_string(),
                report_subtype: "METER_REGISTER".to_string(),
                report_version: 1,
                column_headers: vec!["NMI".to_string(),
                                     "METER_SERIAL".to_string(),
                                     "FIELDID".to_string(),
                                     "VALUE".to_string()]
            }
        );
        assert_eq!(parsed, expected);
    }

    #[test]
    fn data_record() {
        let record = csv::StringRecord::from(vec!["D", "BID", "BIDDAYOFFER_D", "2", "2021/03/31 00:00", "DUID1", "ENERGY", "2021/03/31 00:00", 
                                                  "2021/03/30 12:19", "1", "PARTICIPANTID1", "241", "1054 F PB1 & PB10 LOSS FACTOR"]);
        let parsed = Record::from_csv_record(record).unwrap();
        let expected = Record::Data(
            DataRecord {
                report_type: "BID".to_string(),
                report_subtype: "BIDDAYOFFER_D".to_string(),
                report_version: 2,
                data: vec![DataValue::DateTime(NaiveDate::from_ymd(2021, 3, 31).and_hms(0, 0, 0)),
                                     DataValue::String("DUID1".to_string()),
                                     DataValue::String("ENERGY".to_string()),
                                     DataValue::DateTime(NaiveDate::from_ymd(2021, 3, 31).and_hms(0, 0, 0)),
                                     DataValue::DateTime(NaiveDate::from_ymd(2021, 3, 30).and_hms(12, 19, 0)),
                                     DataValue::Integer(1),
                                     DataValue::String("PARTICIPANTID1".to_string()),
                                     DataValue::Integer(241),
                                     DataValue::String("1054 F PB1 & PB10 LOSS FACTOR".to_string())]
            }
        );
        assert_eq!(parsed, expected);
    }
}

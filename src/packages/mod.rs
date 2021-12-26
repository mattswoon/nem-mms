pub mod schema;
use crate::{
    error::Error,
    flatfile::{
        FlatFile,
        Record,
        InformationRecord,
    }
};
use arrow::{
    array::{
        StringBuilder,
        PrimitiveBuilder,
    },
    datatypes::{
        Float64Type,
        TimestampSecondType,
    },
    record_batch::RecordBatch,
};
use parquet::{
    file::properties::WriterProperties,
    arrow::arrow_writer::ArrowWriter,
};
use std::{
    fs::OpenOptions,
    ffi::{OsStr, OsString},
    path::Path,
    sync::Arc,
    collections::HashMap,
};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Package {
    DispatchUnitScada
}

impl Package {
    pub fn as_str(&self) -> &'_ str {
        use Package::*;
        match self {
            DispatchUnitScada => "DISPATCH_UNIT_SCADA",
        }
    }

    pub fn from_information_record(record: &InformationRecord) -> Option<Self> {
        use Package::*;
        match (record.report_type.as_str(), record.report_subtype.as_str()) {
            ("DISPATCH", "UNIT_SCADA") => Some(DispatchUnitScada),
            _ => None
        }
    }

    pub fn schema(&self) -> &'static arrow::datatypes::Schema {
        use Package::*;
        match self {
            DispatchUnitScada => &schema::DISPATCH_UNIT_SCADA
        }
    }

    pub fn to_arrow(&self, flatfile: FlatFile) -> Result<RecordBatch, Error> {
        use Package::*;
        match self {
            DispatchUnitScada => {
                let len = flatfile.len();
                let mut duid_arr = StringBuilder::new(len);
                let mut settlementdate_arr = PrimitiveBuilder::<TimestampSecondType>::new(len);
                let mut scadavalue_arr = PrimitiveBuilder::<Float64Type>::new(len);
                let mut column_headers: HashMap<String, usize> = HashMap::new();
                for record in flatfile.records() {
                    match record {
                        Record::Information(record) => {
                            for (i, colh) in record.column_headers.iter().enumerate() {
                                column_headers.insert(colh.clone(), i);
                            }
                        },
                        Record::Data(record) => {
                            let duid = column_headers.get("DUID")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_string())
                                .ok_or(Error::MissingColumnHeader("DUID".to_string()))?;
                            let settlementdate = column_headers.get("SETTLEMENTDATE")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_datetime())
                                .ok_or(Error::MissingColumnHeader("SETTLEMENTDATE".to_string()))?
                                .timestamp(); 
                            let scadavalue = column_headers.get("SCADAVALUE")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_f64())
                                .ok_or(Error::MissingColumnHeader("SCADAVALUE".to_string()))?;
                            duid_arr.append_value(duid)
                                .map_err(Error::Arrow)?;
                            settlementdate_arr.append_value(settlementdate)
                                .map_err(Error::Arrow)?;
                            scadavalue_arr.append_value(scadavalue)
                                .map_err(Error::Arrow)?;
                        },
                        _ => {}
                    }
                }
                RecordBatch::try_new(
                    Arc::new(self.schema().clone()),
                    vec![
                        Arc::new(duid_arr.finish()),
                        Arc::new(settlementdate_arr.finish()),
                        Arc::new(scadavalue_arr.finish())
                    ]
                ).map_err(Error::Arrow)
            }
        }
    }

    pub fn to_parquet<P: AsRef<Path>>(&self, flatfiles: Vec<FlatFile>, path: P) -> Result<(), Error> {
        let schema = Arc::new(self.schema().clone());
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(Error::Io)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))
            .map_err(Error::Parquet)?;
        for flatfile in flatfiles {
            let batch = self.to_arrow(flatfile)?;
            writer.write(&batch).map_err(Error::Parquet)?;
        }
        writer.close().map_err(Error::Parquet)?;
        Ok(())
    }
}

pub fn to_parquet<P: AsRef<Path>>(flatfiles: Vec<FlatFile>, path: P) -> Result<(), Error> {
    let mut reports: HashMap<Package, Vec<FlatFile>> = HashMap::new();
    for flatfile in flatfiles {
        if let Some(package) = flatfile.information_record()
            .and_then(|i| Package::from_information_record(i)) {
            if let Some(v) = reports.get_mut(&package) {
                (*v).push(flatfile);
            } else {
                reports.insert(package, vec![flatfile]);
            }
        }
    }
    if reports.len() <= 1 {
        for (p, fs) in reports.into_iter() {
            p.to_parquet(fs, &path)?;
        }
    } else {
        for (p, fs) in reports.into_iter() {
            let ppath = if path.as_ref().is_dir() {
                path.as_ref().join(format!("{}", p.as_str())).with_extension("parquet")
            } else {
                let filename = path.as_ref().file_stem()
                    .map(|s| vec![s, &OsStr::new(&format!("_{}", p.as_str()))].into_iter().collect::<OsString>())
                    .ok_or(Error::InvalidFilename(path.as_ref().to_path_buf()))?;
                Path::new(&filename).with_extension("parquet")
            };
            p.to_parquet(fs, ppath)?;
        }
    }
    Ok(())
}

pub mod schema;
use crate::flatfile::{
    FlatFile,
    Record,
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
    path::Path,
    sync::Arc,
    collections::HashMap,
};

#[derive(Debug)]
pub enum Error {
    Arrow(arrow::error::ArrowError),
    Parquet(parquet::errors::ParquetError),
    Io(std::io::Error),
}


#[derive(Debug)]
pub enum Package {
    DispatchUnitScada
}

impl Package {
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
                            for (i, colh) in record.column_headers.into_iter().enumerate() {
                                column_headers.insert(colh, i);
                            }
                        },
                        Record::Data(record) => {
                            let duid = column_headers.get("DUID")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_string())
                                .unwrap(); // TODO
                            let settlementdate = column_headers.get("SETTLEMENTDATE")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_datetime())
                                .unwrap() // TODO
                                .timestamp(); 
                            let scadavalue = column_headers.get("SCADAVALUE")
                                .and_then(|idx| record.data.get(*idx))
                                .and_then(|v| v.clone().as_f64())
                                .unwrap(); // TODO
                            duid_arr.append_value(duid)
                                .unwrap(); // TODO
                            settlementdate_arr.append_value(settlementdate)
                                .unwrap(); // TODO
                            scadavalue_arr.append_value(scadavalue)
                                .unwrap(); // TODO
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

    pub fn to_parquet<P: AsRef<Path>>(&self, flatfile: FlatFile, path: P) -> Result<(), Error> {
        let schema = Arc::new(self.schema().clone());
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(Error::Io)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))
            .map_err(Error::Parquet)?;
        let batch = self.to_arrow(flatfile)?;
        writer.write(&batch).map_err(Error::Parquet)?;
        writer.close().map_err(Error::Parquet)?;
        Ok(())
    }
}

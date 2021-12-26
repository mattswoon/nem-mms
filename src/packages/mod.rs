pub mod schema;
use crate::{
    error::Error,
    flatfile::{
        FlatFile,
        InformationRecord,
    }
};
use arrow::{
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
    DispatchUnitScada,
    DispatchNegativeResidue,
}

impl Package {
    pub fn as_str(&self) -> &'_ str {
        use Package::*;
        match self {
            DispatchUnitScada => "DISPATCH_UNIT_SCADA",
            DispatchNegativeResidue => "DISPATCH_NEGATIVE_RESIDUE",
        }
    }

    pub fn from_information_record(record: &InformationRecord) -> Option<Self> {
        use Package::*;
        match (record.report_type.as_str(), record.report_subtype.as_str()) {
            ("DISPATCH", "UNIT_SCADA") => Some(DispatchUnitScada),
            ("DISPATCH", "NEGATIVE_RESIDUE") => Some(DispatchNegativeResidue),
            _ => None
        }
    }

    pub fn schema(&self) -> &'static arrow::datatypes::Schema {
        use Package::*;
        match self {
            DispatchUnitScada => &schema::DISPATCH_UNIT_SCADA,
            DispatchNegativeResidue => &schema::DISPATCH_NEGATIVE_RESIDUE,
        }
    }

    pub fn to_arrow(&self, flatfile: FlatFile) -> Result<RecordBatch, Error> {
        flatfile.to_arrow(self.schema())
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
        flatfile.information_record()
            .and_then(|i| Package::from_information_record(i).or_else(|| {
                println!(
                    "Unrecognized package - skipping...\n\tReport type:\t{}\n\tSub-type:\t{}", 
                    i.report_type,
                    i.report_subtype
                );
                None
            }))
            .map(|p| {
                if let Some(v) = reports.get_mut(&p) {
                    (*v).push(flatfile);
                } else {
                    reports.insert(p, vec![flatfile]);
                }});
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

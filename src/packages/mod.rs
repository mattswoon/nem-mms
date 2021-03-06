pub mod schema;
pub mod fetch;

use crate::{
    error::Error,
    flatfile::{
        FlatFile,
        InformationRecord,
    },
    packages::fetch::{
        HistoricDataDownloader,
        NemwebScraper,
        Archive,
    },
};
use arrow::{
    record_batch::RecordBatch,
};
use colored::Colorize;
use parquet::{
    file::properties::WriterProperties,
    arrow::arrow_writer::ArrowWriter,
};
use prettytable::{
    Table, 
    row, 
    cell, 
    format::{
        FormatBuilder,
        LinePosition,
        LineSeparator
    },
};
use strum::IntoEnumIterator;
use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;
use std::{
    fs::OpenOptions,
    ffi::{OsStr, OsString},
    path::Path,
    sync::Arc,
    collections::HashMap,
    fmt::{Display, self},
};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, EnumIter, Serialize, Deserialize)]
pub enum Package {
    DispatchUnitScada,
    DispatchNegativeResidue,
    DispatchLocalPrice,
    DispatchPrice,
    RooftopPvActual,
    RooftopPvForecast,
}

impl Package {
    pub fn available_packages() -> Vec<&'static str> {
        Package::iter()
            .map(|p| p.as_str())
            .collect()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        use Package::*;
        match s {
             "DISPATCH_UNIT_SCADA"       => Some(DispatchUnitScada),
             "DISPATCH_NEGATIVE_RESIDUE" => Some(DispatchNegativeResidue),
             "DISPATCH_LOCAL_PRICE"      => Some(DispatchLocalPrice),
             "ROOFTOP_PV_ACTUAL"         => Some(RooftopPvActual),
             "ROOFTOP_PV_FORECAST"       => Some(RooftopPvForecast),
             "DISPATCHPRICE"             => Some(DispatchPrice),
             _ => None
        }
    }

    pub fn as_str(&self) -> &'static str {
        use Package::*;
        match self {
            DispatchUnitScada => "DISPATCH_UNIT_SCADA",
            DispatchNegativeResidue => "DISPATCH_NEGATIVE_RESIDUE",
            DispatchLocalPrice => "DISPATCH_LOCAL_PRICE",
            RooftopPvActual => "ROOFTOP_PV_ACTUAL",
            RooftopPvForecast => "ROOFTOP_PV_FORECAST",
            DispatchPrice => "DISPATCHPRICE",
        }
    }

    pub fn from_information_record(record: &InformationRecord) -> Option<Self> {
        use Package::*;
        match (record.report_type.as_str(), record.report_subtype.as_str()) {
            ("DISPATCH", "UNIT_SCADA") => Some(DispatchUnitScada),
            ("DISPATCH", "NEGATIVE_RESIDUE") => Some(DispatchNegativeResidue),
            ("DISPATCH", "LOCAL_PRICE") => Some(DispatchLocalPrice),
            ("ROOFTOP", "ACTUAL") => Some(RooftopPvActual),
            ("ROOFTOP", "FORECAST") => Some(RooftopPvForecast),
            ("DISPATCH", "PRICE") => Some(DispatchPrice),
            _ => None
        }
    }

    pub fn schema(&self) -> &'static arrow::datatypes::Schema {
        use Package::*;
        match self {
            DispatchUnitScada => &schema::DISPATCH_UNIT_SCADA,
            DispatchNegativeResidue => &schema::DISPATCH_NEGATIVE_RESIDUE,
            DispatchLocalPrice => &schema::DISPATCH_LOCAL_PRICE,
            RooftopPvActual => &schema::ROOFTOP_PV_ACTUAL,
            RooftopPvForecast => &schema::ROOFTOP_PV_FORECAST,
            DispatchPrice => &schema::DISPATCHPRICE,
        }
    }

    pub fn to_parquet<P: AsRef<Path>>(&self, batches: Vec<RecordBatch>, path: P) -> Result<(), Error> {
        let schema = Arc::new(self.schema().clone());
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(Error::Io)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))
            .map_err(Error::Parquet)?;
        for batch in batches {
            writer.write(&batch).map_err(Error::Parquet)?;
        }
        writer.close().map_err(Error::Parquet)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PackageInfo {
    name: String,
    schema: &'static arrow::datatypes::Schema,
    supports_fetch_current: bool,
    supports_fetch_archive: bool,
    supports_fetch_historic: bool,
}

impl PackageInfo {
    pub fn new(package: Package) -> Self {
        let name = package.as_str().to_string();
        let schema = package.schema();
        let supports_fetch_historic = HistoricDataDownloader::new(package).url().is_some();
        let supports_fetch_current = NemwebScraper::new(package, Archive::Current).url().is_some();
        let supports_fetch_archive = NemwebScraper::new(package, Archive::Archive).url().is_some();
        PackageInfo { name, schema, supports_fetch_current, supports_fetch_archive, supports_fetch_historic }
    }
}

impl Display for PackageInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = "    ";
        write!(f, "Pacakge name: {}\n", self.name)?;
        write!(f, "Supported fetch operations:\n")?;
        if self.supports_fetch_current {
            write!(f, "{}{}", indent, "??? Current\n".green())?;
        } else {
            write!(f, "{}{}", indent, "??? Current\n".red())?;
        }
        if self.supports_fetch_archive {
            write!(f, "{}{}", indent, "??? Archive\n".green())?;
        } else {
            write!(f, "{}{}", indent, "??? Archive\n".red())?;
        }
        if self.supports_fetch_historic {
            write!(f, "{}{}", indent, "??? Historic\n".green())?;
        } else {
            write!(f, "{}{}", indent, "??? Historic\n".red())?;
        }
        write!(f, "Schema: \n")?;
        let mut schema_table = Table::new();
        schema_table.set_format(FormatBuilder::new()
                                .borders(' ')
                                .column_separator(' ')
                                .separator(LinePosition::Title, LineSeparator::new('-', '-', '-', '-'))
                                .indent(indent.len())
                                .padding(0, 1)
                                .build());
        for field in self.schema.fields() {
            schema_table.add_row(
                row![
                    cell!(field.name()),
                    cell!(field.data_type()),
                    field.is_nullable().then(|| cell!("???".green())).unwrap_or(cell!("???".red()))
                ]
            );
        }
        schema_table.set_titles(row!["Name", "Data type", "Nullable"]);
        write!(f, "{}", schema_table)?;
        Ok(())
    }
}

pub fn to_parquet<P: AsRef<Path>>(flatfiles: Vec<FlatFile>, path: P) -> Result<(), Error> {
    let mut reports: HashMap<Package, Vec<RecordBatch>> = HashMap::new();
    for flatfile in flatfiles {
        for res in flatfile.iter().map(|t| t.to_arrow()) {
            match res {
                Err(e) => match e {
                    Error::UnrecognizedPackage { report_type, report_subtype } => 
                        // TODO: change this to a debug log, it's very noisy
                        println!("Unrecognized package ... skipping\n\tReport type: {}\n\tReport subtype: {}",
                                  report_type,
                                  report_subtype),
                    _ => return Err(e)
                },
                Ok((package, rb)) => {
                    if let Some(v) = reports.get_mut(&package) {
                        (*v).push(rb);
                    } else {
                        reports.insert(package, vec![rb]);
                    }
                }
            }
        }
    };
    if reports.len() <= 1 {
        for (p, bs) in reports.into_iter() {
            p.to_parquet(bs, &path)?;
        }
    } else {
        for (p, bs) in reports.into_iter() {
            let ppath = if path.as_ref().is_dir() {
                path.as_ref().join(format!("{}", p.as_str())).with_extension("parquet")
            } else {
                let filename = path.as_ref().file_stem()
                    .map(|s| vec![s, &OsStr::new(&format!("_{}", p.as_str()))].into_iter().collect::<OsString>())
                    .ok_or(Error::InvalidFilename(path.as_ref().to_path_buf()))?;
                Path::new(&filename).with_extension("parquet")
            };
            p.to_parquet(bs, ppath)?;
        }
    }
    Ok(())
}


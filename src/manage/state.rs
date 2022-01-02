use std::{
    path::{
        Path,
        PathBuf,
    },
    fs::{
        create_dir,
        read_to_string,
        write,
    },
    fmt::{Display, Formatter, self},
};
use chrono::NaiveDate;
use crate::{
    packages::{
        Package,
        fetch::{Month, Year},
    },
    manage::config::Config,
};

#[derive(Debug)]
pub enum Error {
    UnrecognizedFilename(PathBuf),
    Io(std::io::Error),
    TomlRead(toml::de::Error),
    TomlWrite(toml::ser::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            UnrecognizedFilename(fname) =>
                write!(f, "Can't parse data file name: {}", fname.to_string_lossy()),
            Io(e) =>
                write!(f, "{}", e),
            TomlRead(e) =>
                write!(f, "{}", e),
            TomlWrite(e) =>
                write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Filename {
    Historic(HistoricFilename),
    NonHistoric(NonHistoricFilename)
}

impl Filename {
    pub fn from_path_buf(path: PathBuf) -> Result<Self, Error> {
        let package = path.parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .and_then(|s| Package::from_str(s))
            .ok_or(Error::UnrecognizedFilename(path.clone()))?;
        let fname = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or(Error::UnrecognizedFilename(path.clone()))?;
        match fname.split("_").collect::<Vec<_>>()[..] {
            ["historic", date] => match date.split("-").collect::<Vec<_>>()[..] {
                [year, month] => Year::from_str(year)
                    .and_then(|y| Month::from_str(month).map(|m| (y, m)))
                    .map(|(year, month)| Filename::Historic(HistoricFilename { package, year, month })),
                _ => None,
            },
            ["nonhistoric", date, file_id] => NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
                .map(|report_date| Filename::NonHistoric(NonHistoricFilename { package, file_id: file_id.to_string(), report_date })),
            _ => None
        }.ok_or(Error::UnrecognizedFilename(path.clone()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HistoricFilename {
    package: Package,
    month: Month,
    year: Year
}

impl HistoricFilename {
    pub fn as_path_buf(&self) -> PathBuf {
        Path::new(self.package.as_str())
            .join(format!("historic_{}-{}.parquet", self.year.as_str(), self.month.as_str()))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonHistoricFilename {
    package: Package,
    file_id: String,
    report_date: NaiveDate,
}

impl NonHistoricFilename {
    pub fn as_path_buf(&self) -> PathBuf {
        Path::new(self.package.as_str())
            .join(format!("nonhistoric_{}_{}.parquet", self.report_date.format("%Y-%m-%d"), self.file_id))
    }
}

/// State of a MMS data depository
///
/// Files are kept in
///  - `[base]/`: all files
///  - `[base]/data/`: parsed parquet tables
///  - `[base]/.raw/`: raw downloaded files
///             
#[derive(Debug, Clone, PartialEq)]
pub struct DepositoryState {
    pub base: PathBuf,
    pub files: Vec<Filename>,
    pub config: Config,
}

impl DepositoryState {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut files = Vec::new();
        for entry in path.as_ref().join("data").read_dir().map_err(Error::Io)? {
            let entry = entry.map_err(Error::Io)?;
            let fname = Filename::from_path_buf(entry.path())?;
            files.push(fname);
        }
        let conifg_contents = read_to_string(path.as_ref().join("config.toml"))
            .map_err(Error::Io)?;
        let config = toml::from_str(&conifg_contents)
            .map_err(Error::TomlRead)?;
        Ok(DepositoryState { base: path.as_ref().to_path_buf(), files, config })
    }

    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        if !path.as_ref().exists() {
            create_dir(path.as_ref()).map_err(Error::Io)?;
        }
        create_dir(path.as_ref().join("data")).map_err(Error::Io)?;
        create_dir(path.as_ref().join(".raw")).map_err(Error::Io)?;
        let config = Config::init();
        let config_str = toml::ser::to_string_pretty(&config).map_err(Error::TomlWrite)?;
        write(path.as_ref().join("config.toml"), config_str.as_bytes())
            .map_err(Error::Io)?;
        Ok(DepositoryState {
            base: path.as_ref().to_path_buf(),
            files: Vec::new(),
            config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn historic_filename() {
        let f = HistoricFilename { 
            package: Package::DispatchUnitScada, 
            month: Month::Jan, 
            year: Year::from_str("2022").unwrap(),
        };
        let exp = "DISPATCH_UNIT_SCADA/historic_2022-01.parquet";
        assert_eq!(
            f.as_path_buf(),
            PathBuf::from(exp)
        );

        assert_eq!(
            Filename::Historic(f.clone()),
            Filename::from_path_buf(f.as_path_buf()).unwrap()
        );
    }
    
    #[test]
    fn non_historic_filename() {
        let f = NonHistoricFilename { 
            package: Package::DispatchUnitScada, 
            file_id: "0003".to_string(),
            report_date: NaiveDate::from_ymd(2020, 01, 01)
        };
        let exp = "DISPATCH_UNIT_SCADA/nonhistoric_2020-01-01_0003.parquet";
        assert_eq!(
            f.as_path_buf(),
            PathBuf::from(exp)
        );
        
        assert_eq!(
            Filename::NonHistoric(f.clone()),
            Filename::from_path_buf(f.as_path_buf()).unwrap()
        );
    }
}

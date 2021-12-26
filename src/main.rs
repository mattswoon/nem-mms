use clap::{Arg, App, SubCommand, crate_version};
use csv::ReaderBuilder;
use zip::read::ZipArchive;
use nem_mms::{
    flatfile::FlatFile,
    packages,
    zip::read_zip,
};
use std::{
    fs::OpenOptions,
};


fn main() {
    let matches = App::new("nem-mms")
        .version(crate_version!())
        .author("mattswoon")
        .about("Fetch and parse AEMO's MMS data into parquet")
        .subcommand(SubCommand::with_name("parse")
                    .about("Parse a flat file csv or zip")
                    .arg(Arg::with_name("FILE")
                         .required(true)
                         .takes_value(true)
                         .index(1)))
        .get_matches();

    match matches.subcommand() {
        ("parse", Some(sub_m)) => {
            let fname = sub_m.value_of("FILE")
                .expect("Expected a file");
            let fname = std::path::Path::new(&fname);
            let parsed_flatfiles = match fname.extension().map(|s| s.to_str()).flatten() {
                Some("csv") | Some("CSV") => {
                    let rdr = ReaderBuilder::new()
                        .flexible(true)
                        .has_headers(false)
                        .from_path(fname)
                        .expect("Couldn't make reader");
                    let flatfile = FlatFile::read_csv(rdr).unwrap();
                    vec![flatfile]
                },
                Some("zip") | Some("ZIP") => {
                    let file = OpenOptions::new()
                        .read(true)
                        .open(fname)
                        .unwrap();
                    let archive = ZipArchive::new(file).unwrap();
                    read_zip(archive).unwrap()
                },
                _ => vec![]
            };
            let out = std::path::Path::new(&fname)
                .with_extension("parquet");
            packages::to_parquet(parsed_flatfiles, out).unwrap();
        },
        _ => {}
    }
}

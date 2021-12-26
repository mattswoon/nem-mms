use std::io::{
    Read, 
    Seek,
    Cursor,
};
use zip::read::ZipArchive;
use crate::{
    flatfile::FlatFile,
    error::Error
};


pub fn read_zip<R: Read + Seek>(mut archive: ZipArchive<R>) -> Result<Vec<FlatFile>, Error> {
    let mut out: Vec<FlatFile> = Vec::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(Error::Zip)?;
        let fname = file.name().to_string();
        match fname.split('.').collect::<Vec<_>>()[..] {
            [.., "zip"] | [.., "ZIP"] => {
                let mut buff = Cursor::new(Vec::new());
                file.read_to_end(buff.get_mut()).map_err(Error::Io)?;
                let sub_archive = ZipArchive::new(buff).map_err(Error::Zip)?;
                let mut to_append = read_zip(sub_archive)?;
                out.append(&mut to_append);
            },
            [.., "csv"] | [.., "CSV"] => {
                let mut buff = Cursor::new(Vec::new());
                file.read_to_end(buff.get_mut()).map_err(Error::Io)?;
                let rdr = csv::ReaderBuilder::new()
                    .flexible(true)
                    .has_headers(false)
                    .from_reader(buff);
                let to_push = FlatFile::read_csv(rdr)?;
                out.push(to_push);
            },
            _ => {}
        }
    }
    Ok(out)
}

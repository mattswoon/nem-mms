use scraper::{Html, Selector};
use reqwest::blocking::get;
use std::{
    io::{Write, stdout},
    path::Path,
    fs::OpenOptions,
};
use crate::{
    packages::Package,
    error::Error,
};

fn package_url_part(package: &Package) -> Option<&'_ str> {
    use Package::*;
    match package {
        DispatchUnitScada => Some("Dispatch_SCADA"),
        DispatchNegativeResidue => Some("DISPATCH_NEGATIVE_RESIDUE"),
        DispatchLocalPrice => None,
    }
}

#[derive(Debug)]
pub struct HistoricDataDownloader {
    pub package: Package,
    pub year: String,
    pub month: String,
}

impl HistoricDataDownloader {
    pub fn new(package: Package) -> Self {
        HistoricDataDownloader {
            package,
            year: "2009".to_string(),
            month: "07".to_string(),
        }
    }

    pub fn url(&self) -> Option<String> {
        use Package::*;
        let filename = match &self.package {
            DispatchUnitScada => Some(format!("PUBLIC_DVD_DISPATCH_UNIT_SCADA_{}{}010000.zip", &self.year, &self.month)),
            DispatchNegativeResidue => None,
            DispatchLocalPrice => None
        }?;
        let url = format!("Data_Archive/Wholesale_Electricity/MMSDM/{}/MMSDM_{}_{}/MMSDM_Historical_Data_SQLLoader/DATA/{}", &self.year, &self.year, &self.month, filename);
        Some(url)
    }

    pub fn download<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let url = self.url()
            .map(|u| format!("{}/{}", BASE_URL, u))
            .ok_or(Error::UnsupportedFetchReport(self.package.clone()))?;
        print!("Fetching {} ... ", &url);
        stdout().flush().map_err(Error::Io)?;
        let fname = url.split('/')
            .last()
            .ok_or(Error::ZipUrlNoFilename(url.to_string()))?;
        let path = path.as_ref().join(fname);
        download_file(&url, path)
            .map(|b| print!(" success ({} bytes)\n", b))
            .or_else(|e| {
                print!(" failed\n");
                match e {
                    Error::FailedToDownload { url, path, status } => {
                        eprintln!("Failed to download {} to {:#?}. Got status {}", url, path.as_os_str(), status);
                        Ok(())
                    },
                    _ => Err(e)
            }})?;
        Ok(())
    }
}


#[derive(Debug)]
pub enum Archive  {
    Current,
    Archive
}

impl Archive {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "current" => Some(Archive::Current),
            "archive" => Some(Archive::Archive),
            _ => None
        }
    }

    pub fn url_part(&self) -> &'_ str {
        match self {
            Archive::Current => "Current",
            Archive::Archive => "Archive",
        }
    }
}

const BASE_URL: &'static str = "https://www.nemweb.com.au";

#[derive(Debug)]
pub struct NemwebScraper {
    pub package: Package,
    pub archive: Archive
}

impl NemwebScraper {
    pub fn new(package: Package, archive: Archive) -> Self {
        NemwebScraper { package, archive }
    }

    pub fn url(&self) -> Option<String> {
        package_url_part(&self.package)
            .map(|p| format!("Reports/{}/{}", self.archive.url_part(), p))
    }

    fn fetch_html_document(&self) -> Result<Html, Error> {
        let url = self.url()
            .map(|u| format!("{}/{}", BASE_URL, u))
            .ok_or(Error::UnsupportedFetchReport(self.package.clone()))?;
        let document = get(url)
            .map_err(Error::Reqwest)?
            .text()
            .map_err(Error::Reqwest)?;
        let document = Html::parse_document(&document);
        Ok(document)
    }

    fn find_all_urls<'a>(&'a self, document: &'a Html) -> Result<Vec<&'a str>, Error> {
        let selector = Selector::parse("a")
            .map_err(|_| Error::ScraperError)?;
        let zip_links: Vec<&str> = document.select(&selector)
            .filter_map(|eref| eref.value()
                 .attr("href")
                 .and_then(|h| if h.ends_with(".zip") { Some(h) } else { None }))
            .collect();
        Ok(zip_links)
    }

    pub fn download_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let document = self.fetch_html_document()?;
        let zip_urls = self.find_all_urls(&document)?;
        for url in zip_urls {
            print!("Fetching {} ... ", &url);
            stdout().flush().map_err(Error::Io)?;
            let fname = url.split('/').last()
                .ok_or(Error::ZipUrlNoFilename(url.to_string()))?;
            let target_path = path.as_ref().join(fname);
            let full_url = format!("{}{}", BASE_URL, url);
            download_file(&full_url, target_path)
                .map(|b| print!(" success ({} bytes)\n", b))
                .or_else(|e| {
                    print!(" failed\n");
                    match e {
                        Error::FailedToDownload { url, path, status } => {
                            eprintln!("Failed to download {} to {:#?}. Got status {}", url, path.as_os_str(), status);
                            Ok(())
                        },
                        _ => Err(e)
                }})?;
        }
        Ok(())
    }
}

fn download_file<P: AsRef<Path>>(url: &str, path: P) -> Result<u64, Error> {
    let mut response = get(url)
        .map_err(Error::Reqwest)?;
    if response.status().is_success() {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(Error::Io)?;
        response.copy_to(&mut file)
            .map_err(Error::Reqwest)
    } else {
        Err(Error::FailedToDownload { 
            url: url.to_string(), 
            path: path.as_ref().to_path_buf(),
            status: response.status(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_all_urls() {
        let html = r#"
<html>
    <head>
        <title>nemweb.com.au - /Reports/Current/Dispatch_SCADA/</title>
    </head>
    <body>
        <H1>nemweb.com.au - /Reports/Current/Dispatch_SCADA/</H1>
        <hr>

        <pre>
            <A HREF="/Reports/Current/">[To Parent Directory]</A><br><br>
            Friday, May 12, 2017 10:56 AM        &lt;dir&gt; <A HREF="/Reports/Current/Dispatch_SCADA/DUPLICATE/">DUPLICATE</A><br>
            Saturday, December 25, 2021 10:40 AM         3157 <A HREF="/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251045_0000000354978413.zip">PUBLIC_DISPATCHSCADA_202112251045_0000000354978413.zip</A><br>  
            Saturday, December 25, 2021 10:46 AM         3128 <A HREF="/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251050_0000000354978611.zip">PUBLIC_DISPATCHSCADA_202112251050_0000000354978611.zip</A><br>  
            Saturday, December 25, 2021 10:50 AM         3145 <A HREF="/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251055_0000000354978803.zip">PUBLIC_DISPATCHSCADA_202112251055_0000000354978803.zip</A><br>  
            Saturday, December 25, 2021 10:55 AM         3144 <A HREF="/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251100_0000000354979009.zip">PUBLIC_DISPATCHSCADA_202112251100_0000000354979009.zip</A><br>
        </pre>
        <hr>
    </body>
</html>
        "#;
        let document = Html::parse_document(html);
        let nemweb_scraper = NemwebScraper { package: Package::DispatchUnitScada, archive: Archive::Current };
        let zip_urls = nemweb_scraper.find_all_urls(&document).unwrap();
        dbg!(&zip_urls);
        let expected = vec![
            "/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251045_0000000354978413.zip",
            "/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251050_0000000354978611.zip",
            "/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251055_0000000354978803.zip",
            "/Reports/Current/Dispatch_SCADA/PUBLIC_DISPATCHSCADA_202112251100_0000000354979009.zip"
        ];
        assert_eq!(zip_urls, expected)
    }
}

use scraper::{Html, Selector};
use reqwest::blocking::get;
use std::{
    io::Write,
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

const BASE_URL: &'static str = "https://www.nemweb.com.au/Reports";

#[derive(Debug)]
pub struct NemwebScraper {
    pub package: Package,
    pub archive: Archive
}

impl NemwebScraper {
    fn fetch_html_document(&self) -> Result<Html, Error> {
        let url = package_url_part(&self.package)
            .ok_or(Error::UnsupportedFetchReport(self.package.clone()))
            .map(|p| format!("{}/{}/{}", BASE_URL, self.archive.url_part(), p))?;
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
            println!("Fetching {}", &url);
            let fname = url.split('/').last()
                .ok_or(Error::ZipUrlNoFilename(url.to_string()))?;
            let target_path = path.as_ref().join(fname);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(target_path)
                .map_err(Error::Io)?;
            let full_url = format!("{}{}", BASE_URL, url);
            download_file(&full_url, &mut file)?;
        }
        Ok(())
    }
}

fn download_file<W: Write>(url: &str, wtr: &mut W) -> Result<u64, Error> {
    get(url)
        .map_err(Error::Reqwest)?
        .copy_to(wtr)
        .map_err(Error::Reqwest)
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

use std::path::Path;

use serde::Deserialize;
pub use derive_builder::Builder;

use crate::DataFrame;


#[derive(Builder, Default)]
pub struct Reader
{
    path: String,

    #[builder(default = r#"b','"#)]
    delimiter: u8,

    #[builder(default = r#"csv::Terminator::Any(b'\n')"#)]
    terminator: csv::Terminator,

    #[builder(default = r#"b'"'"#)]
    quote: u8,

    #[builder(default = r#"true"#)]
    has_headers: bool,
}

impl Reader {

    /// Read a CSV file into a [`DataFrame`] where each column represents a Series
    /// supports automatic decompression of gzipped files if they end with `.gz`
    pub fn read<T>(&self) -> Result<DataFrame<T>, Box<std::error::Error>>
        where for<'de> T: Deserialize<'de>
    {
        use flate2::read::GzDecoder;
        use std::fs::File;
        use std::io::prelude::*;

        let p = Path::new(&self.path);
        let file_reader: Box<Read> = if self.path.to_string().to_lowercase().ends_with(".gz") {
            // Return a Gzip reader
            Box::new(GzDecoder::new(File::open(p)?))
        } else {
            // Return plain file reader
            Box::new(File::open(p)?)
        };

        let mut reader = csv::ReaderBuilder::new()
            .quote(self.quote)
            .has_headers(self.has_headers)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .from_reader(file_reader);

        let mut df = DataFrame::new();

        for result in reader.deserialize() {
            let row: T = result?;
            df.push(row);
        }
        Ok(df)
    }
}
use polars::prelude::*;
use polars_io::prelude::*;
use std::fs::File;

pub fn read_articles(path: &str) -> polars::DataFrame {
    let data_frame = csv::CsvReader::from_path(path)?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    data_frame
}
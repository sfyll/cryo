use crate::{CollectError, Datatype, Partition, Query};
use std::{collections::HashMap, path::PathBuf};

/// Options for file output
#[derive(Clone, Debug)]
pub struct FileOutput {
    /// Path of directory where to save files
    pub output_dir: std::path::PathBuf,
    /// Prefix of file name
    pub prefix: String,
    /// Suffix to use at the end of file names
    pub suffix: Option<String>,
    /// Whether to overwrite existing files or skip them
    pub overwrite: bool,
    /// File format to used for output files
    pub format: FileFormat,
    /// Number of rows per parquet row group
    pub row_group_size: Option<usize>,
    /// Parquet statistics recording flag
    pub parquet_statistics: bool,
    /// Parquet compression options
    pub parquet_compression: polars::prelude::ParquetCompression,
}

impl FileOutput {
    /// get output file paths
    pub fn get_paths(
        &self,
        query: &Query,
        partition: &Partition,
    ) -> Result<HashMap<Datatype, PathBuf>, CollectError> {
        let mut paths = HashMap::new();
        for meta_datatype in query.datatypes.iter() {
            for datatype in meta_datatype.datatypes().into_iter() {
                paths.insert(datatype, self.get_path(query, partition, datatype)?);
            }
        }
        Ok(paths)
    }

    /// get output file path
    pub fn get_path(
        &self,
        query: &Query,
        partition: &Partition,
        datatype: Datatype,
    ) -> Result<PathBuf, CollectError> {
        let filename = format!(
            "{}__{}__{}.{}",
            self.prefix.clone(),
            datatype.name(),
            partition.label(&query.partitioned_by)?,
            self.format.as_str(),
        );
        Ok(std::path::Path::new(&self.output_dir).join(filename))
    }
}

/// File format
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FileFormat {
    /// Parquet file format
    Parquet,
    /// Csv file format
    Csv,
    /// Json file format
    Json,
}

impl FileFormat {
    /// convert FileFormat to str
    pub fn as_str(&self) -> &'static str {
        match *self {
            FileFormat::Parquet => "parquet",
            FileFormat::Csv => "csv",
            FileFormat::Json => "json",
        }
    }
}

/// Encoding for binary data in a column
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ColumnEncoding {
    /// Raw binary encoding
    Binary,
    /// Hex binary encoding
    Hex,
}

impl ColumnEncoding {
    /// convert ColumnEncoding to str
    pub fn as_str(&self) -> &'static str {
        match *self {
            ColumnEncoding::Binary => "binary",
            ColumnEncoding::Hex => "hex",
        }
    }
}

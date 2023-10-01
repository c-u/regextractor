use std::error::Error;

use crate::datatable::datatable_error::DataTableError;

#[derive(Debug)]
pub enum ExtractionError {
    DataTable(DataTableError),
    ReadError(std::io::Error),
}

impl std::fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for ExtractionError {}

impl From<DataTableError> for ExtractionError {
    fn from(value: DataTableError) -> Self {
        Self::DataTable(value)
    }
}

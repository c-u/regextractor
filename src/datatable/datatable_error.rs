use std::error::Error;

#[derive(Debug)]
pub enum DataTableError {
    InvalidColumnName,
    InvalidColumnCount,
    InvalidColumnIndex,
    InvalidCBaseDataIndex,
    InvalidCBaseDataName,
    InconsistentBuilderData,
    InconsistentContainerSize,
    DuplicateName,
}

impl std::fmt::Display for DataTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DataTableError {}

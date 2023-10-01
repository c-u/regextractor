use super::datatable_error::DataTableError;

pub(crate) struct ZipIter<'a, T> {
    pub(crate) first: &'a Vec<T>,
    pub(crate) second: &'a Vec<T>,
    pub(crate) index: usize,
}

impl<'a, T> ZipIter<'a, T> {
    pub fn new(first: &'a Vec<T>, second: &'a Vec<T>) -> Result<Self, DataTableError> {
        if first.len() != second.len() {
            return Err(DataTableError::InconsistentContainerSize);
        }
        Ok(Self {
            first,
            second,
            index: 0,
        })
    }
}

impl<'a, T> Iterator for ZipIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.first.len() {
            return None;
        }
        self.index += 1;
        Some((&self.first[self.index - 1], &self.second[self.index - 1]))
    }
}

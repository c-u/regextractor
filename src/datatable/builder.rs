use super::{datatable_error::DataTableError, DataTable};

use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct DataTableBuilder<T> {
    pub(crate) data: HashMap<String, Vec<T>>,
}

impl<T: Copy + num::Num> DataTableBuilder<T> {
    pub(crate) fn new(names: &[String]) -> Result<Self, DataTableError> {
        let mut builder = Self {
            data: HashMap::new(),
        };
        for name in names {
            if builder.data.contains_key(name) {
                return Err(DataTableError::DuplicateName);
            }
            builder.data.insert(name.clone(), vec![]);
        }
        Ok(builder)
    }

    pub(crate) fn add_value(&mut self, name: &str, value: T) -> Result<(), DataTableError> {
        if let Some(col) = self.data.get_mut(name) {
            col.push(value);
            Ok(())
        } else {
            Err(DataTableError::InvalidColumnName)
        }
    }

    pub(crate) fn build(
        self,
        base_data_name: Option<&str>,
    ) -> Result<DataTable<T>, DataTableError> {
        let len = self.get_len()?;

        let names: Vec<_> = self.data.keys().cloned().collect();
        let mut dt = if let Some(name) = base_data_name {
            DataTable::<T>::new_with_base_data_name(self.data.keys().count(), &names, name)?
        } else {
            DataTable::<T>::new(self.data.keys().count(), Some(&names), None)
        };

        for i in 0..len {
            dt.add_row(&self.get_row(i)?).ok();
        }
        Ok(dt)
    }

    fn get_len(&self) -> Result<usize, DataTableError> {
        let mut lens = self.data.values().map(|c| c.len());
        if lens.clone().max() != lens.clone().min() {
            return Err(DataTableError::InconsistentBuilderData);
        }
        lens.next().ok_or(DataTableError::InconsistentBuilderData)
    }

    fn get_row(&self, index: usize) -> Result<Vec<T>, DataTableError> {
        self.data
            .values()
            .map(|v| v.get(index).cloned())
            .collect::<Option<Vec<_>>>()
            .ok_or(DataTableError::InvalidColumnIndex)
    }
}

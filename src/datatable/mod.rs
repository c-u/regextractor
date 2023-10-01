use self::datatable_error::DataTableError;

pub(crate) mod builder;
pub mod datatable_error;
mod iter;

#[derive(Default, Debug)]
pub struct DataTable<T> {
    value_columns: usize,
    value_rows: usize,
    base_data_index: Option<usize>,
    value_names: Vec<String>,
    value_data: Vec<Vec<T>>,
    base_data: Vec<T>,
}

impl<T: Copy + num::Num> DataTable<T> {
    pub(crate) fn get_base_data(&self) -> &Vec<T> {
        if let Some(index) = self.base_data_index {
            &self.value_data[index]
        } else {
            &self.base_data
        }
    }

    pub fn new(columns: usize, names: Option<&[String]>, base_data_index: Option<usize>) -> Self {
        let a: Vec<String> = (0..columns).map(|f| f.to_string()).collect();
        DataTable {
            value_columns: columns,
            value_rows: 0,
            base_data_index,
            value_names: names.unwrap_or(&a).to_vec(),
            value_data: vec![Vec::new(); columns],
            base_data: vec![],
        }
    }

    pub fn new_with_base_data_index(
        columns: usize,
        names: Option<&[String]>,
        base_data_index: usize,
    ) -> Result<Self, DataTableError> {
        if base_data_index < columns {
            Ok(Self::new(columns, names, base_data_index.into()))
        } else {
            Err(DataTableError::InvalidCBaseDataIndex)
        }
    }

    pub fn new_with_base_data_name(
        columns: usize,
        names: &[String],
        base_data_name: &str,
    ) -> Result<Self, DataTableError> {
        let index = names
            .iter()
            .position(|n| n == base_data_name)
            .ok_or(DataTableError::InvalidCBaseDataName)?;
        Self::new_with_base_data_index(columns, Some(names), index)
    }

    pub(crate) fn add_row(&mut self, data: &[T]) -> Result<(), DataTableError> {
        if data.len() != self.value_columns {
            return Err(DataTableError::InvalidColumnCount);
        }

        self.value_data
            .iter_mut()
            .zip(data.iter())
            .for_each(|a| a.0.push(*a.1));

        if let Some(base_index) = self.base_data_index {
            self.base_data.push(data[base_index])
        } else if let Some(prev) = self.base_data.last() {
            self.base_data.push(*prev + T::one())
        } else {
            self.base_data.push(T::zero())
        }
        self.value_rows += 1;
        Ok(())
    }

    pub fn get_name(&self, index: usize) -> Result<&str, DataTableError> {
        self.check_column_index(index)?;
        Ok(&self.value_names[index])
    }

    pub fn get_col(&self, index: usize) -> Result<impl Iterator<Item = T> + '_, DataTableError> {
        self.check_column_index(index)?;
        Ok(self.value_data[index].clone().into_iter())
    }
    pub fn get_col_by_name(
        &self,
        name: &str,
    ) -> Result<impl Iterator<Item = T> + '_, DataTableError> {
        self.get_col(
            self.value_names
                .iter()
                .position(|n| n == name)
                .ok_or(DataTableError::InvalidColumnName)?,
        )
    }

    pub fn get_col_with_base(
        &self,
        index: usize,
    ) -> Result<impl Iterator<Item = (&T, &T)> + '_, DataTableError> {
        self.check_column_index(index)?;
        iter::ZipIter::new(&self.base_data, &self.value_data[index])
    }
    pub fn get_col_by_name_with_base(
        &self,
        name: &str,
    ) -> Result<impl Iterator<Item = (&T, &T)> + '_, DataTableError> {
        self.get_col_with_base(
            self.value_names
                .iter()
                .position(|n| n == name)
                .ok_or(DataTableError::InvalidColumnName)?,
        )
    }

    pub fn get_row(&self, index: usize) -> Result<impl Iterator<Item = T> + '_, DataTableError> {
        self.check_column_index(index)?;
        let mut a: Vec<T> = Vec::new();
        a.push(self.get_base_data()[index]);
        for vd in self.value_data.iter() {
            a.push(vd[index]);
        }
        Ok(a.into_iter())
    }

    pub fn get_rows(&self) -> impl Iterator<Item = impl Iterator<Item = T> + '_> + '_ {
        (0..self.get_base_data().len()).map(|i| self.value_data.iter().map(move |vd| vd[i]))
    }

    pub fn get_names(&self) -> impl Iterator<Item = &String> + '_ {
        self.value_names.iter()
    }

    fn check_column_index(&self, index: usize) -> Result<(), DataTableError> {
        if index >= self.get_base_data().len() {
            Err(DataTableError::InvalidColumnIndex)
        } else {
            Ok(())
        }
    }
}

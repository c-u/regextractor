use datatable::{datatable_error::DataTableError, DataTable};
use error::ExtractionError;
use fancy_regex::Regex;
use std::io::Read;

pub mod datatable;
pub mod error;
mod filter_iter;
#[derive(Debug)]
pub struct NamedRegex {
    pub name: String,
    pub regex: Regex,
}

impl NamedRegex {
    pub fn new_from_string(name: &str, regex: &str) -> Option<Self> {
        Some(Self {
            name: name.into(),
            regex: Regex::new(regex).ok()?,
        })
    }
}
/// Extracts and processes data from an input source based on regular expressions.
///
/// This function takes an input data source implementing the `Read` trait and extracts specific data
/// patterns based on provided regular expressions (`data_regex`). It filters lines from the input
/// based on inclusion and exclusion criteria specified by `included_lines` and `excluded_lines`.
///
/// The extracted data is used to construct a data table of generic type `T` using `DataTableBuilder`
/// from the `datatable` crate. The user can optionally provide a `base_data_name` to indicate that data
/// of the regarding row shall be used as base data ('x-axis').
///
/// # Parameters
///
/// - `reader`: An input source implementing the `Read` trait.
/// - `data_regex`: A vector of named regular expressions (`NamedRegex`) for data extraction.
/// - `included_lines`: A vector of regular expressions (`Regex`) for line inclusion criteria.
/// - `excluded_lines`: A vector of regular expressions (`Regex`) for line exclusion criteria.
/// - `base_data_name`: An optional string reference (`Option<&str>`) for setting a base name for extracted data.
/// - `group`: A boolean flag indicating whether the data are contained in the first group of each regular expression
///
/// # Type Parameters
///
/// - `Reader`: A type implementing the `Read` trait for the input data source.
/// - `T`: A generic type parameter that represents the data type of values in the resulting data table.
///
/// # Returns
///
/// - `Result<DataTable<T>, ExtractionError>`: A `Result` containing the constructed data table
///   or an `ExtractionError` in case of an error during the extraction and processing.
///
/// # Errors
///
/// This function may return an `ExtractionError` in case of errors during data extraction or table construction.
pub fn extract_data<Reader, T>(
    reader: Reader,
    data_regex: Vec<NamedRegex>,
    included_lines: Vec<Regex>,
    excluded_lines: Vec<Regex>,
    base_data_name: Option<&str>,
    group: bool,
) -> Result<DataTable<T>, ExtractionError>
where
    Reader: Read,
    T: Copy + num::Float + std::str::FromStr,
{
    let mut builder: datatable::builder::DataTableBuilder<T> =
        datatable::builder::DataTableBuilder::new(
            &data_regex
                .iter()
                .map(|r| r.name.clone())
                .collect::<Vec<_>>(),
        )?;

    filter_iter::FilterIter::new(reader, included_lines, excluded_lines).try_for_each(
        |filtered_line| -> Result<(), DataTableError> {
            if let Ok(line) = filtered_line {
                for (name, value) in get_numbers(&line, &data_regex, group) {
                    builder.add_value(&name, value)?;
                }
            }
            Ok(())
        },
    )?;

    let dt = builder.build(base_data_name)?;
    Ok(dt)
}

/// Filters data from an input source based on regular expressions.
///
/// This function takes an input data source implementing the `Read` trait and filters lines from the input
/// based on inclusion and exclusion criteria specified by `included_lines` and `excluded_lines`.
///
/// # Parameters
///
/// - `reader`: An input source implementing the `Read` trait.
/// - `included_lines`: A vector of regular expressions (`Regex`) for line inclusion criteria.
/// - `excluded_lines`: A vector of regular expressions (`Regex`) for line exclusion criteria.
///
/// # Type Parameters
///
/// - `Reader`: A type implementing the `Read` trait for the input data source.
///
/// # Returns
///
/// - `Result<Vec<String>, ExtractionError>`: A `Result` containing the filtered lines
///   or an `ExtractionError` in case of an error during the extraction and processing.
///
/// # Errors
///
/// This function may return an `ExtractionError` in case of errors during data extraction or table construction.
pub fn filter<Reader>(
    reader: Reader,
    included_lines: Vec<Regex>,
    excluded_lines: Vec<Regex>,
) -> Result<std::vec::Vec<String>, ExtractionError>
where
    Reader: Read,
{
    let mut output = vec![];
    filter_iter::FilterIter::new(reader, included_lines, excluded_lines).try_for_each(
        |filtered_line| -> Result<(), DataTableError> {
            if let Ok(line) = filtered_line {
                output.push(line);
            };
            Ok(())
        },
    )?;

    Ok(output)
}

fn get_number<T>(line: &str, rgx: &Regex, group: bool) -> T
where
    T: num::Float + std::str::FromStr,
{
    let match_index = if group { 1 } else { 0 };
    if let Ok(Some(r)) = rgx.captures(line) {
        let a = if let Some(d) = r.get(match_index) {
            match d.as_str().parse() {
                std::result::Result::Ok(n) => n,
                Err(_) => T::nan(),
            }
        } else {
            T::nan()
        };
        return a;
    }
    T::nan()
}

fn get_numbers<T>(line: &str, rgxs: &[NamedRegex], group: bool) -> Vec<(String, T)>
where
    T: num::Float + std::str::FromStr,
{
    rgxs.iter()
        .map(|rgx| (rgx.name.clone(), get_number(line, &rgx.regex, group)))
        .collect()
}

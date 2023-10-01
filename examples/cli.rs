use clap::{Parser, Subcommand};
use fancy_regex::Regex;
use regextractor::NamedRegex;
use std::fs::File;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Simple program extract data from line based text files like logs or gcode

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extracts data into a csv format
    ExtractData {
        #[arg(short, long)]
        file: String,

        /// Regex to extract data from a line. Can be specified several times to extract multiple values from a line
        #[arg(short, long)]
        data_expr: Vec<String>,

        /// Name of the extracted data. Has to be the same order as 'data_expr'.
        #[arg(short, long)]
        names: Vec<String>,

        /// Data is only extracted from lines which match one of these expressions. Can be specified several times
        #[arg(short, long)]
        include_expr: Vec<String>,

        /// Data is not extracted from lines which match one of these expressions. Can be specified several times
        #[arg(short, long)]
        skip_expr: Vec<String>,

        /// Use the first group of the match as data instead of the full match.
        #[arg(short, long)]
        group: bool,
    },
    /// Filter input based on regular expressions
    FilterData {
        #[arg(short, long)]
        file: String,

        /// Data is only extracted from lines which match one of these expressions. Can be specified several times. All lines are included if no regex is specified
        #[arg(short, long)]
        include_expr: Vec<String>,

        /// Data is not extracted from lines which match one of these expressions. Can be specified several times
        #[arg(short, long)]
        skip_expr: Vec<String>,
    },
}
fn get_expr_name_from_regex(regex: &Regex) -> Option<&str> {
    regex.capture_names().nth(1).flatten()
}

//
fn main() {
    let args = Cli::parse();

    let output = match args.command {
        Commands::ExtractData {
            file,
            data_expr,
            names,
            include_expr,
            skip_expr,
            group,
        } => extract(file, data_expr, names, include_expr, skip_expr, group),
        Commands::FilterData {
            file,
            include_expr,
            skip_expr,
        } => filter(file, include_expr, skip_expr),
    };

    output.iter().for_each(|o| println!("{}", o));
}

fn extract(
    file: String,
    data_expr: Vec<String>,
    names: Vec<String>,
    include_expr: Vec<String>,
    skip_expr: Vec<String>,
    group: bool,
) -> Vec<String> {
    let file = File::open(file.clone())
        .unwrap_or_else(|_| panic!("Could not open specified file: '{}'", file));

    let mut regexes = Vec::<NamedRegex>::new();
    let mut includes = Vec::<Regex>::new();
    let mut ignores = Vec::<Regex>::new();

    let mut counter = 0;

    for (expr, expr_name) in data_expr.iter().zip(
        names
            .iter()
            .map(Some)
            .chain(std::iter::repeat_with(|| None)),
    ) {
        let regex =
            Regex::new(expr).unwrap_or_else(|_| panic!("Invalid regular expression: '{}'", expr));

        let capture_name = get_expr_name_from_regex(&regex);

        let name = if let Some(n) = expr_name {
            n.clone()
        } else if let Some(cn) = capture_name {
            cn.to_string()
        } else {
            counter += 1;
            counter.to_string()
        };

        regexes.push(NamedRegex { name, regex });
    }

    for incl in include_expr {
        includes.push(
            Regex::new(&incl).unwrap_or_else(|_| panic!("Invalid regular expression: '{}'", incl)),
        );
    }
    for ignr in skip_expr {
        ignores.push(
            Regex::new(&ignr).unwrap_or_else(|_| panic!("Invalid regular expression: '{}'", ignr)),
        );
    }

    let data = regextractor::extract_data(file, regexes, includes, ignores, None, group)
        .expect("Could not extract data from file.");

    let mut out = vec![];

    out.push(data.get_names().cloned().collect::<Vec<_>>().join(";"));

    for row in data.get_rows() {
        let csv_col: Vec<_> = row.map(|f: f32| f.to_string()).collect();
        out.push(csv_col.join(";"));
    }
    out
}
fn filter(file: String, include_expr: Vec<String>, skip_expr: Vec<String>) -> Vec<String> {
    let file = File::open(file.clone())
        .unwrap_or_else(|_| panic!("Could not open specified file: '{}'", file));

    let includes = include_expr.iter().map(|incl| {
        Regex::new(incl).unwrap_or_else(|_| panic!("Invalid regular expression: '{}'", incl))
    });
    let ignores = skip_expr.iter().map(|excl| {
        Regex::new(excl).unwrap_or_else(|_| panic!("Invalid regular expression: '{}'", excl))
    });

    regextractor::filter(file, includes.collect(), ignores.collect())
        .expect("Could not filter file.")
}

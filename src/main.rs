#[macro_use] extern crate prettytable;
use prettytable::Table;
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use walkdir::WalkDir;
use std::time::Instant;
use std::fs;

/// Wizzy: Efficiently counts files and directories
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Root directory to start counting from
    #[clap(long, default_value = ".")]
    root: String,

    /// Sort order for file counts
    #[clap(long, default_value_t = SortOrder::Asc)]
    sort: SortOrder,

    /// Calculate total file size
    #[clap(long)]
    count_size: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
enum SortOrder {
    Asc,
    Desc,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let dir_map = count_files_and_dirs(&args.root, args.count_size);

    let mut entries: Vec<_> = dir_map.into_iter().collect();
    sort_entries(&mut entries, args.sort);

    // Create the table
    let mut table = Table::new();

    // Conditionally add "Size (GB)" column based on --count-size flag
    if args.count_size {
        table.add_row(row!["Directory", "Subdirectories", "Files", "Size (GB)"]);
    } else {
        table.add_row(row!["Directory", "Subdirectories", "Files"]);
    }

    let (total_files, total_size) = entries.iter().fold((0, 0u64), |(total_files, total_size), (_, (_, files, size))| {
        (total_files + files, total_size + size)
    });

    // Add rows to the table
    for (dir, (subdirs, files, size)) in entries {
        if args.count_size {
            let size_in_gb = size as f64 / 1_073_741_824.0; // Convert bytes to gigabytes
            table.add_row(row![dir, subdirs.to_string(), files.to_string(), format!("{:.2}", size_in_gb)]);
        } else {
            table.add_row(row![dir, subdirs.to_string(), files.to_string()]);
        }
    }

    // Print the table to stdout
    table.printstd();

    let total_size_gb = total_size as f64 / 1_073_741_824.0; // Convert bytes to gigabytes

    println!("Total files: {}", total_files);
    if args.count_size {
        println!("Total size in GB: {:.2}", total_size_gb);
    }

    println!("Time taken: {:?}", start.elapsed());
}

fn count_files_and_dirs(root: &str, count_size: bool) -> HashMap<String, (usize, usize, u64)> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .par_bridge()
        .fold(HashMap::new, |mut acc, entry| {
            let path = entry.path();
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            let parent_str = parent.to_str().unwrap_or("");

            let counter = acc.entry(String::from(parent_str)).or_insert((0, 0, 0));

            if entry.file_type().is_dir() {
                counter.0 += 1;
            } else {
                counter.1 += 1;
                if count_size {
                    let file_size = fs::metadata(path).map(|metadata| metadata.len()).unwrap_or(0);
                    counter.2 += file_size;
                }
            }

            acc
        })
        .reduce(HashMap::new, |mut a, b| {
            for (key, (subdirs, files, size)) in b {
                let counter = a.entry(key).or_insert((0, 0, 0));
                counter.0 += subdirs;
                counter.1 += files;
                counter.2 += size;
            }
            a
        })
}

fn sort_entries(entries: &mut Vec<(String, (usize, usize, u64))>, order: SortOrder) {
    match order {
        SortOrder::Asc => entries.sort_by_key(|k| k.1 .1),
        SortOrder::Desc => entries.sort_by(|a, b| b.1 .1.cmp(&a.1 .1)),
    }
}

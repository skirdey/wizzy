# Wizzy - File and Directory Analyzer

![OIG](https://github.com/skirdey/wizzy/assets/5145792/f5d8890e-e35a-42c2-a679-75f52869403e)


## Introduction

**Wizzy** is a command-line tool designed for efficient file count with cross-platform support.  

Count subdirectories and files
```bash
$ ./target/release/wizzy.exe --root "H:\test\sub-test" --sort desc
+------------------+----------------+-------+
| Directory        | Subdirectories | Files |
+------------------+----------------+-------+
| H:\test\sub-test | 0              | 1     |
+------------------+----------------+-------+
| H:\test          | 1              | 0     |
+------------------+----------------+-------+
Total files: 1
Time taken: 3.0214ms
```


Count subdirectories, files and total files' size
```bash
./target/release/wizzy.exe --root "H:\test\sub-test" --sort desc --count-size
+------------------+----------------+-------+-----------+
| Directory        | Subdirectories | Files | Size (GB) |
+------------------+----------------+-------+-----------+
| H:\test\sub-test | 0              | 1     | 0.00      |
+------------------+----------------+-------+-----------+
| H:\test          | 1              | 0     | 0.00      |
+------------------+----------------+-------+-----------+
Total files: 1
Total size in GB: 0.00
Time taken: 3.341ms

```

## Features

- Fast and efficient file system scanning.
- Cross-platform support (Windows, macOS, Linux).
- Command-line interface for easy integration with scripts.
- Output in an easily readable table format.

## Building Wizzy

### Prerequisites

- Rust programming language: Install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Building from Source

1. **Clone the Repository:**
   ```sh
   git clone https://your-repository-url/wizzy.git
   cd wizzy
   ```

2. **Build the Project:**
   ```sh
   cargo build --release
   ```
   The executable will be located in `target/release`.

## Installation

### Windows

1. **Copy the Executable:**
    - Copy `wizzy.exe` from `target/release` to a desired folder, e.g., `C:\Users\YourUsername\bin`.

2. **Add to PATH:**
    - Right-click on 'This PC' or 'My Computer' and choose 'Properties'.
    - Click 'Advanced system settings' and then 'Environment Variables'.
    - Under 'System variables', find and select `Path`, then click 'Edit'.
    - Click 'New' and add the path to the folder where `wizzy.exe` is located.
    - Click 'OK' to close all dialogs.

### macOS / Linux

1. **Copy the Executable:**
    - Copy the `wizzy` binary from `target/release` to a desired folder, e.g., `/usr/local/bin`:
      ```sh
      cp target/release/wizzy /usr/local/bin/
      ```

2. **Make Executable:**
    - Ensure `wizzy` is executable:
      ```sh
      chmod +x /usr/local/bin/wizzy
      ```

## Usage

Run `wizzy` from the command line with the following options:

- `--root <path>`: Specify the root directory to start scanning. Defaults to the current directory if not provided.
- `--sort <asc|desc>`: Sort the output by the number of files in ascending or descending order.
- `--count-size` (default off): Add total file size in GB to the counts. Increases total run time of the script.

### Example

```sh
wizzy --root "path/to/directory" --sort desc
```

This command will scan the specified directory and display a table of directories, the number of sub-directories, and the total number of files, sorted in descending order.

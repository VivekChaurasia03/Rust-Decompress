## Rust Zip Decompression CLI Tool

This project is a small CLI tool written in Rust that performs basic decompression on a file provided by the user. It utilizes the `zip` crate to work with zip files.

### Usage

To use the tool, run the following command:

```bash
cargo run <filename>
```

Replace `<filename>` with the path to the file you want to unzip.

### Code Overview

#### Main Function Structure

The code follows a common pattern in Rust projects where the `main` function is used to cleanly exit from the program, and the actual logic is handled in the `real_main` function.

#### Command-Line Arguments

The tool takes command-line arguments to determine the file to be unzipped. A basic check is performed to ensure that the correct number of arguments is provided.

#### File Processing

1. The tool opens the specified file.
2. It creates a mutable archive to work with the file and process its content.
3. The archive is traversed using a `for` loop to handle each file inside.

#### File Extraction

For each file in the archive:

- The name is validated as a safe path.
- If it is a directory, a corresponding directory is created.
- If it is a file, it is extracted to the specified path, and its size is printed.

#### Permissions (Linux)

The tool attempts to set permissions for the extracted files on Linux using the `std::os::unix::fs::PermissionsExt`. However, this part is currently commented out, awaiting documentation around setting permissions for Windows.

### How to Run

1. Clone the repository.
2. Navigate to the project directory.
3. Run the tool using the provided cargo command:

```bash
cargo run <filename>
```

### Notes

- The tool aims to maintain the same folder structure as the zip file during extraction.
- The logic for permissions on Linux is present but commented out until the equivalent functionality is explored for Windows.

Feel free to explore, modify, and contribute to this Rust CLI decompression tool!

Here’s the updated README to reflect the need for `cargo run --`:

# mingrep

`mingrep` is a simple command-line utility written in Rust, inspired by the well-known Unix `grep` tool. It searches through a file for lines that match a specific query string.

## Features

- **Fast and Lightweight:** Built in Rust, `mingrep` leverages Rust’s speed and safety.
- **Simple Usage:** Requires just two arguments— the **search term** and the **file path**.
- **Line-Based Matching:** Displays any lines where the query string is found.

## Installation

1. **Clone this repository**:
    ```bash
    git clone https://github.com/<your-username>/mingrep.git
    ```

2. **Navigate to the project directory**:
    ```bash
    cd mingrep
    ```

3. **Run the project using Cargo**:
    ```bash
    cargo run -- <search_term> <file_path>
    ```

## Usage

1. **Basic usage**:
    ```bash
    cargo run -- <search_term> <file_path>
    ```
   - **search_term**: The word or phrase you want to search for.
   - **file_path**: The path to the file you want to search in.

2. **Example**:
    ```bash
    cargo run -- hello README.md
    ```
   This command will search for the word `hello` in the file `README.md` and print out any matching lines.

## How It Works

`mingrep` takes two command-line arguments:
1. The word or phrase to search for.
2. The file path to search within.

It reads the file line by line, checks each line for the presence of the query string, and prints out any matching lines.

### Example Output
Suppose you have a file `example.txt` containing:
```
Hello, world!
This is an example file.
Search for something interesting here.
```

Running the command:
```bash
cargo run -- example example.txt
```

Will output:
```
String that we have to find is : to
File in which we are searching is : poem.txt
With Text :
 This is an example file.
```

## Contributing

Contributions are always welcome! Feel free to:

- Submit issues for bugs or feature requests.
- Fork the repository and open a pull request with new features or fixes.

## License

This project is licensed under the [MIT License](LICENSE).

---

Happy hacking with Rust! If you have any questions or feedback, please open an issue or reach out. Thank you for using **mingrep**!

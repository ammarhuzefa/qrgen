# qrgen

## Description

qrgen is a command-line application that generates QR codes for given text or urls and outputs an image in SVG or PNG formats.

## Features
- Generates QR codes from text, URLs, or file contents.
- Supports output in **SVG** and **PNG** formats.
- Saves the QR code with a specified filename.
- Uses the `qrcode`, `resvg`, and `image` crates for rendering.

## Requirements

- **Operating System**: Linux (Ubuntu recommended) or macOS.
- **Rust**: Version 1.42.0 or later.
- A terminal or command-line interface.

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/ammarhuzefa/qrgen
    cd qrgen
    ```

2. **Build the application in release mode:**

    ```bash
    cargo build --release
    ```

The compiled binary will be located at
```bash
target/release/qrgen`.
```

## Usage

### Running the Program

Navigate to the release directory:

```bash
cd target/release
```
Run the program with a url to our repository as input.

```bash
./qrgen https://github.com/ammarhuzefa/qrgen
```

## Output
The generated QR code will be saved in qrcode.png (default)

## Notes
Ensure the program has execute permissions:

```bash
chmod +x qrgen
```

The application can handle filenames pointing to `.txt` files located in the same directory as input.

You can also specify and change the name of the output file (second argument), and produce output as an svg instead (third argument). Example usage:
```bash
./qrgen https://github.com/ammarhuzefa/qrgen repo
```
or
```bash
./qrgen https://github.com/ammarhuzefa/qrgen repo svg
```
or (2nd argument defaulted)
```bash
./qrgen https://github.com/ammarhuzefa/qrgen - svg
```

## Contributing
Contributions are welcome! Feel free to submit a pull request or open an issue for bug reports or feature requests.

## Contact
For any questions or feedback, please contact:

GitHub: ammarhuzefa
Email: ammarhuzefa02@gmail.com

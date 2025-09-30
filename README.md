# dpl-sh

A simple Terminal User Interface (TUI) application in Rust for fast and easy communication with DeepL, allowing you to quickly translate any text you want.

## Features

- **Fast and intuitive TUI:** Easily input text and receive translations right from your terminal.
- **DeepL integration:** Uses the DeepL API for high-quality translations.
- **Simple setup:** Minimal configuration required—start translating in seconds.
- **Customizable:** Choose source and target languages, copy results, and more.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A DeepL API key ([Get one here](https://www.deepl.com/pro-api))

### Build from source

```sh
git clone https://github.com/ffex/dpl-sh.git
cd dpl-sh
cargo build --release
```

The executable will be located at `target/release/dpl-sh`.

## Usage

```sh
./dpl-sh
```

The app will prompt you for text to translate and ask for your DeepL API key (or read from an environment variable like `DEEPL_API_KEY`).

### Example

```
Enter text to translate: Hello, world!
Select target language: [de] German
Translation: Hallo, Welt!
```

## Configuration

You can set your DeepL API key as an environment variable for convenience:

```sh
export DEEPL_API_KEY=your-api-key-here
```

## Supported Languages

All languages supported by DeepL. The app allows you to select both source and target languages.

## Contributing

Pull requests and suggestions are welcome! Please open an issue or PR if you have ideas or improvements.

## License

This project is licensed under the MIT License.

---

**Project description:**  
A simple TUI application in Rust to communicate fast and simple with DeepL to translate what you want.

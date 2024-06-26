# Inquire-Ollama: Your Intelligent Question Answering Companion

Inquire-Ollama: is a command-line tool that allows users to interact with the [Ollama](https://ollama.ai/) LLM models directly from the terminal. This tool provides a simple and intuitive way to inquire questions and receive responses from Ollama models.

## Features

- **Interactive CLI**: Easily inquire questions and get responses.
- **Model Selection**: Choose different models for varied responses.

## Installation

To install Inquire-Ollama:, you need to have Rust and Cargo installed on your system. If you haven't already installed Rust, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install Inquire-Ollama: using Cargo:

```sh
cargo install ollama-inquire
```

## Usage

After installation, you can start using Inquire-Ollama: by running:

```sh
inquire [OPTIONS] [PROMPT]
```

### Options

- `--model=[MODEL]`: Specify the model to use (default is 'mistral').
- `--version`: Display the version of the installed Inquire-Ollama: tool.
- `[PROMPT]`: The question or prompt to send to Ollama. Quotation marks are not required.

### Examples

Asking a question using the default model:

```sh
inquire "What is the capital of kenya?"
```
or
```sh
inquire What is the capital of France?
```

Specifying a different model:

```sh
inquire --model=gale "Explain the theory of relativity"
```
Find all available models from Ollama [here](https://ollama.ai/library).

Checking the version:

```sh
inquire --version
```

Seeing the help info:
```sh
inquire --help
```

## Contributing

Contributions to Inquire-Ollama: are welcome! If you have suggestions for improvements or encounter any issues, please feel free to open an issue or submit a pull request on our [GitHub repository](https://github.com/obaraelijah/ollama-inquire).

## License

Inquire-Ollama: is licensed under the MIT License.
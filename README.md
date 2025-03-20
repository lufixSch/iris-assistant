# Iris Assistant

## Overview

Iris is a quick access tool that uses LLM's to perform various text-based tasks such as explaining, summarizing, editing, and answering questions based on provided context. The project consists of both a library (`lib`) for core functionalities and a GUI application (`gui`) for user interaction.

## Features

- **Explain**: Provides an explanation of the given text.
- **Summarize**: Generates a concise summary of the provided content.
- **Edit**: Rewrites the input based on specific instructions.
- **Ask**: Answers questions using the context provided.

## Installation

### Prerequisites

- Rust (version 1.70.0 or later)
- Cargo (Rust's package manager)

#### Steps to Install

1. Build and install the project using Cargo:

   ```bash
   cargo install --path .
   ```

2. On the first start, a configuration file will be generated at `~/.config/iris/config.toml` (The location may change depending on your OS). You can then modify it as needed.

3. Set up the environment variable for OpenAI API key:

   ```bash
   export OPENAI_API_KEY=your_openai_api_key
   ```

## Usage

### Running the GUI Application

The GUI application reads context from standard input. To start it, run:

```bash
iris-gui < path/to/context.txt
```

Replace `path/to/context.txt` with the path to your context file or use any other method to provide input via stdin.

### Using the Library

You can also use Iris as a library in your Rust projects. Add it to your `Cargo.toml`:

```toml
[dependencies]
iris = { path = "../path/to/iris/lib" }
```

Here is an example of how to use the library:

```rust
use iris::{Actions, run};
use iris::IrisConfig;

fn main() {
    let context = "The Earth orbits the Sun at an average distance of about 149.6 million kilometers.";
    let user_input = Some("What is the distance between the earth and the moon?");
    let config = IrisConfig::load().unwrap_or_default();

    match run(&Actions::Ask, context, user_input, config) {
        Some(response) => println!("Response: {}", response),
        None => eprintln!("Failed to generate response!"),
    }
}
```

## Contributing

Feel free to contribute to Iris by opening issues or submitting pull requests. Contributions are welcome!

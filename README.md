# RS Gamebook Engine

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://github.com/vtombesi/rs-gamebook-engine/actions/workflows/rust.yml/badge.svg)](https://github.com/vtombesi/rs-gamebook-engine/actions/workflows/rust.yml)

## Description

RS Gamebook Engine is a Rust library that allows you to create and play interactive text-based gamebooks. Inspired by the nostalgic gamebooks of the 80s, this engine provides a framework for building your own gamebooks with multiple branching paths, choices, combats, and loots.

## Features

- Create interactive gamebooks with multiple branching paths.
- Define pages with text, options, combats, and loots.
- Handle combats between the player and creatures with customizable attributes and attacks.
- Collect and manage items with different types, quantities, and effects.
- Save and load game progress.

## Installation

1. Install Rust and Cargo by following the instructions at [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Clone the RS Gamebook Engine repository:

```shell
git clone https://github.com/your-username/rs-gamebook-engine.git
```

## Usage

To use RS Gamebook Engine in your own project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
rs_gamebook_engine = "0.1.0"
```

Then, you can start using the library by importing the necessary modules:

```rust
use rs_gamebook_engine::gamebook::{Gamebook, Page};
use rs_gamebook_engine::player::Player;
use rs_gamebook_engine::utils::{load_gamebook, read_user_input};

fn main() {
    // Load the gamebook from a JSON file
    let gamebook = load_gamebook("path/to/gamebook.json");

    // Create a new player
    let mut player = Player::new();

    // Start the game from the first page
    let mut current_page_id = gamebook.start_page;

    // Main game loop
    loop {
        // Find the current page
        let current_page = gamebook.pages.iter().find(|p| p.id == current_page_id);

        if let Some(page) = current_page.cloned() {
            // Print the text of the current page
            println!("{}", page.text);

            // If there are no options, exit the loop
            if page.options.is_empty() {
                break;
            }

            // Print the available options
            for (index, option) in page.options.iter().enumerate() {
                println!("{}. {}", index + 1, option.text);
            }

            // Read user input
            let user_input = read_user_input();

            // Handle user input and update the current page
            // ...

        }
    }
}
```

For a complete example of how to use RS Gamebook Engine, refer to the [examples](examples) directory.

## License

RS Gamebook Engine is licensed under the [MIT License](LICENSE).

## Acknowledgements

RS Gamebook Engine is inspired by the gamebooks of the 80s and the joy they brought to many readers and adventurers.

## Get in Touch

If you have any questions, suggestions, or just want to share your gamebook creations, feel free to reach out to us at [valentino.tombesi@gmail.com](mailto:valentino.tombesi@gmail.com).

Happy adventuring!

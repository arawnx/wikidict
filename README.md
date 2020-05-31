# Wikidict

Wikidict is a program that generates a dictionary from various Wikipedia articles.

# Usage

## Compilation

To compile Wikidict, simply run `cargo build --release` and navigate to `target/release`.

## Running

Wikidict takes a number specifying the number of batches through which the program will iterate; for each batch, Wikidict fetches 255 Wikipedia articles and sifts through their words, outputting a JSON-formatted result. It returns a JSON array containing each word found, without repetition, sorted by length. Each batch, when tested, takes around 55 seconds; therefore, each article takes around 0.22 seconds.

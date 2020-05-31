# Wikidict

Wikidict is a program that generates a dictionary from various Wikipedia articles.

# Usage

## Compilation

To compile Wikidict, simply run `cargo build --release` and navigate to `target/release`.

## Running

Wikidict takes a number specifying the number of batches through which the program will iterate; for each batch, Wikidict fetches a Wikipedia article and sifts through its words, outputting a JSON-formatted result. By default it returns a dictionary mapping each word to its frequency; to output only an array of the constituent words, pass `-a` as a command-line argument. Each batch takes approximately 0.35 seconds to complete.

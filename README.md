# Wikidict

Wikidict is a program that generates a dictionary from various Wikipedia articles. 

# Usage

## Compilation

To compile Wikidict, simply run `cargo build --release` and navigate to `target/release`. 

## Running

Wikidict takes a number specifying the number of batches through which the program will iterate; for each batch, Wikidict fetches 255 Wikipedia articles and sifts through their words (& pure numbers, such as dates or Bible chapters). In my experience, it takes ~90 seconds for a batch to be completed, purely by virtue of the amount of time it takes to fetch 255 Wikipedia articles. 
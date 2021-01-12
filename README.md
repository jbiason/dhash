# dhash

A simple DHash calculator in Rust.

## What's a "DHash"?

A DHash is a image [perceptual
hashing](https://en.wikipedia.org/wiki/Perceptual_hashing) in which the image
is reduced to 9x8 pixels and have it converted to grayscale. Each pixel is
compared to the next: if it is whiter, the hash produces a 0 bit; if if is
darker, it produces a 1. The final result is a 64-bit hash for the image.

## Running

Simply call `dhash <filename>`.

## License

GNU AFFERO GENERAL PUBLIC LICENSE, Version 3.

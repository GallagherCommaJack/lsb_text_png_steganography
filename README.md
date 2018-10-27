
# ( Work in progress )

# lsb_text_png_steganography

(Least significant bit text into portable network graphic steganography)

This repo is a module for the commandline tool [`steg`](https://github.com/peterheesterman/steg) but can also be used independently

A steganography strategy that uses the least significant bits of a png to hide text.


### Usage

Add the following to the Cargo.toml in your project:

```toml
[dependencies]
lsb_text_png_steganography = "*" ## replace with latest version
```

and import using ```extern crate```:

```rust
extern crate lsb_text_png_steganography;

use lsb_text_png_steganography::{ hide, reveal };

fn run () {
    let payload_path = "./texts/payload.txt";
    let carrier_path = "./images/carrier.png";
    let output_carrier_path = "./output_carrier.png";
   
    // Coming soon. . .
}
```

## Documentation

Read it. . .(coming soon)



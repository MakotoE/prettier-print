[![Latest version](https://img.shields.io/crates/v/prettier-print.svg)](https://crates.io/crates/prettier-print) [![Documentation](https://docs.rs/prettier-print/badge.svg)](https://docs.rs/prettier-print/)

I'm not a fan of the built-in "pretty-print" debug output (format string `"{:#?}"`) because they don't look so pretty to me. That is why I made this crate.

`prettier-print` contains two modules. The first is `prettier_printer` which adds rainbows and stars to the debug string.

<p align="center">
    <img src="https://raw.githubusercontent.com/MakotoE/prettier-print/main/screenshot.png" width="400" alt="Drake meme">
</p>

```rust
// How to use PrettierPrinter
println!("{}", PrettierPrinter::default().print(&variable));
```

`sparkles` prints the debug string, and then runs game of life on top of the printed string.

https://user-images.githubusercontent.com/36318069/127730094-cbd2884c-3aa4-4084-addd-3536aec43278.mp4

```rust
let stdout = stdout();
Sparkles::new(stdout.lock()).run(&variable)?;
```

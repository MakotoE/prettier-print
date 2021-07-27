#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use prettier_print::prettier_printer::{PrettierPrintDisplayer, Seed};

#[derive(Debug, Arbitrary)]
struct Input {
    seed: Seed,
    debug_str: String,
}

fuzz_target!(|input: Input| {
    let _ = PrettierPrintDisplayer::<i32>::output(input.seed, &input.debug_str);
});

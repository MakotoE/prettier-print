#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use prettier_print::sparkles::CenteredDebugString;

#[derive(Debug, Arbitrary)]
struct Input {
    s: String,
    terminal_size: (u8, u8),
}

fuzz_target!(|input: Input| {
    let mut debug_string = CenteredDebugString::new(
        &input.s,
        (
            input.terminal_size.0 as usize,
            input.terminal_size.0 as usize,
        ),
    );
    for _ in 0..debug_string.len() {
        debug_string.next().unwrap();
    }
});

#[cfg(feature = "extractor-derive")]
#[test]
fn pass() {
    macrotest::expand_args("tests/expand/extraction/pass/*.rs", ["--ugly"]);
}

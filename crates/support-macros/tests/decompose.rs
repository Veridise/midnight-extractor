#[test]
fn pass() {
    macrotest::expand_args("tests/expand/decompose/pass/*.rs", ["--ugly"]);
}

#[test]
#[should_panic]
fn fail() {
    macrotest::expand_args("tests/expand/decompose/fail/*.rs", ["--ugly"]);
}

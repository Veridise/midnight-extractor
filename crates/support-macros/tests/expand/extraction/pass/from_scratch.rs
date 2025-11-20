use mdnt_support_macros::InitFromScratch;

#[derive(InitFromScratch)]
struct S2<F> {
    f: F,
}

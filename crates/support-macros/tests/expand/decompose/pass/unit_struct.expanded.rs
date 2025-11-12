use mdnt_support_macros::DecomposeInCells;
struct S;
impl picus_support::DecomposeInCells for S {
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty()
    }
}

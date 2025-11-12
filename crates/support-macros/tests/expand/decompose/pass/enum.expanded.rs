use mdnt_support_macros::DecomposeInCells;
enum S {
    UnitCase,
    NamedFieldsCase { a: usize, b: usize },
    TupleCase([usize; 4], Vec<S>),
}
impl picus_support::DecomposeInCells for S
where
    usize: picus_support::DecomposeInCells,
    usize: picus_support::DecomposeInCells,
    [usize; 4]: picus_support::DecomposeInCells,
    Vec<S>: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        match self {
            Self::UnitCase => std::iter::empty(),
            Self::NamedFieldsCase { a, b } => {
                std::iter::empty().chain(a.cells()).chain(b.cells())
            }
            Self::TupleCase(f0, f1) => {
                std::iter::empty().chain(f0.cells()).chain(f1.cells())
            }
        }
    }
}

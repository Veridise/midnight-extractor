use mdnt_support_macros::DecomposeInCells;
enum S {
    UnitCase,
    NamedFieldsCase { a: usize, b: usize },
    TupleCase([usize; 4], Vec<S>),
}
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S
where
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    [usize; 4]: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    Vec<S>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        match self {
            Self::UnitCase => std::iter::empty(),
            Self::NamedFieldsCase { a, b } => {
                std::iter::empty().chain(a.cells()).chain(b.cells())
            }
            Self::TupleCase(__0, __1) => {
                std::iter::empty().chain(__0.cells()).chain(__1.cells())
            }
        }
    }
}

use mdnt_support_macros::DecomposeInCells;

#[derive(DecomposeInCells)]
enum S {
    UnitCase,
    NamedFieldsCase { a: usize, b: usize },
    TupleCase([usize; 4], Vec<S>),
}

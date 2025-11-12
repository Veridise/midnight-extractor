use mdnt_support_macros::DecomposeInCells;

#[derive(DecomposeInCells)]
union U {
    a: usize,
    b: usize,
}

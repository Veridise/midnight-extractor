use mdnt_support_macros::DecomposeInCells;

#[derive(DecomposeInCells)]
struct S {
    field: usize,
}

#[derive(DecomposeInCells)]
struct S2 {
    a: usize,
    b: usize,
}

#[derive(DecomposeInCells)]
struct S3 {
    s: S,
    x: [S2; 5],
}

#[derive(DecomposeInCells)]
struct S4<A> {
    a: Vec<A>,
}

#[derive(DecomposeInCells)]
struct S5<A: Copy> {
    a: Vec<A>,
}

#[derive(DecomposeInCells)]
struct S6<A>
where
    A: Copy,
{
    a: Vec<A>,
}

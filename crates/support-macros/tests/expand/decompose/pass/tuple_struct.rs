use mdnt_support_macros::DecomposeInCells;

#[derive(DecomposeInCells)]
struct S(usize);

#[derive(DecomposeInCells)]
struct S2(usize, usize);

#[derive(DecomposeInCells)]
struct S3(S, [S2; 5]);

#[derive(DecomposeInCells)]
struct S4<A>(Vec<A>);

#[derive(DecomposeInCells)]
struct S5<A: Clone>(Vec<A>);

#[derive(DecomposeInCells)]
struct S6<A>(Vec<A>)
where
    A: Clone;

use mdnt_support_macros::DecomposeInCells;
struct S {
    field: usize,
}
impl picus_support::DecomposeInCells for S
where
    usize: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.field.cells())
    }
}
struct S2 {
    a: usize,
    b: usize,
}
impl picus_support::DecomposeInCells for S2
where
    usize: picus_support::DecomposeInCells,
    usize: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells()).chain(self.b.cells())
    }
}
struct S3 {
    s: S,
    x: [S2; 5],
}
impl picus_support::DecomposeInCells for S3
where
    S: picus_support::DecomposeInCells,
    [S2; 5]: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.s.cells()).chain(self.x.cells())
    }
}
struct S4<A> {
    a: Vec<A>,
}
impl<A> picus_support::DecomposeInCells for S4<A>
where
    Vec<A>: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells())
    }
}
struct S5<A: Copy> {
    a: Vec<A>,
}
impl<A: Copy> picus_support::DecomposeInCells for S5<A>
where
    Vec<A>: picus_support::DecomposeInCells,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells())
    }
}
struct S6<A>
where
    A: Copy,
{
    a: Vec<A>,
}
impl<A> picus_support::DecomposeInCells for S6<A>
where
    Vec<A>: picus_support::DecomposeInCells,
    A: Copy,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells())
    }
}

use mdnt_support_macros::DecomposeInCells;
struct S {
    field: usize,
}
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S
where
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.field.cells())
    }
}
struct S2 {
    a: usize,
    b: usize,
}
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S2
where
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells()).chain(self.b.cells())
    }
}
struct S3 {
    s: S,
    x: [S2; 5],
}
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S3
where
    S: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    [S2; 5]: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.s.cells()).chain(self.x.cells())
    }
}
struct S4<A> {
    a: Vec<A>,
}
impl<A> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S4<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells())
    }
}
struct S5<A: Copy> {
    a: Vec<A>,
}
impl<A: Copy> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S5<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
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
impl<A> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S6<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    A: Copy,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.a.cells())
    }
}

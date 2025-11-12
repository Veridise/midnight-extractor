use mdnt_support_macros::DecomposeInCells;
struct S(usize);
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S
where
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells())
    }
}
struct S2(usize, usize);
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S2
where
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    usize: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells()).chain(self.1.cells())
    }
}
struct S3(S, [S2; 5]);
impl picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S3
where
    S: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    [S2; 5]: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells()).chain(self.1.cells())
    }
}
struct S4<A>(Vec<A>);
impl<A> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S4<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells())
    }
}
struct S5<A: Clone>(Vec<A>);
impl<A: Clone> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S5<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells())
    }
}
struct S6<A>(
    Vec<A>,
)
where
    A: Clone;
impl<A> picus_support::DecomposeIn<midnight_proofs::circuit::Cell> for S6<A>
where
    Vec<A>: picus_support::DecomposeIn<midnight_proofs::circuit::Cell>,
    A: Clone,
{
    fn cells(&self) -> impl IntoIterator<Item = midnight_proofs::circuit::Cell> {
        std::iter::empty().chain(self.0.cells())
    }
}

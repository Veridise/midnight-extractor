# mdnt-groups-support

Support traits for the `picus::group` macro.

This crate defines the `DecomposeIn` trait. This trait enables decomposing a complex type into an iterator of simple types.
It is meant to be used with something like `halo2_proofs::circuit::Cell` but the trait is parametric.

## Example 

```
use mdnt_groups_support::DecomposeIn;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
struct Cell(usize);

// The base type needs to implement the trait 
impl DecomposeIn<Cell> for Cell {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        std::iter::once(*self)
    }
}

struct A {
    cells: [Cell; 2],
}

impl DecomposeIn<Cell> for A {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        // Some standard types come with implementations.
        self.cells.cells()
    }
}

struct B {
    a: A,
    cell: Cell
}

impl DecomposeIn<Cell> for B {
    fn cells(&self) -> impl IntoIterator<Item = Cell> {
        self.a.cells().into_iter().chain([self.cell])
    }
}

let b = B {
    a: A {
        cells: [Cell(2), Cell(8)]
    },
    cell: Cell(15)
};
let cells = b.cells().into_iter().collect::<Vec<_>>();
let flat = vec![Cell(2), Cell(8), Cell(15)];
assert_eq!(cells, flat);
```

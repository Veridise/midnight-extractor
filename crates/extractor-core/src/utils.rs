use ff::PrimeField;
use haloumi_ir::stmt::IRStmt;
use mdnt_support::error::Error;
use midnight_proofs::{
    circuit::{Cell, RegionIndex},
    plonk::Expression,
    poly::Rotation,
};

fn cell_to_expr_inner<F: PrimeField>(c: Cell) -> Result<Expression<F>, Error> {
    Ok(c.column.query_cell::<F>(Rotation(c.row_offset.try_into()?)))
}

type StmtInRegion<F> = (RegionIndex, IRStmt<(usize, Expression<F>)>);

/// Convenience method for creating a less-than constraint between a cell and a
/// constant.
pub fn injectable_less_than<F: PrimeField>(
    cell: Cell,
    constant: F,
) -> Result<StmtInRegion<F>, Error> {
    let lhs = cell_to_expr_inner(cell)?;
    let rhs = Expression::Constant(constant);
    Ok((
        cell.region_index,
        IRStmt::lt((cell.row_offset, lhs), (cell.row_offset, rhs)),
    ))
}

use std::collections::HashMap;

use crate::utils::range_lookup;
use haloumi_ir::{cmp::CmpOp, expr::IRBexpr, stmt::IRStmt};
use mdnt_extractor_core::circuit::to_plonk_error;
use mdnt_extractor_core::{
    cells::store::FreshVar,
    chips::{AF, FC},
    entry,
};
use mdnt_extractor_macros::harness;
use mdnt_support::cell_to_expr;
use midnight_circuits::{
    instructions::{
        ArithInstructions as _, AssertionInstructions as _, PublicInputInstructions as _,
    },
    midnight_proofs::plonk::Error,
};

pub type F = mdnt_extractor_core::fields::Blstrs;
pub type K = mdnt_extractor_core::fields::MidnightFp;

entry!("arithmetic/add/field/field", add_foreign);
entry!("arithmetic/add_non_det/field/field", add_foreign_non_det);
entry!("arithmetic/div/field/field", div_foreign);
entry!("arithmetic/inv/field/field", inv_foreign);
entry!("arithmetic/inv0/field/field", inv0_foreign);
entry!("arithmetic/inv_non_det/field/field", inv_foreign_non_det_v2);
entry!("arithmetic/mul_no_const/field/field", mul_no_const_foreign);
entry!("arithmetic/neg/field/field", neg_foreign);
entry!("arithmetic/pow0/field/field", pow0_foreign);
entry!("arithmetic/pow1/field/field", pow1_foreign);
entry!("arithmetic/pow2/field/field", pow2_foreign);
entry!("arithmetic/square/field/field", square_foreign);
entry!("arithmetic/sub/field/field", sub_foreign);

#[harness(range_lookup(8))]
pub fn add_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<AF<F, K>, Error> {
    chip.add(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn add_foreign_non_det(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<FreshVar, Error> {
    let o1 = chip.add(layouter, &x, &y)?;
    let o2 = chip.add(layouter, &x, &y)?;
    chip.assert_not_equal(layouter, &o1, &o2)?;
    Ok(FreshVar)
}

#[harness(range_lookup(8))]
pub fn div_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<AF<F, K>, Error> {
    chip.div(layouter, &x, &y)
}

#[harness(range_lookup(8))]
pub fn inv_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.inv(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn inv_foreign_non_det(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<FreshVar, Error> {
    let o1 = chip.inv(layouter, &x)?;
    let o2 = chip.inv(layouter, &x)?;
    chip.assert_not_equal(layouter, &o1, &o2)?;
    Ok(FreshVar)
}

#[harness(range_lookup(8))]
pub fn inv_foreign_non_det_v2(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
    injected_ir: &mut InjectedIR<F>,
) -> Result<FreshVar, Error> {
    let o1 = chip.inv(layouter, &x)?;
    let o2 = chip.inv(layouter, &x)?;
    let o1_parts = chip.as_public_input(layouter, &o1)?;
    let o2_parts = chip.as_public_input(layouter, &o2)?;
    let (o1_parts, o2_parts) = layouter.assign_region(
        || "assert not equal",
        |mut region| {
            let mut column_offsets = HashMap::new();
            let o1_parts = o1_parts
                .iter()
                .map(|c| {
                    let offset_ref = column_offsets.entry(c.cell().column.index()).or_default();
                    let next_offset = *offset_ref;
                    *offset_ref += 1;
                    c.copy_advice(
                        || "cell",
                        &mut region,
                        c.cell().column.try_into().map_err(to_plonk_error)?,
                        next_offset,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let o2_parts = o2_parts
                .iter()
                .map(|c| {
                    let offset_ref = column_offsets.entry(c.cell().column.index()).or_default();
                    let next_offset = *offset_ref;
                    *offset_ref += 1;
                    c.copy_advice(
                        || "cell",
                        &mut region,
                        c.cell().column.try_into().map_err(to_plonk_error)?,
                        next_offset,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok((o1_parts, o2_parts))
        },
    )?;

    std::iter::zip(o1_parts, o2_parts)
        .inspect(|(o1, o2)| assert_eq!(o1.cell().region_index, o2.cell().region_index))
        .try_for_each(|(o1, o2)| -> Result<(), Error> {
            let region = o1.cell().region_index;
            let stmt = IRStmt::assert(IRBexpr::Not(Box::new(IRBexpr::Cmp(
                CmpOp::Eq,
                (
                    o1.cell().row_offset,
                    cell_to_expr!(&o1, F).map_err(to_plonk_error)?,
                ),
                (
                    o2.cell().row_offset,
                    cell_to_expr!(&o2, F).map_err(to_plonk_error)?,
                ),
            ))));
            injected_ir.entry(region).or_default().push(stmt);

            Ok(())
        })?;
    Ok(FreshVar)
}

#[harness(range_lookup(8))]
pub fn inv0_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.inv0(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn mul_no_const_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<AF<F, K>, Error> {
    chip.mul(layouter, &x, &y, None)
}

#[harness(range_lookup(8))]
pub fn neg_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.neg(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn pow0_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.pow(layouter, &x, 0)
}

#[harness(range_lookup(8))]
pub fn pow1_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.pow(layouter, &x, 1)
}

#[harness(range_lookup(8))]
pub fn pow2_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.pow(layouter, &x, 2)
}

#[harness(range_lookup(8))]
pub fn square_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    x: AF<F, K>,
) -> Result<AF<F, K>, Error> {
    chip.square(layouter, &x)
}

#[harness(range_lookup(8))]
pub fn sub_foreign(
    chip: &FC<F, K>,
    layouter: &mut impl Layouter<F>,
    (x, y): (AF<F, K>, AF<F, K>),
) -> Result<AF<F, K>, Error> {
    chip.sub(layouter, &x, &y)
}

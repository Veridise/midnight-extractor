use mdnt_extractor_core::fields::Blstrs as F;
use mdnt_extractor_macros::harness;
use mdnt_support::{cell_to_expr, circuit::injected::InjectedIR, ir::stmt::IRStmt};
use midnight_circuits::{
    parsing::{DateFormat, Separator},
    types::{AssignedByte, AssignedNative},
};
use midnight_proofs::{
    circuit::RegionIndex,
    plonk::{Error, Expression},
};

use crate::utils::{range_lookup, vec2array};
use mdnt_extractor_core::{chips::PG, entry};

/// Emits a range check on the provided range of the form `lower <= value <= upper`.
///
/// # Panics
///
/// If `lower > upper`.
fn emit_range_check(
    value: &AssignedNative<F>,
    lower: u64,
    upper: u64,
    injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
) -> Result<(), Error> {
    assert!(lower <= upper);
    let region = value.cell().region_index;
    let row = value.cell().row_offset;
    let expr = cell_to_expr!(value, F)?;
    let lower = Expression::from(lower);
    let upper = Expression::from(upper);

    let ir = injected_ir.entry(region).or_default();
    ir.push(IRStmt::le(lower, expr.clone()).with(row));
    ir.push(IRStmt::le(expr, upper).with(row));

    Ok(())
}

fn emit_range_check_byte(
    value: &AssignedByte<F>,
    lower: u64,
    upper: u64,
    injected_ir: &mut InjectedIR<RegionIndex, Expression<F>>,
) -> Result<(), Error> {
    emit_range_check(&value.into(), lower, upper, injected_ir)
}

entry!("parser/ascii_to_int_1/parser/byte", ascii_to_int::<1>);
entry!("parser/ascii_to_int_5/parser/byte", ascii_to_int::<5>);
#[harness(range_lookup(8))]
pub fn ascii_to_int<const N: usize>(
    chip: &PG<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; N],
    injected_ir: &mut InjectedIR<F>,
) -> Result<AssignedNative<F>, Error> {
    for ch in &input {
        emit_range_check_byte(ch, 48, 57, injected_ir)?;
    }
    chip.ascii_to_int(layouter, &input)
}

entry!(
    "parser/date_to_int_YYYYMMDD_no_sep/parser/byte",
    date_to_int_no_sep::</*YEAR_FIRST=*/ true>
);
entry!(
    "parser/date_to_int_DDMMYYYY_no_sep/parser/byte",
    date_to_int_no_sep::</*YEAR_FIRST=*/ false>
);
#[harness(range_lookup(8))]
pub fn date_to_int_no_sep<const YEAR_FIRST: bool>(
    chip: &PG<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; 8],
    injected_ir: &mut InjectedIR<F>,
) -> Result<AssignedNative<F>, Error> {
    let df = if YEAR_FIRST {
        DateFormat::YYYYMMDD
    } else {
        DateFormat::DDMMYYYY
    };
    for ch in &input {
        emit_range_check_byte(ch, 48, 57, injected_ir)?;
    }
    chip.date_to_int(layouter, &input, (df, Separator::NoSep))
}

entry!(
    "parser/date_to_int_YYYYMMDD_sep/parser/byte",
    date_to_int_sep::</*YEAR_FIRST=*/ true, '-'>
);
entry!(
    "parser/date_to_int_DDMMYYYY_sep/parser/byte",
    date_to_int_sep::</*YEAR_FIRST=*/ false, '-'>
);
#[harness(range_lookup(8))]
pub fn date_to_int_sep<const YEAR_FIRST: bool, const SEP: char>(
    chip: &PG<F>,
    layouter: &mut impl Layouter<F>,
    input: [AssignedByte<F>; 10],
    injected_ir: &mut InjectedIR<F>,
) -> Result<AssignedNative<F>, Error> {
    let df = if YEAR_FIRST {
        DateFormat::YYYYMMDD
    } else {
        DateFormat::DDMMYYYY
    };
    for (n, ch) in input.iter().enumerate() {
        if matches!(
            (df, n),
            (DateFormat::YYYYMMDD, 4)
                | (DateFormat::YYYYMMDD, 7)
                | (DateFormat::DDMMYYYY, 2)
                | (DateFormat::DDMMYYYY, 5)
        ) {
            continue;
        }
        emit_range_check_byte(ch, 48, 57, injected_ir)?;
    }

    chip.date_to_int(layouter, &input, (df, Separator::Sep(SEP)))
}

entry!("parser/fetch_bytes_10_5/parser/byte", fetch_bytes::<10, 5>);
#[harness(range_lookup(8))]
pub fn fetch_bytes<const INPUT: usize, const OUTPUT: usize>(
    chip: &PG<F>,
    layouter: &mut impl Layouter<F>,
    (sequence, idx): ([AssignedByte<F>; INPUT], AssignedNative<F>),
    injected_ir: &mut InjectedIR<F>,
) -> Result<[AssignedByte<F>; OUTPUT], Error> {
    assert!(INPUT >= OUTPUT);
    emit_range_check(&idx, 0, (INPUT - OUTPUT) as u64, injected_ir)?;
    chip.fetch_bytes(layouter, &sequence, &idx, OUTPUT).and_then(vec2array)
}

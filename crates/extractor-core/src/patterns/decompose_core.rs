use ff::{Field, PrimeField};
use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use haloumi::{
    gates::GateRewritePattern, gates::GateScope, gates::RewriteError, gates::RewriteOutput,
};
use haloumi_ir::stmt::IRStmt;
use haloumi_ir_base::cmp::CmpOp;
use midnight_proofs::{
    plonk::{
        Advice, AdviceQuery, ColumnType, Expression, FirstPhase, Fixed, Instance, SecondPhase,
        ThirdPhase,
    },
    poly::Rotation,
};

use mdnt_support::fe_to_big;

type RewriteResult<T> = Result<T, &'static str>;

/// Gate rewrite pattern for decomposition constraints.
///
/// The high-level logic is to create an array of expressions representing all the sums in the
/// polynomials, `x + (y + (z + w))` becomes `[x,y,z,w]`. The left hand side of the equality must
/// be a negated [`AdviceQuery`] in that array. To compose the equality we remove that query from
/// the array. In the previous example lets assume `w` is the query, we are left with an equality
/// as `(w, [x,y,z])`.
///
/// Then, to chain the sequence we check for another [`AdviceQuery`] that points to the same column
/// but not the same offset. For `(w, [x,y,z])` and `(w', [x',y',z'])`, assuming `z` is that query
/// then `z ~ w'`. We join the sequence by removing the recursive query and adding the rhs of the
/// corresponding equality: `(w, [x,y,x',y',z'])`.
///
/// Finally, we reconstruct the sum and emit the constraint: `w = (((x + y) + x') + y') + 'z`.
#[derive(Default, Copy, Clone)]
pub struct DecomposeCorePattern;

impl<F: PrimeField> GateRewritePattern<F, Expression<F>> for DecomposeCorePattern {
    fn match_gate<'a>(&self, gate: GateScope<'a, '_, F, Expression<F>>) -> Result<(), RewriteError>
    where
        F: Field,
    {
        if gate.gate_name() == "arith_gate"
            && gate.region_name() == "decompose core"
            && gate.polynomials().len() == 1
        {
            Ok(())
        } else {
            Err(RewriteError::NoMatch)
        }
    }
    fn rewrite_gate<'a>(
        &self,
        gate: GateScope<'a, '_, F, Expression<F>>,
    ) -> Result<RewriteOutput<'a, Expression<F>>, anyhow::Error>
    where
        F: Field,
    {
        let mut stmts = vec![];
        for (_, exprs) in gate.polynomials_per_row()? {
            rewrite_poly(exprs, &mut stmts, gate.start_row());
        }

        Ok(IRStmt::seq(stmts).map(&|(row, expr)| (row, Cow::Owned(expr))))
    }
}

/// Rewrites the polynomials as either the nicer 1-row decomposition or just one per row as
/// fallback.
fn rewrite_poly<F: PrimeField>(
    exprs: Vec<(usize, Expression<F>)>,
    stmts: &mut Vec<IRStmt<(usize, Expression<F>)>>,
    base_row: usize,
) {
    match try_rewrite(&exprs, base_row) {
        Ok((lhs, rhs)) => {
            stmts.push(IRStmt::constraint(CmpOp::Eq, lhs, rhs));

            log::debug!("Decompose gate was rewritten");
        }
        Err(reason) => {
            log::debug!("Decompose gate was not rewritten: {reason}");
            for (row, expr) in exprs {
                stmts.push(IRStmt::constraint(
                    CmpOp::Eq,
                    (row, expr),
                    (row, Expression::Constant(F::ZERO)),
                ));
            }
        }
    }
}

/// Attempts to rewrite the polynomials as a 1-row decomposition equality constraint. Returns None
/// if it fails.
///
/// See the doc for [`DecomposeCorePattern`] for details on how the algorithm works.
fn try_rewrite<F: PrimeField>(
    exprs: &[(usize, Expression<F>)],
    base_row: usize,
) -> RewriteResult<(ExpressionAtRow<F>, ExpressionAtRow<F>)> {
    let mut eqs = HashMap::with_capacity(exprs.len());

    let last_row = exprs.last().ok_or("Exprs list is empty")?.0;
    for (row, expr) in exprs {
        log::debug!("Working with expression in row {row}");
        let mut rhs = flatten_sums(expr)?;
        log::debug!("Flattening OK");
        let lhs = rhs
            .iter()
            .position(|leaf| {
                matches!(leaf,
                    Expression::Negated(x) if
                        matches!(x.as_ref(), Expression::Advice(q) if q.rotation().0 == 0 )
                )
            })
            .ok_or("Could not find the lhs anchor")?;
        log::debug!("Negated advice was found!");
        let lhs = match rhs.swap_remove(lhs) {
            Expression::Negated(x) => match **x {
                Expression::Advice(q) => q,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        if !is_recursive(&lhs, &rhs) && *row < last_row {
            log::debug!("Is not recursive!");
            return Err("Not recursive");
        }
        eqs.insert(*row, (lhs, rhs.into_iter().cloned().collect()));
    }

    log::debug!("Creating the final expression...");
    process_rec_equality(eqs, exprs.len(), base_row)
}

/// Returns true if for a given equality there is an advice query with a rotation other than 0 to
/// the same advice column in the lhs of the equality and the rhs is not self-refential to the lhs.
fn is_recursive<F>(lhs: &AdviceQuery, rhs: &[&Expression<F>]) -> bool {
    let queries = || {
        rhs.iter().filter_map(|e| match e {
            Expression::Advice(q) => Some(q),
            _ => None,
        })
    };
    queries().all(|a| {
        // No query points to the same cell as the lhs
        !advice_query_eq(a, lhs)
    }) && queries().any(|a| {
        // Same column but different rotation.
        a.column_index() == lhs.column_index() && a.rotation() != lhs.rotation()
    })
}

/// Returns true if the expression contains a sum operand inside.
fn contains_sum<F: Field>(expr: &Expression<F>) -> bool {
    expr.evaluate(
        &|_| false,
        &|_| false,
        &|_| false,
        &|_| false,
        &|_| false,
        &|_| false,
        &|inner| inner,
        &|_, _| true,
        &|lhs, rhs| lhs || rhs,
        &|lhs, _| lhs,
    )
}

/// Returns how many leaf nodes a flatten sum version of this expression would have.
fn n_leafs<F: Field>(expr: &Expression<F>) -> usize {
    expr.evaluate(
        &|_| 1,
        &|_| 1,
        &|_| 1,
        &|_| 1,
        &|_| 1,
        &|_| 1,
        &|_| 1,
        &|lhs, rhs| lhs + rhs,
        &|_, _| 1,
        &|_, _| 1,
    )
}

/// Flattens the expression tree of all sums. Fails if a leaf node has a sum inside.
fn flatten_sums<F: Field>(expr: &Expression<F>) -> RewriteResult<Vec<&Expression<F>>> {
    let mut sums = Vec::with_capacity(n_leafs(expr));
    let mut worklist = VecDeque::from_iter([expr]);

    while !worklist.is_empty() {
        let e = worklist.pop_front().ok_or("Worklist empty")?;
        log::debug!("Working with {e:?}");
        match e {
            Expression::Sum(lhs, rhs) => {
                worklist.push_back(lhs);
                worklist.push_back(rhs);
            }
            e => {
                if contains_sum(e) {
                    log::debug!("Expression contains sums");
                    return Err("The expressions inside a non-sum contains sums");
                }
                sums.push(e);
            }
        }
    }

    Ok(sums)
}

/// Because the definition of equality in == takes into account things that we don't care about and
/// would make queries that we would consider equal not to be.
fn advice_query_eq(lhs: &AdviceQuery, rhs: &AdviceQuery) -> bool {
    lhs.column_index() == rhs.column_index() && lhs.rotation() == rhs.rotation()
}

type ExpressionAtRow<F> = (usize, Expression<F>);

/// Takes the recursive equality and unrolls it over the rows of the region.
fn process_rec_equality<F: PrimeField>(
    mut map: HashMap<usize, (AdviceQuery, Vec<Expression<F>>)>,
    n_rows: usize,
    base_row: usize,
) -> RewriteResult<(ExpressionAtRow<F>, ExpressionAtRow<F>)> {
    log::debug!(
        "base_row = {base_row} | map keys = {:?}",
        map.keys().collect::<Vec<_>>()
    );
    let (anchor_lhs, mut anchor_rhs) = map.remove(&base_row).expect("Missing base row");

    for row_offset in 1..n_rows {
        log::debug!("Processing row offset {row_offset}");
        let row = base_row + row_offset;
        let (mut bumped_lhs, mut bumped_rhs) = map.remove(&row).expect("Missing row");
        log::debug!(
            "Bumped expr from lhs = {bumped_lhs:?} | rhs = {:?}",
            bumped_rhs
        );
        bump_offsets(
            &mut bumped_lhs,
            &mut bumped_rhs,
            row_offset.try_into().unwrap(),
        );
        log::debug!(
            "Bumped expr to lhs = {bumped_lhs:?} | rhs = {:?}",
            bumped_rhs
        );
        anchor_rhs.retain(|e| match e {
            Expression::Advice(q) => !advice_query_eq(q, &bumped_lhs),
            _ => true,
        });
        anchor_rhs.extend(bumped_rhs);
    }

    anchor_rhs.sort_by(|e1, e2| {
        fn match_products<F: PrimeField>(
            lhs1: &Expression<F>,
            rhs1: &Expression<F>,
            lhs2: &Expression<F>,
            rhs2: &Expression<F>,
        ) -> Ordering {
            match (lhs1, rhs1, lhs2, rhs2) {
                (
                    Expression::Constant(lhs),
                    Expression::Advice(_),
                    Expression::Constant(rhs),
                    Expression::Advice(_),
                ) => {
                    let lhs = fe_to_big(*lhs);
                    let rhs = fe_to_big(*rhs);
                    lhs.cmp(&rhs)
                }
                _ => unreachable!(),
            }
        }
        match (e1, e2) {
            (Expression::Advice(_), _) => Ordering::Greater,
            (_, Expression::Advice(_)) => Ordering::Greater,
            (Expression::Product(lhs1, rhs1), Expression::Product(lhs2, rhs2)) => {
                match_products(lhs1, rhs1, lhs2, rhs2)
            }
            _ => unreachable!(),
        }
    });
    anchor_rhs
        .into_iter()
        .reduce(|acc, e| Expression::Sum(Box::new(acc), Box::new(e)))
        .ok_or("No anchor rhs")
        .map(|rhs| ((base_row, Expression::Advice(anchor_lhs)), (base_row, rhs)))
}

/// Increases the value of the rotation in queries by 1
fn bump_offsets<F: Field>(lhs: &mut AdviceQuery, rhs: &mut Vec<Expression<F>>, by: i32) {
    bump_advice_query::<F>(lhs, by);
    for e in rhs {
        bump_expr(e, by);
    }
}

fn bump_expr<F: Field>(e: &mut Expression<F>, by: i32) {
    match e {
        Expression::Fixed(q) => {
            *q = match Fixed.query_cell::<F>(q.column_index(), bump_rot(q.rotation(), by)) {
                Expression::Fixed(q) => q,
                _ => unreachable!(),
            };
        }
        Expression::Advice(q) => bump_advice_query::<F>(q, by),
        Expression::Instance(q) => {
            *q = match Instance.query_cell::<F>(q.column_index(), bump_rot(q.rotation(), by)) {
                Expression::Instance(q) => q,
                _ => unreachable!(),
            }
        }
        Expression::Negated(inner) => bump_expr(inner, by),
        Expression::Sum(_, _) => {
            unreachable!()
        }
        Expression::Product(lhs, rhs) => {
            bump_expr(lhs, by);
            bump_expr(rhs, by);
        }
        Expression::Scaled(expression, _) => bump_expr(expression, by),
        Expression::Constant(_) | Expression::Selector(_) | Expression::Challenge(_) => {}
    }
}

fn bump_rot(r: Rotation, by: i32) -> Rotation {
    Rotation(r.0 + by)
}

fn bump_advice_query<F: Field>(q: &mut AdviceQuery, offset: i32) {
    let advice = match q.phase() {
        0 => Advice::new(FirstPhase),
        1 => Advice::new(SecondPhase),
        2 => Advice::new(ThirdPhase),
        _ => unreachable!(),
    };
    *q = match advice.query_cell::<F>(q.column_index(), bump_rot(q.rotation(), offset)) {
        Expression::Advice(q) => q,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {

    use crate::fields::Blstrs as F;
    use log::LevelFilter;
    use midnight_proofs::{
        plonk::{
            Advice, AdviceQuery, Column, ConstraintSystem, Expression, Fixed, Instance, Selector,
        },
        poly::Rotation,
    };
    use rstest::{fixture, rstest};
    use simplelog::{Config, TestLogger};

    use ff::Field;

    use super::process_rec_equality;
    use std::array::from_fn;

    #[fixture]
    fn cs() -> ConstraintSystem<F> {
        ConstraintSystem::default()
    }

    #[fixture]
    fn without_selector(mut cs: ConstraintSystem<F>) -> RecEqualityTestFixture {
        RecEqualityTestFixture::setup(&mut cs)
    }

    fn run_process_rec_equality_test(test: RecEqualityTestFixture, n: usize, start_row: usize) {
        let _ = TestLogger::init(LevelFilter::Debug, Config::default());
        log::info!(
            " Begin logic ===========================================================================",
        );
        let exprs = test.expr_per_row(start_row..=(start_row + n));
        for (row, (lhs, rhs)) in &exprs {
            log::debug!(" ===========> ROW {row} ");
            for (idx, e) in rhs.iter().enumerate() {
                log::debug!("      ============> EXPR {idx} ");
                log::debug!(
                    "{:?}",
                    D(((*row, Expression::Advice(*lhs)), (*row, e.clone())))
                );
                log::debug!("      <===========");
            }
            log::debug!("<===========");
        }
        let out = process_rec_equality(exprs.into_iter().collect(), n, start_row).map(D);

        log::debug!("Output: {out:?}");
        let expected = D(test.step(n, start_row));
        log::debug!("Expected: {expected:?}");
        log::info!(
            " End logic =============================================================================",
        );
        similar_asserts::assert_eq!(out, Ok(expected));
    }

    #[rstest]
    fn process_rec_equality_tes_test(
        without_selector: RecEqualityTestFixture,
        #[values(0, 1, 4)] n: usize,
    ) {
        run_process_rec_equality_test(without_selector, n, 0)
    }

    struct RecEqualityTestFixture {
        f0: Column<Fixed>,
        a0: Column<Advice>,
        a1: Column<Advice>,
    }

    fn c(n: usize) -> Expression<F> {
        Expression::Constant(vec![F::ONE; n].into_iter().sum())
    }

    impl RecEqualityTestFixture {
        fn setup(cs: &mut ConstraintSystem<F>) -> Self {
            let (_, fixed, advice, _instance) = Self::cols(cs);

            Self {
                f0: fixed[0],
                a0: advice[0],
                a1: advice[1],
            }
        }

        fn lhs_expr(&self) -> Expression<F> {
            self.a0.query_cell::<F>(Rotation::cur())
        }

        fn lhs(&self) -> AdviceQuery {
            match self.lhs_expr() {
                Expression::Advice(a) => a,
                _ => panic!("Advice query is not of the expected type"),
            }
        }

        fn rhs_on_row_flat(&self, row: usize) -> Vec<Expression<F>> {
            let a1 = self.a1.query_cell(Rotation::cur());
            let a0 = self.a0.query_cell(Rotation::next());
            vec![(c(row + 2)) * a1, a0]
        }

        fn query(&self, n: i32) -> (Expression<F>, Expression<F>, Expression<F>) {
            let f0 = self.f0.query_cell(Rotation(n));
            let a1 = self.a1.query_cell(Rotation(n));
            let a0 = self.a0.query_cell(Rotation(n));
            (f0, a1, a0)
        }

        fn expr_per_row(
            &self,
            rows: impl Iterator<Item = usize>,
        ) -> Vec<(usize, (AdviceQuery, Vec<Expression<F>>))> {
            rows.map(|row| (row, (self.lhs(), self.rhs_on_row_flat(row)))).collect()
        }

        fn eq_expr_with(&self, rhs: Expression<F>) -> (AdviceQuery, Expression<F>) {
            (self.lhs(), rhs)
        }

        /// f(n) = f0 * s2 + f(n+1)
        fn step<'a>(
            &self,
            n: usize,
            start_row: usize,
        ) -> ((usize, Expression<F>), (usize, Expression<F>)) {
            log::debug!("call to step(n={n}, start_row={start_row})");
            fn step_rhs_impl(
                f: &RecEqualityTestFixture,
                goal: usize,
                count: usize,
                acc: Expression<F>,
            ) -> Expression<F> {
                log::debug!("call to step_rhs_impl(goal={goal}, count={count}, acc={acc:?})");
                let (_, a1, a0) = f.query((count + 1).try_into().unwrap());
                if count + 1 >= goal {
                    return acc + a0;
                }

                let expr = acc + c(count + 3) * a1;
                step_rhs_impl(f, goal, count + 1, expr)
            }

            let (_, a1, _) = self.query(0);
            let init = c(2) * a1;
            let final_rhs = step_rhs_impl(self, n, 0, init);

            let (lhs, rhs) = self.eq_expr_with(final_rhs);
            ((start_row, Expression::Advice(lhs)), (start_row, rhs))
        }

        fn cols(
            cs: &mut ConstraintSystem<F>,
        ) -> (
            [Selector; 0],
            [Column<Fixed>; 1],
            [Column<Advice>; 2],
            [Column<Instance>; 0],
        ) {
            columns::<F, 0, 1, 2, 0>(cs)
        }
    }

    fn columns<
        F: Field,
        const SELECTOR: usize,
        const FIXED: usize,
        const ADVICE: usize,
        const INSTANCE: usize,
    >(
        meta: &mut ConstraintSystem<F>,
    ) -> (
        [Selector; SELECTOR],
        [Column<Fixed>; FIXED],
        [Column<Advice>; ADVICE],
        [Column<Instance>; INSTANCE],
    ) {
        (
            from_fn(|_| meta.selector()),
            from_fn(|_| meta.fixed_column()),
            from_fn(|_| meta.advice_column()),
            from_fn(|_| meta.instance_column()),
        )
    }

    /// This struct implements a custom Debug that prints the expression as s-expressions for
    /// easier visual debugging.
    #[derive(PartialEq, Eq)]
    struct D(((usize, Expression<F>), (usize, Expression<F>)));

    impl std::fmt::Debug for D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let ((lhs_row, lhs), (rhs_row, rhs)) = &self.0;
            writeln!(f, "lhs[{lhs_row}]:")?;
            {
                let mut p = Printer::new(f);
                p.print_expr(lhs)?;
            }
            writeln!(f)?;
            writeln!(f, "rhs[{rhs_row}]:")?;
            {
                let mut p = Printer::new(f);
                p.print_expr(rhs)
            }
        }
    }

    struct Printer<'a, 'f> {
        f: &'a mut std::fmt::Formatter<'f>,
        indent: Indent,
    }

    impl<'a, 'f> Printer<'a, 'f> {
        fn new(f: &'a mut std::fmt::Formatter<'f>) -> Self {
            // Starts at one indetation level
            Self {
                f,
                indent: Indent::new(),
            }
        }

        fn print_expr(&mut self, expr: &Expression<F>) -> std::fmt::Result {
            match expr {
                Expression::Constant(f) => self.print_f(f),
                Expression::Selector(selector) => {
                    writeln!(self.f, "{}(selector {})", self.indent, selector.index())
                }
                Expression::Fixed(query) => writeln!(
                    self.f,
                    "{}(fixed {} {})",
                    self.indent,
                    query.column_index(),
                    query.rotation().0
                ),
                Expression::Advice(query) => writeln!(
                    self.f,
                    "{}(advice {} {})",
                    self.indent,
                    query.column_index(),
                    query.rotation().0
                ),
                Expression::Instance(query) => writeln!(
                    self.f,
                    "{}(instance {} {})",
                    self.indent,
                    query.column_index(),
                    query.rotation().0
                ),
                Expression::Challenge(challenge) => {
                    write!(self.f, "{}(challenge {})", self.indent, challenge.index())
                }
                Expression::Negated(expression) => {
                    writeln!(self.f, "{}(-", self.indent)?;
                    self.indent.push();
                    self.print_expr(&expression)?;
                    self.indent.pop();
                    writeln!(self.f, "{})", self.indent)
                }
                Expression::Sum(lhs, rhs) => {
                    writeln!(self.f, "{}(+", self.indent)?;
                    self.indent.push();
                    self.print_expr(&lhs)?;
                    self.print_expr(&rhs)?;
                    self.indent.pop();
                    writeln!(self.f, "{})", self.indent)
                }
                Expression::Product(lhs, rhs) => {
                    writeln!(self.f, "{}(*", self.indent)?;
                    self.indent.push();
                    self.print_expr(&lhs)?;
                    self.print_expr(&rhs)?;
                    self.indent.pop();
                    writeln!(self.f, "{})", self.indent)
                }
                Expression::Scaled(lhs, rhs) => {
                    writeln!(self.f, "{}(scaled/*", self.indent)?;
                    self.indent.push();
                    self.print_expr(&lhs)?;
                    self.print_f(&rhs)?;
                    self.indent.pop();
                    writeln!(self.f, "{})", self.indent)
                }
            }
        }

        fn print_f(&mut self, f: &F) -> std::fmt::Result {
            if *f == F::ZERO {
                writeln!(self.f, "{}0", self.indent)
            } else if *f == F::ONE {
                writeln!(self.f, "{}1", self.indent)
            } else if *f == -F::ONE {
                writeln!(self.f, "{}-1", self.indent)
            } else {
                let s = format!("{f:?}");
                if let Some(s) = clean_debug_repr(&s) {
                    writeln!(self.f, "{}0x{s}", self.indent,)
                } else {
                    writeln!(self.f, "{}{s}", self.indent)
                }
            }
        }
    }

    fn clean_debug_repr(s: &str) -> Option<&str> {
        s.strip_prefix("Fq(0x")?.trim_start_matches('0').strip_suffix(')')
    }
    const INDENT: usize = 2;
    struct Indent(String);

    impl Indent {
        fn new() -> Self {
            Self(String::from_iter(std::iter::repeat_n(' ', INDENT)))
        }

        fn push(&mut self) {
            for _ in 0..INDENT {
                self.0.push(' ');
            }
        }

        fn pop(&mut self) {
            for _ in 0..INDENT {
                self.0.pop();
            }
        }
    }

    impl std::fmt::Display for Indent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

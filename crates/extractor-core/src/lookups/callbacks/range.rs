use ff::Field;
use haloumi_ir::meta::HasMeta;
use haloumi_ir::{expr::IRBexpr, stmt::IRStmt};
use haloumi_ir_gen::lookups::callbacks::LookupResult;
use haloumi_ir_gen::{
    lookups::{callbacks::LookupCallbacks, table::LookupTableGenerator},
    temps::{ExprOrTemp, Temps},
};
use haloumi_synthesis::lookups::Lookup;
use midnight_proofs::plonk::Expression;
use std::{array, borrow::Cow, collections::HashSet};

/// Maps a set of tag values to a set of ranges
pub struct TagsToRangesMap<T, V, const TAGS: usize, const VALUES: usize>(
    Vec<([T; TAGS], [V; VALUES])>,
);

impl<T, V, const TAGS: usize, const VALUES: usize> FromIterator<([T; TAGS], [V; VALUES])>
    for TagsToRangesMap<T, V, TAGS, VALUES>
{
    fn from_iter<IT: IntoIterator<Item = ([T; TAGS], [V; VALUES])>>(iter: IT) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Lookup callback for handling lookups used for range constraints based on a N+M column approach
/// where N columns contain tags and M contain the contents of the lookup.
pub struct TagRangeLookup<F: Field, const TAGS: usize, const VALUES: usize> {
    ranges: TagsToRangesMap<F, F, TAGS, VALUES>,
    tag_indices: [usize; TAGS],
    value_indices: [usize; VALUES],
}

fn to_set<const N: usize>(values: [usize; N]) -> HashSet<usize> {
    values.into_iter().collect()
}

fn f_to_e<'a, 'f, F: Field>(
    fs: impl IntoIterator<Item = &'f F>,
) -> impl Iterator<Item = Cow<'a, Expression<F>>> {
    fs.into_iter().copied().map(Expression::Constant).map(Cow::Owned)
}

fn chunk<'a, 'f, F: Field>(
    op: impl Fn(Cow<'a, Expression<F>>, Cow<'a, Expression<F>>) -> IRBexpr<Cow<'a, Expression<F>>>,
    lhs: impl IntoIterator<Item = &'a Expression<F>>,
    rhs: impl IntoIterator<Item = &'f F>,
) -> impl Iterator<Item = IRBexpr<Cow<'a, Expression<F>>>> {
    lhs.into_iter()
        .map(Cow::Borrowed)
        .zip(f_to_e(rhs))
        .map(move |(lhs, rhs)| op(lhs, rhs))
}

impl<F: Field, const TAGS: usize, const VALUES: usize> TagRangeLookup<F, TAGS, VALUES> {
    /// Creates a new TagRangeLookup with the given ranges associated with their tags.
    /// The arrays `tag_indices` and `value_indices` are the indices (not the columns!) in the lookup's
    /// expressions that represent each.
    ///
    /// For example, for a lookup with 1 tag and 2 values, where the tag's expression
    /// is the first one from the list the arrays are then [0] and [1,2]
    pub fn new(
        tag_indices: [usize; TAGS],
        value_indices: [usize; VALUES],
        ranges: impl IntoIterator<Item = ([F; TAGS], [F; VALUES])>,
    ) -> Self {
        Self {
            ranges: ranges.into_iter().collect(),
            tag_indices,
            value_indices,
        }
        .validate()
    }

    fn validate(self) -> Self {
        let tags_set = to_set(self.tag_indices);
        let values_set = to_set(self.value_indices);
        // No duplicate indices in either array.
        assert!(tags_set.len() == self.tag_indices.len());
        assert!(values_set.len() == self.value_indices.len());
        // No indices on double duty.
        assert!(tags_set.intersection(&values_set).next().is_none());
        self
    }

    fn tag_exprs<'a>(&self, lookup: &'a Lookup<Expression<F>>) -> [&'a Expression<F>; TAGS] {
        array::from_fn(|idx| {
            let tag_idx = self.tag_indices[idx];
            &lookup.inputs()[tag_idx]
        })
    }

    pub(super) fn value_exprs<'a>(
        &self,
        lookup: &'a Lookup<Expression<F>>,
    ) -> [&'a Expression<F>; VALUES] {
        array::from_fn(|idx| {
            let val_idx = self.value_indices[idx];
            lookup.inputs().get(val_idx).unwrap_or_else(|| {
                panic!(
                    "Index out of bounds in lookup '{}': len is {} but the index is {}",
                    lookup.name(),
                    lookup.inputs().len(),
                    val_idx
                )
            })
        })
    }

    fn process_row<'a>(
        &self,
        lookup: &'a Lookup<Expression<F>>,
        tags: &[F; TAGS],
        values: &[F; VALUES],
    ) -> IRBexpr<Cow<'a, Expression<F>>> {
        let tags = chunk(IRBexpr::eq, self.tag_exprs(lookup), tags);
        let values = chunk(IRBexpr::lt, self.value_exprs(lookup), values);
        IRBexpr::and_many(tags.chain(values))
    }

    fn process_rows<'a>(
        &self,
        lookup: &'a Lookup<Expression<F>>,
    ) -> IRBexpr<Cow<'a, Expression<F>>> {
        IRBexpr::or_many(
            self.ranges
                .0
                .iter()
                .map(|(tags, values)| self.process_row(lookup, tags, values)),
        )
    }
}

impl<const TAGS: usize, const VALUES: usize, F: Field> LookupCallbacks<F, Expression<F>>
    for TagRangeLookup<F, TAGS, VALUES>
{
    fn on_lookup<'a>(
        &self,
        lookup: &'a Lookup<Expression<F>>,
        _table: &dyn LookupTableGenerator<F>,
        _temps: &mut Temps,
    ) -> LookupResult<'a, Expression<F>> {
        let mut stmt = IRStmt::assert(self.process_rows(lookup)).map(&mut ExprOrTemp::Expr);
        stmt.meta_mut().at_lookup(lookup.name(), lookup.idx(), None);
        Ok(stmt)
    }
}

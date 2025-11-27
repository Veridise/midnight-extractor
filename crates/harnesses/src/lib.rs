pub mod arithmetic;
pub mod assertion;
pub mod assignment;

pub mod utils {
    //! Convenience functions for writing harness.

    pub fn vec_len_err<const N: usize, T>(e: Vec<T>) -> midnight_proofs::plonk::Error {
        midnight_proofs::plonk::Error::Synthesis(format!(
            "Failed to convert vec of {} elements to array of {N} elements",
            e.len()
        ))
    }

    use ff::PrimeField;

    use mdnt_extractor_core::lookups::callbacks::{
        ignore::IgnoreLookup, mux::LookupMux, plain_spread::PlainSpreadLookup,
        range::TagRangeLookup,
    };

    /// Returns a lookup callback that treats the lookup as a range check.
    pub fn range_lookup<F: PrimeField>(size: usize) -> TagRangeLookup<F, 1, 1> {
        let ranges = (0..=size)
            .map(|n| -> u64 { n.try_into().unwrap() })
            .map(|n| ([F::from(n)], [F::from(1 << n)]));
        TagRangeLookup::new([0], [1], ranges)
    }

    /// Returns a lookup callback that treats the lookup as a range check of specific bit lengths
    /// and the spreaded versions (i.e. 2->4, 3->5, 7->21, etc).
    ///
    /// Is meant for the lookup used by the Sha256Chip.
    pub fn plain_spread_lookup<F: PrimeField>(
        spread_module: &'static str,
        unspread_module: &'static str,
    ) -> PlainSpreadLookup<F> {
        PlainSpreadLookup::new(spread_module, unspread_module)
    }

    /// Returns a lookup callback that ignores all lookups
    #[allow(dead_code)]
    pub fn ignore_lookup() -> IgnoreLookup {
        IgnoreLookup
    }

    /// Creates an empty mux
    pub fn lookup_mux<'a, F: PrimeField>() -> LookupMux<'a, F> {
        Default::default()
    }
}

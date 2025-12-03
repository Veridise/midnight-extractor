use mdnt_extractor_core::harness::Entry;

pub mod arithmetic;
pub mod assertion;
pub mod assignment;
pub mod automaton;
pub mod base64;
pub mod base64var;
pub mod biguint;
pub mod binary;
pub mod bitwise;
pub mod canonicity;
pub mod committed_instance;
pub mod comparison;
pub mod control_flow;
pub mod conversion;
pub mod core_decomposition;
pub mod decomposition;
pub mod division;
pub mod ecc;
pub mod equality;
pub mod field;
pub mod foreign_ecc;
pub mod hash;
pub mod hash_to_curve;
pub mod map;
pub mod map_to_curve;
pub mod parser;
pub mod pow2range;
pub mod public_input;
pub mod range_check;
pub mod sponge;
pub mod stdlib;
pub mod unsafe_conversion;
pub mod varhash;
pub mod vector;
pub mod zero;

pub fn harnesses() -> impl Iterator<Item = &'static Entry> {
    inventory::iter::<Entry>()
}

pub mod utils {
    //! Convenience functions for writing harness.

    pub fn vec_len_err<const N: usize, T>(e: Vec<T>) -> midnight_proofs::plonk::Error {
        midnight_proofs::plonk::Error::Synthesis(format!(
            "Failed to convert vec of {} elements to array of {N} elements",
            e.len()
        ))
    }

    pub fn vec2array<T, const N: usize>(
        v: Vec<T>,
    ) -> Result<[T; N], midnight_proofs::plonk::Error> {
        v.try_into().map_err(vec_len_err::<N, T>)
    }

    use ff::PrimeField;

    use mdnt_extractor_core::lookups::callbacks::{
        automaton::AutomatonLookup, ignore::IgnoreLookup, mux::LookupMux,
        plain_spread::PlainSpreadLookup, range::TagRangeLookup,
    };
    use midnight_circuits::parsing::{automaton_chip::NativeAutomaton, spec_library};

    /// Returns a lookup callback that treats the lookup as a range check.
    pub fn range_lookup<F: PrimeField>(size: usize) -> TagRangeLookup<F, 1, 1> {
        let ranges = (0..=size)
            .map(|n| -> u64 { n.try_into().unwrap() })
            .map(|n| ([F::from(n)], [F::from(1 << n)]));
        TagRangeLookup::new([0], [1], ranges)
    }

    /// Returns an automaton lookup using the midnight parsing library.
    pub fn automaton<F: PrimeField + Ord>() -> AutomatonLookup<F> {
        let automata = spec_library();
        // The offset needs to start from 1 and not 0, to ensure that no automata will
        // use the state 0 (required by the automaton chip for soundness, since
        // 0 is used as a dummy state to encode some checks as fake
        // transitions).
        let mut offset = 1;
        let offset_automata = (automata.iter()).map(|(name, automaton)| {
            let na: NativeAutomaton<F> = automaton.offset_states(offset).into();
            offset += automaton.nb_states;
            (*name, na)
        });
        AutomatonLookup::new(offset_automata, 8)
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

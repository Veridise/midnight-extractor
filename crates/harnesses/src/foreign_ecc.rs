pub mod native {

    use crate::utils::range_lookup;
    use crate::utils::vec2array;
    use mdnt_extractor_core::entry as add_entry;
    use mdnt_extractor_core::fields::G1 as G;
    use mdnt_extractor_core::{
        cells::load::LoadedG1,
        chips::{Afp, Fecn},
    };
    use mdnt_extractor_macros::{entry, harness, unit_harness};
    use midnight_circuits::instructions::AssertionInstructions as _;
    use midnight_circuits::types::InnerConstants as _;
    use midnight_circuits::{midnight_proofs::plonk::Error, types::AssignedBit};
    use midnight_proofs::circuit::Value;

    pub type F = mdnt_extractor_core::fields::Blstrs;

    #[entry("foreign-ecc/assert_in_bls12_381_subgroup/foreign-ecc-native/point")]
    #[unit_harness(range_lookup(8))]
    pub fn assert_in_bls12_381_subgroup(
        chip: &Fecn<F, G>,
        layouter: &mut impl Layouter<F>,
        _: (),
        x: Afp<F, G>,
    ) -> Result<(), Error> {
        chip.assert_in_bls12_381_subgroup(layouter, &x)
    }

    add_entry!(
        "foreign-ecc/k_out_of_n_points_10_3/foreign-ecc-native/point",
        k_out_of_n_points_native::<10, 3>
    );
    add_entry!(
        "foreign-ecc/k_out_of_n_points_5_3/foreign-ecc-native/point",
        k_out_of_n_points_native::<5, 3>
    );
    #[harness(range_lookup(8))]
    pub fn k_out_of_n_points_native<const N: usize, const K: usize>(
        chip: &Fecn<F, G>,
        layouter: &mut impl Layouter<F>,
        (table, selected): ([Afp<F, G>; N], [LoadedG1; K]),
    ) -> Result<[Afp<F, G>; K], Error> {
        let selected = selected.into_iter().map(Into::into).map(Value::known).collect::<Vec<_>>();
        chip.k_out_of_n_points(layouter, &table, &selected).and_then(vec2array)
    }

    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<1, 8>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<5, 8>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<1, 16>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<5, 16>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<1, 64>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-native/point",
        msm_by_le_bits_native::<5, 64>
    );
    #[harness(range_lookup(8))]
    pub fn msm_by_le_bits_native<const N: usize, const BITS: usize>(
        chip: &Fecn<F, G>,
        layouter: &mut impl Layouter<F>,
        (scalars, bases): ([[AssignedBit<F>; BITS]; N], [Afp<F, G>; N]),
    ) -> Result<Afp<F, G>, Error> {
        let identity = Afp::<F, G>::inner_zero();
        for base in &bases {
            chip.assert_not_equal_to_fixed(layouter, base, identity)?;
        }
        let scalars = scalars.map(|a| a.to_vec());
        chip.msm_by_le_bits(layouter, &scalars, &bases)
    }
}

pub mod field {

    use crate::utils::range_lookup;
    use crate::utils::vec2array;
    use mdnt_extractor_core::entry as add_entry;
    use mdnt_extractor_core::fields::Secp256k1 as G;
    use mdnt_extractor_core::{
        cells::load::LoadedSecp256k1,
        chips::{Afp, Fecf},
    };
    use mdnt_extractor_macros::harness;
    use midnight_circuits::instructions::AssertionInstructions as _;
    use midnight_circuits::types::InnerConstants as _;
    use midnight_circuits::{midnight_proofs::plonk::Error, types::AssignedBit};
    use midnight_proofs::circuit::Value;

    pub type F = mdnt_extractor_core::fields::Blstrs;

    add_entry!(
        "foreign-ecc/k_out_of_n_points_10_3/foreign-ecc-field/point",
        k_out_of_n_points_field::<10, 3>
    );
    add_entry!(
        "foreign-ecc/k_out_of_n_points_5_3/foreign-ecc-field/point",
        k_out_of_n_points_field::<5, 3>
    );
    #[harness(range_lookup(8))]
    pub fn k_out_of_n_points_field<const N: usize, const K: usize>(
        chip: &Fecf<F, G>,
        layouter: &mut impl Layouter<F>,
        (table, selected): ([Afp<F, G>; N], [LoadedSecp256k1; K]),
    ) -> Result<[Afp<F, G>; K], Error> {
        let selected = selected.into_iter().map(Into::into).map(Value::known).collect::<Vec<_>>();
        chip.k_out_of_n_points(layouter, &table, &selected).and_then(vec2array)
    }

    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<1, 8>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<5, 8>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<1, 16>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<5, 16>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<1, 64>
    );
    add_entry!(
        "foreign-ecc/msm_by_le_bits/foreign-ecc-field/point",
        msm_by_le_bits_field::<5, 64>
    );
    #[harness(range_lookup(8))]
    pub fn msm_by_le_bits_field<const N: usize, const BITS: usize>(
        chip: &Fecf<F, G>,
        layouter: &mut impl Layouter<F>,
        (scalars, bases): ([[AssignedBit<F>; BITS]; N], [Afp<F, G>; N]),
    ) -> Result<Afp<F, G>, Error> {
        let identity = Afp::<F, G>::inner_zero();
        for base in &bases {
            chip.assert_not_equal_to_fixed(layouter, base, identity)?;
        }
        let scalars = scalars.map(|a| a.to_vec());
        chip.msm_by_le_bits(layouter, &scalars, &bases)
    }
}

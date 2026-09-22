//! Mass flux (base unit kilogram per square meter second, m⁻² · kg ·  s⁻¹).

quantity! {
    ///  Mass flux (base unit kilogram per square meter second, m⁻² · kg ·  s⁻¹).
    quantity: MassFlux; "mass flux";
    /// Dimension of mass flux, L⁻²MT⁻¹ (base unit kilogram per square meter second,
    /// m⁻² · kg ·  s⁻¹).
    dimension: ISQ<
        N2,     // length
        P1,     // mass
        N1,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_per_square_meter_second: prefix!(none); "kg/(m² · s)",
            "kilogram per square meter second", "kilograms per square meter second";
        @kilogram_per_square_centimeter_second: prefix!(none) / prefix!(centi) / prefix!(centi);
            "kg/(cm² · s)", "kilogram per square centimeter second",
            "kilograms per square centimeter second";
        @gram_per_square_centimeter_second: prefix!(milli) / prefix!(centi) / prefix!(centi);
            "g/(cm² · s)", "gram per square centimeter second",
            "grams per square centimeter second";
    }
}

#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::mass as m;
        use crate::si::mass_flux as mf;
        use crate::si::quantities::*;
        use crate::si::time as t;
        use crate::si::area as a;
        use crate::tests::Test;
        use crate::num::FromPrimitive;

        #[test]
        fn manual_peace_of_mind_conversion_test() {
            // 1 kg/(cm²·s) == 10_000 kg/(m²·s)
            Test::assert_approx_eq(
                &MassFlux::new::<mf::kilogram_per_square_centimeter_second>(V::one())
                    .get::<mf::kilogram_per_square_meter_second>(),
                &V::from_f64(1.0_E4).unwrap(),
            );

             Test::assert_approx_eq(
                &MassFlux::new::<mf::kilogram_per_square_meter_second>(V::one())
                    .get::<mf::kilogram_per_square_centimeter_second>(),
                &V::from_f64(0.0001).unwrap(),
            );

            // 1 g/(cm²·s) == 10 kg/(m²·s)
            Test::assert_approx_eq(
                &MassFlux::new::<mf::gram_per_square_centimeter_second>(V::one())
                    .get::<mf::kilogram_per_square_meter_second>(),
                &V::from_f64(1.0_E1).unwrap(),
            );

            // 1 kg/(cm²·s) == 1_000 g/(cm²·s)
            Test::assert_approx_eq(
                &MassFlux::new::<mf::kilogram_per_square_centimeter_second>(V::one())
                    .get::<mf::gram_per_square_centimeter_second>(),
                &V::from_f64(1.0_E3).unwrap(),
            );
        }

        #[test]
        fn check_dimension() {
            let _: MassFlux<V> = Mass::new::<m::kilogram>(V::one())
                / Time::new::<t::second>(V::one())
                / Area::new::<a::square_meter>(V::one());
        }

        #[test]
        fn check_units() {
            test::<m::kilogram, a::square_meter, t::second, mf::kilogram_per_square_meter_second>();
            test::<m::gram, a::square_centimeter, t::second,
                mf::gram_per_square_centimeter_second>();
            test::<m::kilogram, a::square_centimeter, t::second,
                mf::kilogram_per_square_centimeter_second>();

            fn test<M: m::Conversion<V>,
                A: a::Conversion<V>,
                T: t::Conversion<V>,
                MF: mf::Conversion<V>>()
            {
                Test::assert_approx_eq(&MassFlux::new::<MF>(V::one()),
                    &(Mass::new::<M>(V::one())
                        / Time::new::<T>(V::one())
                        / Area::new::<A>(V::one())));
            }
        }
    }
}

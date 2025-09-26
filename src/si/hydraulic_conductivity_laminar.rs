//! Laminar hydraulic conductivity (base unit kg/(s·m⁴)).


quantity! {
    /// Hydraulic conductivity laminar (base unit kg/(s·m⁴)).
    quantity: HydraulicConductivityLaminar; "laminar hydraulic conductivity";
    /// Dimension of laminar hydraulic conductivity, L⁻⁴M¹T⁻¹ (base unit kg/(s·m⁴)).
    dimension: ISQ<
        N4,     // length (negative 4)
        P1,     // mass (positive 1)
        N1,     // time (negative 1)
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @kilogram_per_second_meter_to_the_fourth: prefix!(none); "kg/(s·m⁴)", 
            "kilogram per second-meter to the fourth power", "kilograms per second-meter to the fourth power";
        @gram_per_second_meter_to_the_fourth: prefix!(none) / prefix!(kilo); "g/(s·m⁴)",
            "gram per second-meter to the fourth power", "grams per second-meter to the fourth power";
    }
}


#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::hydraulic_conductivity_laminar as hcl;
        use crate::si::mass as m;
        use crate::si::time as t;
        use crate::si::length as l;
        use crate::si::area as a;
        use crate::si::quantities::*;
        use crate::tests::Test;
        
        #[test]
        fn check_dimension() {
            let _: HydraulicConductivityLaminar<V> = (
                Mass::new::<m::kilogram>(V::one()) 
                / (Time::new::<t::second>(V::one()) * Area::new::<a::square_meter>(V::one()) * Area::new::<a::square_meter>(V::one()))
            ).into();
        }
        
        #[test]
        fn check_units() {
            test::<m::kilogram, t::second, l::meter, hcl::kilogram_per_second_meter_to_the_fourth>();
            test::<m::gram, t::second, l::meter, hcl::gram_per_second_meter_to_the_fourth>();
            
            fn test<M: m::Conversion<V>, T: t::Conversion<V>, L: l::Conversion<V>, HCL: hcl::Conversion<V>>() {
                Test::assert_approx_eq(&HydraulicConductivityLaminar::new::<HCL>(V::one()),
                    &(
                        Mass::new::<M>(V::one()) 
                        / (Time::new::<T>(V::one()) * Area::new::<a::square_meter>(V::one()) * Area::new::<a::square_meter>(V::one()))
                    ).into());
            }
        }
        #[test]
        fn check_known_conversions() {
            // 1 kg/(s·m⁴) = 1000 g/(s·m⁴)
            Test::assert_approx_eq(&HydraulicConductivityLaminar::new::<hcl::kilogram_per_second_meter_to_the_fourth>(V::one()),
                &HydraulicConductivityLaminar::new::<hcl::gram_per_second_meter_to_the_fourth>(1000.0));
        }
    }
}
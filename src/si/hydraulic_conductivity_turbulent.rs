//! Turbulent hydraulic conductivity (base unit kg/m⁷).

quantity! {
    /// Hydraulic conductivity for turbulent flow (base unit kg/m⁷).
    quantity: HydraulicConductivityTurbulent; "turbulent hydraulic conductivity";
    /// Dimension of turbulent hydraulic conductivity, L⁻⁷M¹ (base unit kg/m⁷).
    dimension: ISQ<
        N7,     // length (negative 7)
        P1,     // mass (positive 1)
        Z0,     // time (zero)
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        /// Base unit of turbulent hydraulic conductivity.
        @kilogram_per_meter_to_the_seventh: prefix!(none); "kg/m⁷", 
            "kilogram per meter to the seventh power", "kilograms per meter to the seventh power";
        @gram_per_meter_to_the_seventh: prefix!(none) / prefix!(kilo); "g/m⁷",
            "gram per meter to the seventh power", "grams per meter to the seventh power";
        // You can add more units here as needed
    }
}


#[cfg(test)]
mod test {
    storage_types! {
        use crate::num::One;
        use crate::si::hydraulic_conductivity_turbulent as hct;
        use crate::si::mass as m;
        use crate::si::length as l;
        use crate::si::volume as v;
        use crate::si::quantities::*;
        use crate::tests::Test;
        
        #[test]
        fn check_dimension() {
            let _: HydraulicConductivityTurbulent<V> = (
                Mass::new::<m::kilogram>(V::one()) 

                / 
                // two volumes and a length = 7, since we cant use powi
                (Volume::new::<v::cubic_meter>(V::one()) * Volume::new::<v::cubic_meter>(V::one()) * Length::new::<l::meter>(V::one()))
            ).into();
        }
        
        #[test]
        fn check_units() {
            test::<m::kilogram, l::meter, hct::kilogram_per_meter_to_the_seventh>();
            test::<m::gram, l::meter, hct::gram_per_meter_to_the_seventh>();
            
            fn test<M: m::Conversion<V>, L: l::Conversion<V>, HCT: hct::Conversion<V>>() {
                Test::assert_approx_eq(&HydraulicConductivityTurbulent::new::<HCT>(V::one()),
                    &(
                        Mass::new::<M>(V::one()) 
                        / (Volume::new::<v::cubic_meter>(V::one()) * Volume::new::<v::cubic_meter>(V::one()) * Length::new::<L>(V::one()))
                    ).into());
            }
        }
            #[test]
    fn check_known_conversions() {
        // 1 kg/m⁷ = 1000 g/m⁷
        Test::assert_approx_eq(&HydraulicConductivityTurbulent::new::<hct::kilogram_per_meter_to_the_seventh>(V::one()),
            &HydraulicConductivityTurbulent::new::<hct::gram_per_meter_to_the_seventh>(1000.0));
                
    }
    }


        
}
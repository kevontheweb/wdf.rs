pub mod resistor {
    use crate::components::base_wdf::base_wdf::BaseWDF;
    use crate::components::root_wdf::root_wdf::RootWDF;
    use crate::components::WDFComponent;
    use std::cell::RefCell;
    use std::rc::Rc;
    // Define the Resistor component
    pub struct Resistor {
        base: BaseWDF,
        resistance: f64,
    }

    impl Resistor {
        pub fn new(resistance: f64) -> Self {
            let mut resistor = Resistor {
                base: BaseWDF {
                    rp: resistance,
                    ..Default::default()
                },
                resistance,
            };
            resistor.calc_impedance();
            resistor
        }

        pub fn set_resistance(&mut self, resistance: f64) {
            if self.base.rp != resistance {
                self.base.rp = resistance;
                self.resistance = resistance;
                self.base.update_impedance();
            }
        }
    }

    impl WDFComponent for Resistor {
        fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
            todo!();
        }

        fn accept_incident_wave(&mut self, a: f64) {
            self.base.accept_incident_wave(a);
        }

        fn propagate_reflected_wave(&mut self) -> f64 {
            self.base.b = 0.0;
            self.base.b
        }

        fn calc_impedance(&mut self) {
            self.base.g = 1.0 / self.base.rp;
        }

        fn update_impedance(&mut self) {
            self.base.update_impedance();
        }

        fn reset(&mut self) {
            self.base.reset();
        }

        fn get_reflected_wave(&self) -> f64 {
            self.base.get_reflected_wave()
        }

        fn get_port_impedance(&self) -> f64 {
            self.base.get_port_impedance()
        }

        fn get_port_admittance(&self) -> f64 {
            self.base.get_port_admittance()
        }

        fn print_type(&self) {
            println!("Resistor");
        }
    }
}

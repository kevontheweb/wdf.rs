pub mod sources {
    use crate::components::BaseWDF;
    use crate::components::RootWDF;
    use crate::components::WDFComponent;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Define the IdealVoltageSource component
    pub struct IdealVoltageSource {
        pub base: RootWDF,
        next: Option<Rc<RefCell<dyn WDFComponent>>>,
        v: f64,
    }

    impl IdealVoltageSource {
        pub fn new(next: Option<Rc<RefCell<dyn WDFComponent>>>) -> Self {
            let mut source = IdealVoltageSource { next: next, v: 0.0 };
            source.calc_impedance();
            source
        }

        pub fn set_voltage(&mut self, voltage: f64) {
            if self.v != voltage {
                self.v = voltage;
            }
        }
    }

    impl WDFComponent for IdealVoltageSource {
        fn accept_incident_wave(&mut self, a: f64) {
            self.a = a;
        }

        fn propagate_reflected_wave(&mut self) -> f64 {
            self.b = -self.a + 2.0 * self.v;
            self.b
        }

        fn calc_impedance(&mut self) {
            unimplemented!();
            // this.Rp = this.Rp;
            // this.G = 1.0 / this.Rp;
        }

        fn update_impedance(&mut self) {
            self.calc_impedance();
        }

        fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {}

        fn reset(&mut self) {
            todo!();
        }

        fn print_type(&self) {
            println!("IdealVoltageSource")
        }

        fn get_port_admittance(&self) -> f64 {
            self.g
        }

        fn get_port_impedance(&self) -> f64 {
            self.rp
        }

        fn get_reflected_wave(&self) -> f64 {
            self.b
        }

        fn get_current_from_wave(&self) -> f64 {
            self.base.get_current_from_wave()
        }

        fn get_voltage_from_wave(&self) -> f64 {
            self.base.get_voltage_from_wave()
        }
    }
}

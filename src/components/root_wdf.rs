pub mod root_wdf {
    use crate::components::BaseWDF;
    use crate::components::WDFComponent;
    use std::cell::RefCell;
    use std::rc::Rc;
    // Define the RootWDF component
    #[derive(Default)]
    pub struct RootWDF {
        base: BaseWDF,
        pub next: Option<Rc<RefCell<dyn WDFComponent>>>,
    }

    impl RootWDF {
        pub fn new(next: Rc<RefCell<dyn WDFComponent>>) -> Self {
            let root_wdf = RootWDF {
                base: BaseWDF::default(),
                next: Some(next.clone()),
            };
            root_wdf.connect_to_parent();
            root_wdf
        }

        pub fn connect_to_parent(&self) {
            panic!("Root elements cannot be connected to a parent!");
        }
    }

    impl WDFComponent for RootWDF {
        fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
            panic!("Root elements cannot be connected to a parent!");
        }

        fn accept_incident_wave(&mut self, a: f64) {
            self.base.accept_incident_wave(a);
        }

        fn propagate_reflected_wave(&mut self) -> f64 {
            if let Some(next) = &self.next {
                let mut next_ref = next.borrow_mut();
                next_ref.propagate_reflected_wave()
            } else {
                0.0
            }
        }

        fn calc_impedance(&mut self) {
            self.base.calc_impedance();
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
            println!("RootWDF");
        }

        fn get_current_from_wave(&self) -> f64 {
            self.base.get_current_from_wave()
        }

        fn get_voltage_from_wave(&self) -> f64 {
            self.base.get_voltage_from_wave()
        }
    }
}

pub mod base_wdf {
    use crate::components::root_wdf::root_wdf::RootWDF;
    use crate::components::WDFComponent;
    use std::cell::RefCell;
    use std::rc::Rc;

    // Define the base component with common functionality
    #[derive(Default)]
    pub struct BaseWDF {
        pub a: f64,
        pub b: f64,
        pub rp: f64,
        pub g: f64,
        pub parent: Option<Rc<RefCell<dyn WDFComponent>>>,
    }

    impl BaseWDF {
        fn update_impedance_common(&mut self) {
            self.calc_impedance();
            if let Some(parent) = &self.parent {
                let mut parent_ref = parent.borrow_mut();
                parent_ref.update_impedance();
            }
        }
    }

    impl WDFComponent for BaseWDF {
        fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
            todo!();
        }

        fn accept_incident_wave(&mut self, a: f64) {
            self.a = a;
        }

        fn propagate_reflected_wave(&mut self) -> f64 {
            unimplemented!()
        }

        fn calc_impedance(&mut self) {
            unimplemented!()
        }

        fn update_impedance(&mut self) {
            self.update_impedance_common();
        }

        fn reset(&mut self) {
            self.a = 0.0;
            self.b = 0.0;
        }

        fn get_reflected_wave(&self) -> f64 {
            self.b
        }
        fn get_port_impedance(&self) -> f64 {
            self.rp
        }

        fn get_port_admittance(&self) -> f64 {
            self.g
        }

        fn print_type(&self) {
            println!("BaseWDF");
        }

        fn get_current_from_wave(&self) -> f64 {
            (self.a + self.b) / (self.rp * 2.0)
        }

        fn get_voltage_from_wave(&self) -> f64 {
            self.a + self.b / 2.0
        }
    }
}

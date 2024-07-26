pub mod parallel_adaptor{
use crate::components::base_wdf::base_wdf::BaseWDF;
use crate::components::root_wdf::root_wdf::RootWDF;
use crate::components::WDFComponent;
use std::cell::RefCell;
use std::rc::Rc;


pub struct ParallelAdaptor {
    base: BaseWDF,
    child_a: Rc<RefCell<dyn WDFComponent>>,
    child_b: Rc<RefCell<dyn WDFComponent>>,
    b_temp: f64,
    b_diff: f64,
    b2: f64,
    b_child_a: f64,
}

impl ParallelAdaptor {
    pub fn new(
        &self,
        child_a: Rc<RefCell<dyn WDFComponent>>,
        child_b: Rc<RefCell<dyn WDFComponent>>,
    ) -> Self {
        let mut parallel = ParallelAdaptor {
            base: BaseWDF {
                ..Default::default()
            },
            child_a,
            child_b,
            b_diff: 0.0,
            b_temp: 0.0,
            b2: 0.0,
            b_child_a: 1.0,
        };
        self.child_a
            .borrow_mut()
            .connect_to_parent(self.base.parent.to_owned());
        self.child_b
            .borrow_mut()
            .connect_to_parent(self.base.parent.to_owned());
        parallel.calc_impedance();
        parallel
    }
}

impl WDFComponent for ParallelAdaptor {
    fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
        todo!();
    }

    fn accept_incident_wave(&mut self, a: f64) {
        self.b2 = a + self.b_temp;
        self.base.rp = 1.0 / self.base.g;
        self.b_child_a = self.child_a.borrow().get_port_admittance() / self.base.g;
    }

    fn propagate_reflected_wave(&mut self) -> f64 {
        self.b_diff = self.child_b.borrow_mut().propagate_reflected_wave()
            - self.child_a.borrow_mut().propagate_reflected_wave();
        self.b_temp = self.b_child_a * self.b_diff;
        self.base.b = self.child_b.borrow().get_reflected_wave() + self.b_temp;
        self.base.b
    }

    fn calc_impedance(&mut self) {
        self.base.rp = self.child_a.borrow().get_port_admittance()
            + self.child_b.borrow().get_port_admittance();
        self.base.g = self.child_a.borrow().get_port_admittance() / self.base.g;
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
        println!("ParallelAdaptor");
    }
}
}
pub mod series_adaptor{
use crate::components::base_wdf::base_wdf::BaseWDF;
use crate::components::root_wdf::root_wdf::RootWDF;
use crate::components::WDFComponent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SeriesAdaptor {
    base: BaseWDF,
    child_a: Rc<RefCell<dyn WDFComponent>>,
    child_b: Rc<RefCell<dyn WDFComponent>>,
    b_temp: f64,
    b_child_a: f64,
}

impl SeriesAdaptor {
    pub fn new(
        &self,
        child_a: Rc<RefCell<dyn WDFComponent>>,
        child_b: Rc<RefCell<dyn WDFComponent>>,
    ) -> Self {
        let mut series = SeriesAdaptor {
            base: BaseWDF {
                ..Default::default()
            },
            child_a,
            child_b,
            b_temp: 0.0,
            b_child_a: 1.0,
        };
        self.child_a
            .borrow_mut()
            .connect_to_parent(self.base.parent.to_owned());
        self.child_b
            .borrow_mut()
            .connect_to_parent(self.base.parent.to_owned());
        series.calc_impedance();
        series
    }
}

impl WDFComponent for SeriesAdaptor {
    fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
        todo!();
    }

    fn accept_incident_wave(&mut self, a: f64) {
        self.b_temp = self.child_a.borrow().get_port_impedance()
            - self.b_child_a
                * (a + self.child_a.borrow().get_reflected_wave()
                    + self.child_b.borrow().get_reflected_wave());
        self.child_a.borrow_mut().accept_incident_wave(self.b_temp);
        self.child_b
            .borrow_mut()
            .accept_incident_wave(a + self.b_temp);
        self.base.a = a;
    }

    fn propagate_reflected_wave(&mut self) -> f64 {
        self.base.b = -(self.child_a.borrow_mut().propagate_reflected_wave()
            + self.child_b.borrow_mut().propagate_reflected_wave());
        self.base.b
    }

    fn calc_impedance(&mut self) {
        self.base.rp =
            self.child_a.borrow().get_port_impedance() + self.child_b.borrow().get_port_impedance();
        self.base.g = 1.0 / self.base.rp;
        self.b_child_a = self.child_a.borrow().get_port_impedance() / self.base.rp;
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
        println!("SeriesAdaptor");
    }
}
}
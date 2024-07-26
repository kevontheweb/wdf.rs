pub mod capacitor{
use crate::components::base_wdf::base_wdf::BaseWDF;
use crate::components::root_wdf::root_wdf::RootWDF;
use crate::components::WDFComponent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Capacitor {
    base: BaseWDF,
    memory: f64,
    capacitance: f64,
    sample_rate: u16,
}

impl Capacitor {
    pub fn new(capacitance: f64, sample_rate: u16) -> Self {
        let mut capacitor = Capacitor {
            base: BaseWDF {
                rp: 1.0 / (2.0 * capacitance * (sample_rate as f64)),
                ..Default::default()
            },
            memory: 0.0,
            capacitance: capacitance,
            sample_rate: sample_rate,
        };
        capacitor.calc_impedance();
        capacitor
    }

    pub fn set_capacitance(&mut self, capacitance: f64) {
        if self.base.rp != capacitance {
            self.base.rp = capacitance;
            self.capacitance = capacitance;
            self.base.update_impedance();
        }
    }
}

impl WDFComponent for Capacitor {
    fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>) {
        todo!();
    }

    fn accept_incident_wave(&mut self, a: f64) {
        self.base.a = a;
        self.memory = a;
    }

    fn propagate_reflected_wave(&mut self) -> f64 {
        self.base.b = self.memory;
        self.base.b
    }

    fn calc_impedance(&mut self) {
        self.base.rp = 1.0 / (2.0 * self.capacitance * (self.sample_rate as f64));
        self.base.g = 1.0 / self.base.rp;
    }

    fn update_impedance(&mut self) {
        self.base.update_impedance();
    }

    fn reset(&mut self) {
        self.base.reset();
        self.memory = 0.0;
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
        println!("Capacitor");
    }
}
}
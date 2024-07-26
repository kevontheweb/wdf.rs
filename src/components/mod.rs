pub mod base_wdf;
pub mod capacitor;
pub mod parallel_adaptor;
pub mod resistor;
pub mod root_wdf;
pub mod series_adaptor;
pub mod sources;

use crate::components::base_wdf::base_wdf::BaseWDF;
use crate::components::capacitor::capacitor::Capacitor;
use crate::components::parallel_adaptor::parallel_adaptor::ParallelAdaptor;
use crate::components::resistor::resistor::Resistor;
use crate::components::root_wdf::root_wdf::RootWDF;
use crate::components::series_adaptor::series_adaptor::SeriesAdaptor;

use std::cell::RefCell;
use std::rc::Rc;

// Define the trait for WDF components
pub trait WDFComponent {
    fn connect_to_parent(&mut self, parent: Option<Rc<RefCell<dyn WDFComponent>>>);
    fn accept_incident_wave(&mut self, a: f64);
    fn propagate_reflected_wave(&mut self) -> f64;
    fn calc_impedance(&mut self);
    fn update_impedance(&mut self);
    fn reset(&mut self);
    fn print_type(&self);
    fn get_port_admittance(&self) -> f64;
    fn get_port_impedance(&self) -> f64;
    fn get_reflected_wave(&self) -> f64;
    fn get_voltage_from_wave(&self) -> f64;
    fn get_current_from_wave(&self) -> f64;
}

// Define the enum to represent different WDF components
pub enum WDFComponentEnum {
    Base(BaseWDF),
    Root(RootWDF),
    Resistor(Resistor),
    Capacitor(Capacitor),
    SeriesAdaptor(SeriesAdaptor),
    ParallelAdaptor(ParallelAdaptor),
}

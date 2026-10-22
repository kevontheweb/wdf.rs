use crate::components::{BaseWDF, WDFComponent};

/// Polarity inverter: flips the sign of the voltage and current seen by the
/// parent while keeping the port impedance of its child.
#[derive(Debug, Clone)]
pub struct Inverter<C: WDFComponent> {
    base: BaseWDF,
    pub child: C,
}

impl<C: WDFComponent> Inverter<C> {
    pub fn new(child: C) -> Self {
        let mut inverter = Inverter {
            base: BaseWDF::default(),
            child,
        };
        inverter.calc_impedance();
        inverter
    }
}

impl<C: WDFComponent> WDFComponent for Inverter<C> {
    fn port_impedance(&self) -> f64 {
        self.base.rp
    }

    fn calc_impedance(&mut self) {
        self.child.calc_impedance();
        self.base.set_impedance(self.child.port_impedance());
    }

    fn incident(&mut self, a: f64) {
        self.base.a = a;
        self.child.incident(-a);
    }

    fn reflected(&mut self) -> f64 {
        self.base.b = -self.child.reflected();
        self.base.b
    }

    fn reflected_wave(&self) -> f64 {
        self.base.b
    }

    fn voltage(&self) -> f64 {
        self.base.voltage()
    }

    fn current(&self) -> f64 {
        self.base.current()
    }

    fn reset(&mut self) {
        self.base.reset();
        self.child.reset();
    }
}

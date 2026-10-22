use crate::components::WDFComponent;

/// Ideal voltage source acting as the root of a WDF tree.
///
/// A root is not adapted, so it is not itself a `WDFComponent`: it owns the
/// subtree below it and drives one wave exchange per sample.
#[derive(Debug, Clone)]
pub struct IdealVoltageSource<C: WDFComponent> {
    pub child: C,
    v: f64,
    /// Wave incident on the source (reflected by the child)
    a: f64,
    /// Wave reflected by the source (incident on the child)
    b: f64,
}

impl<C: WDFComponent> IdealVoltageSource<C> {
    pub fn new(child: C) -> Self {
        IdealVoltageSource {
            child,
            v: 0.0,
            a: 0.0,
            b: 0.0,
        }
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.v = voltage;
    }

    pub fn voltage(&self) -> f64 {
        self.v
    }

    /// Run one wave exchange: gather the child's reflected wave, reflect it
    /// off the source (`b = 2v - a`) and send it back down the tree.
    pub fn process(&mut self) {
        self.a = self.child.reflected();
        self.b = 2.0 * self.v - self.a;
        self.child.incident(self.b);
    }

    pub fn calc_impedance(&mut self) {
        self.child.calc_impedance();
    }

    pub fn reset(&mut self) {
        self.a = 0.0;
        self.b = 0.0;
        self.child.reset();
    }
}

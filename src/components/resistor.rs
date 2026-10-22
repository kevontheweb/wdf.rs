use crate::components::{BaseWDF, WDFComponent};

/// Linear resistor. Reflects nothing: `b = 0`.
#[derive(Debug, Clone)]
pub struct Resistor {
    base: BaseWDF,
    resistance: f64,
}

impl Resistor {
    pub fn new(resistance: f64) -> Self {
        assert!(resistance > 0.0, "resistance must be positive");
        Resistor {
            base: BaseWDF::new(resistance),
            resistance,
        }
    }

    /// Store a new resistance. Call `calc_impedance()` on the top of the tree
    /// afterwards so parent adaptors pick it up.
    pub fn set_resistance(&mut self, resistance: f64) {
        assert!(resistance > 0.0, "resistance must be positive");
        self.resistance = resistance;
    }

    pub fn resistance(&self) -> f64 {
        self.resistance
    }
}

impl WDFComponent for Resistor {
    fn port_impedance(&self) -> f64 {
        self.base.rp
    }

    fn calc_impedance(&mut self) {
        self.base.set_impedance(self.resistance);
    }

    fn incident(&mut self, a: f64) {
        self.base.a = a;
    }

    fn reflected(&mut self) -> f64 {
        self.base.b = 0.0;
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
    }
}

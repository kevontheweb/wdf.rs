use crate::components::{BaseWDF, WDFComponent};

/// Capacitor discretised with the trapezoidal rule (bilinear transform).
///
/// Port impedance is `1 / (2 * C * fs)`; the reflected wave is the incident
/// wave from the previous sample.
#[derive(Debug, Clone)]
pub struct Capacitor {
    base: BaseWDF,
    /// z^-1 state: the incident wave from the previous sample
    memory: f64,
    capacitance: f64,
    sample_rate: f64,
}

impl Capacitor {
    pub fn new(capacitance: f64, sample_rate: f64) -> Self {
        assert!(capacitance > 0.0, "capacitance must be positive");
        assert!(sample_rate > 0.0, "sample rate must be positive");
        let mut capacitor = Capacitor {
            base: BaseWDF::default(),
            memory: 0.0,
            capacitance,
            sample_rate,
        };
        capacitor.calc_impedance();
        capacitor
    }

    /// Store a new capacitance. Call `calc_impedance()` on the top of the tree
    /// afterwards so parent adaptors pick it up.
    pub fn set_capacitance(&mut self, capacitance: f64) {
        assert!(capacitance > 0.0, "capacitance must be positive");
        self.capacitance = capacitance;
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        assert!(sample_rate > 0.0, "sample rate must be positive");
        self.sample_rate = sample_rate;
    }

    pub fn capacitance(&self) -> f64 {
        self.capacitance
    }
}

impl WDFComponent for Capacitor {
    fn port_impedance(&self) -> f64 {
        self.base.rp
    }

    fn calc_impedance(&mut self) {
        self.base
            .set_impedance(1.0 / (2.0 * self.capacitance * self.sample_rate));
    }

    fn incident(&mut self, a: f64) {
        self.base.a = a;
        self.memory = a;
    }

    fn reflected(&mut self) -> f64 {
        self.base.b = self.memory;
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
        self.memory = 0.0;
    }
}

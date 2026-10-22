/// Port state shared by every WDF element.
///
/// Wave conventions (voltage waves, as in chowdsp_wdf):
///
/// * `a` is the wave incident *into* the element, `b` the wave reflected *out of* it
/// * `v = (a + b) / 2`
/// * `i = (a - b) / (2 * rp)`
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct BaseWDF {
    /// Incident wave
    pub a: f64,
    /// Reflected wave
    pub b: f64,
    /// Port impedance (ohms)
    pub rp: f64,
    /// Port admittance (siemens), always `1 / rp`
    pub g: f64,
}

impl BaseWDF {
    pub fn new(rp: f64) -> Self {
        let mut base = BaseWDF::default();
        base.set_impedance(rp);
        base
    }

    /// Set the port impedance and keep the admittance in sync.
    pub fn set_impedance(&mut self, rp: f64) {
        self.rp = rp;
        self.g = 1.0 / rp;
    }

    pub fn reset(&mut self) {
        self.a = 0.0;
        self.b = 0.0;
    }

    pub fn voltage(&self) -> f64 {
        0.5 * (self.a + self.b)
    }

    pub fn current(&self) -> f64 {
        (self.a - self.b) / (2.0 * self.rp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voltage_and_current_from_waves() {
        let mut p = BaseWDF::new(100.0);
        p.a = 3.0;
        p.b = 1.0;
        assert_eq!(p.voltage(), 2.0);
        assert_eq!(p.current(), 2.0 / 200.0);
        assert_eq!(p.g, 0.01);
    }

    #[test]
    fn reset_clears_waves_but_not_impedance() {
        let mut p = BaseWDF::new(50.0);
        p.a = 1.0;
        p.b = 2.0;
        p.reset();
        assert_eq!((p.a, p.b, p.rp), (0.0, 0.0, 50.0));
    }
}

use crate::components::{BaseWDF, WDFComponent};

/// Two-port series adaptor, adapted at its upward-facing port.
///
/// `Rp = R_a + R_b`. Both children are owned and reachable through the public
/// `child_a` / `child_b` fields.
#[derive(Debug, Clone)]
pub struct SeriesAdaptor<A: WDFComponent, B: WDFComponent> {
    base: BaseWDF,
    pub child_a: A,
    pub child_b: B,
    /// `R_a / (R_a + R_b)`
    a_reflect: f64,
}

impl<A: WDFComponent, B: WDFComponent> SeriesAdaptor<A, B> {
    pub fn new(child_a: A, child_b: B) -> Self {
        let mut series = SeriesAdaptor {
            base: BaseWDF::default(),
            child_a,
            child_b,
            a_reflect: 1.0,
        };
        series.calc_impedance();
        series
    }
}

impl<A: WDFComponent, B: WDFComponent> WDFComponent for SeriesAdaptor<A, B> {
    fn port_impedance(&self) -> f64 {
        self.base.rp
    }

    fn calc_impedance(&mut self) {
        self.child_a.calc_impedance();
        self.child_b.calc_impedance();
        let ra = self.child_a.port_impedance();
        let rb = self.child_b.port_impedance();
        self.base.set_impedance(ra + rb);
        self.a_reflect = ra / self.base.rp;
    }

    fn incident(&mut self, a: f64) {
        let b_a = self.child_a.reflected_wave();
        let b_b = self.child_b.reflected_wave();
        let b1 = b_a - self.a_reflect * (a + b_a + b_b);
        self.child_a.incident(b1);
        self.child_b.incident(-(a + b1));
        self.base.a = a;
    }

    fn reflected(&mut self) -> f64 {
        self.base.b = -(self.child_a.reflected() + self.child_b.reflected());
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
        self.child_a.reset();
        self.child_b.reset();
    }
}

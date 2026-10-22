use crate::components::{BaseWDF, WDFComponent};

/// Two-port parallel adaptor, adapted at its upward-facing port.
///
/// `G = G_a + G_b`, `Rp = 1 / G`. Both children are owned and reachable
/// through the public `child_a` / `child_b` fields.
#[derive(Debug, Clone)]
pub struct ParallelAdaptor<A: WDFComponent, B: WDFComponent> {
    base: BaseWDF,
    pub child_a: A,
    pub child_b: B,
    /// `G_a / (G_a + G_b)`
    a_reflect: f64,
    /// `b_0 - b_b` from the last `reflected()` call
    b_temp: f64,
    /// `b_b - b_a` from the last `reflected()` call
    b_diff: f64,
}

impl<A: WDFComponent, B: WDFComponent> ParallelAdaptor<A, B> {
    pub fn new(child_a: A, child_b: B) -> Self {
        let mut parallel = ParallelAdaptor {
            base: BaseWDF::default(),
            child_a,
            child_b,
            a_reflect: 1.0,
            b_temp: 0.0,
            b_diff: 0.0,
        };
        parallel.calc_impedance();
        parallel
    }
}

impl<A: WDFComponent, B: WDFComponent> WDFComponent for ParallelAdaptor<A, B> {
    fn port_impedance(&self) -> f64 {
        self.base.rp
    }

    fn calc_impedance(&mut self) {
        self.child_a.calc_impedance();
        self.child_b.calc_impedance();
        let ga = self.child_a.port_admittance();
        let gb = self.child_b.port_admittance();
        self.base.set_impedance(1.0 / (ga + gb));
        self.a_reflect = ga / (ga + gb);
    }

    fn incident(&mut self, a: f64) {
        // outgoing wave on each child port is `a + b_0 - b_child`
        let b2 = a + self.b_temp;
        self.child_a.incident(b2 + self.b_diff);
        self.child_b.incident(b2);
        self.base.a = a;
    }

    fn reflected(&mut self) -> f64 {
        let b_a = self.child_a.reflected();
        let b_b = self.child_b.reflected();
        self.b_diff = b_b - b_a;
        self.b_temp = -self.a_reflect * self.b_diff;
        self.base.b = b_b + self.b_temp;
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
        self.b_temp = 0.0;
        self.b_diff = 0.0;
        self.child_a.reset();
        self.child_b.reset();
    }
}

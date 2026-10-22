use crate::components::{IdealVoltageSource, WDFComponent};

/// A voltage-driven WDF circuit plus a probe that reads the output voltage.
///
/// ```ignore
/// let mut circuit = Circuit::new(source, |tree| tree.child_b.voltage());
/// let out = circuit.process_signal(&input);
/// ```
pub struct Circuit<C: WDFComponent, F: Fn(&C) -> f64> {
    source: IdealVoltageSource<C>,
    output: F,
}

impl<C: WDFComponent, F: Fn(&C) -> f64> Circuit<C, F> {
    /// `output` picks the voltage to return from the tree below the source.
    pub fn new(source: IdealVoltageSource<C>, output: F) -> Self {
        let mut circuit = Circuit { source, output };
        circuit.update_impedance();
        circuit
    }

    pub fn process_sample(&mut self, sample: f64) -> f64 {
        self.source.set_voltage(sample);
        self.source.process();
        (self.output)(&self.source.child)
    }

    /// Process `input` into the pre-allocated `output` slice. Does not allocate.
    pub fn process_block(&mut self, input: &[f64], output: &mut [f64]) {
        assert_eq!(input.len(), output.len(), "input and output lengths differ");
        for (x, y) in input.iter().zip(output.iter_mut()) {
            *y = self.process_sample(*x);
        }
    }

    pub fn process_signal(&mut self, input_signal: &[f64]) -> Vec<f64> {
        let mut output = vec![0.0; input_signal.len()];
        self.process_block(input_signal, &mut output);
        output
    }

    /// Recompute all port impedances. Call after changing a component value.
    pub fn update_impedance(&mut self) {
        self.source.calc_impedance();
    }

    pub fn reset(&mut self) {
        self.source.reset();
    }

    /// Access the tree to change component values, then call `update_impedance`.
    pub fn tree_mut(&mut self) -> &mut C {
        &mut self.source.child
    }
}

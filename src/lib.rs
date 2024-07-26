pub mod components;
pub mod circuit;
use crate::components::circuit::Circuit;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn RCLowPassFilter() {

let c = 1e-6;
let r = 1.0 / (2.0 * constants::PI * c * fc);
let r1 = Resistor::new(r);
let c1 = Capacitor::new(c, constants::SAMPLE_RATE);
let s1 = SeriesAdaptor::new(&r1, &c1);
let i1 = Inverter::new(&s1);
let vs = IdealVoltageSource::new(&i1);
let rc = Circuit::new(&vs, &vs, &c1);
let output = rc.process_signal(input_signal);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

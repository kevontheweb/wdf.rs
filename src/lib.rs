pub mod circuit;
pub mod components;

pub use circuit::Circuit;
pub use components::{
    Capacitor, IdealVoltageSource, Inverter, ParallelAdaptor, Resistor, SeriesAdaptor, WDFComponent,
};

/// Tree used by [`rc_low_pass_filter`]: `Vs -- Inverter -- Series(R, C)`.
pub type RcTree = Inverter<SeriesAdaptor<Resistor, Capacitor>>;

/// Type of the circuit returned by [`rc_low_pass_filter`].
pub type RcLowPass = Circuit<RcTree, fn(&RcTree) -> f64>;

/// First-order RC low-pass: voltage source in series with R and C, output
/// taken across C.
///
/// A series adaptor uses the passive sign convention (port voltages sum to
/// zero), so the source is connected through an [`Inverter`] to make the
/// capacitor voltage follow the source with positive sign.
///
/// `R = 1 / (2 * pi * C * fc)`. The capacitor is discretised with the
/// bilinear transform, so the actual -3 dB point is frequency-warped away
/// from `cutoff_hz` as it approaches Nyquist.
pub fn rc_low_pass_filter(cutoff_hz: f64, capacitance: f64, sample_rate: f64) -> RcLowPass {
    let r = 1.0 / (2.0 * std::f64::consts::PI * capacitance * cutoff_hz);
    let series = SeriesAdaptor::new(Resistor::new(r), Capacitor::new(capacitance, sample_rate));
    let tree = Inverter::new(series);
    Circuit::new(IdealVoltageSource::new(tree), |t| t.child.child_b.voltage())
}

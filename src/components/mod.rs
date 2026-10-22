pub mod base_wdf;
pub mod capacitor;
pub mod inverter;
pub mod parallel_adaptor;
pub mod resistor;
pub mod series_adaptor;
pub mod sources;

pub use base_wdf::BaseWDF;
pub use capacitor::Capacitor;
pub use inverter::Inverter;
pub use parallel_adaptor::ParallelAdaptor;
pub use resistor::Resistor;
pub use series_adaptor::SeriesAdaptor;
pub use sources::IdealVoltageSource;

/// A node in a wave digital filter tree.
///
/// The tree is *owned*: adaptors hold their children by value (generically),
/// so there are no parent pointers, no `Rc<RefCell<..>>`, no reference cycles
/// and no runtime borrow checks. Per sample the root drives the tree:
///
/// 1. `reflected()` is called up the tree, leaves first
/// 2. the root turns that wave into an incident wave
/// 3. `incident(a)` is called back down the tree
///
/// After changing a component parameter (e.g. `Resistor::set_resistance`)
/// call `calc_impedance()` on the top of the tree (or `Circuit::update_impedance`)
/// to propagate the new port impedances.
pub trait WDFComponent {
    /// Port impedance seen by the parent.
    fn port_impedance(&self) -> f64;

    /// Port admittance seen by the parent.
    fn port_admittance(&self) -> f64 {
        1.0 / self.port_impedance()
    }

    /// Recompute this element's port impedance from its parameters and
    /// children. Recurses down the tree.
    fn calc_impedance(&mut self);

    /// Accept the wave incident from the parent. Adaptors forward waves to
    /// their children. Must be called after `reflected()` in the same sample.
    fn incident(&mut self, a: f64);

    /// Compute the wave reflected towards the parent. Adaptors call
    /// `reflected()` on their children first.
    fn reflected(&mut self) -> f64;

    /// Last computed reflected wave, without recomputing it.
    fn reflected_wave(&self) -> f64;

    /// Voltage across the port, `(a + b) / 2`.
    fn voltage(&self) -> f64;

    /// Current into the port, `(a - b) / (2 * rp)`.
    fn current(&self) -> f64;

    /// Clear all wave and state memory (not the parameters).
    fn reset(&mut self);
}

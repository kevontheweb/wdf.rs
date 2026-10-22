//! Numerical checks of the WDF tree against closed-form discrete-time
//! references. The capacitor is trapezoidal, so a WDF RC circuit must match
//! the bilinear transform of the analogue circuit sample for sample.

use wdf_lib::{
    rc_low_pass_filter, Capacitor, Circuit, IdealVoltageSource, Inverter, ParallelAdaptor,
    Resistor, SeriesAdaptor, WDFComponent,
};

const TOL: f64 = 1e-10;

/// Deterministic test signal with low, mid and high frequency content plus a step.
fn test_signal(n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let t = i as f64;
            let step = if i > n / 2 { 0.5 } else { 0.0 };
            (0.05 * t).sin() + 0.5 * (1.3 * t).sin() + 0.25 * (2.9 * t).cos() + step
        })
        .collect()
}

/// Bilinear-transform RC low-pass, `H(s) = gain / (1 + s * r * c)`:
/// `y[n] = gain * (x[n] + x[n-1]) - (1 - k) * y[n-1]) / (1 + k)`, `k = 2 * fs * r * c`.
fn bilinear_rc_low_pass(input: &[f64], r: f64, c: f64, fs: f64, gain: f64) -> Vec<f64> {
    let k = 2.0 * fs * r * c;
    let mut out = Vec::with_capacity(input.len());
    let (mut x_prev, mut y_prev) = (0.0, 0.0);
    for &x in input {
        let y = (gain * (x + x_prev) - (1.0 - k) * y_prev) / (1.0 + k);
        out.push(y);
        x_prev = x;
        y_prev = y;
    }
    out
}

fn assert_close(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (i, (a, e)) in actual.iter().zip(expected).enumerate() {
        assert!((a - e).abs() < TOL, "sample {i}: got {a}, expected {e}");
    }
}

#[test]
fn rc_low_pass_matches_bilinear_reference() {
    let (fc, c, fs) = (1000.0, 1e-6, 44_100.0);
    let r = 1.0 / (2.0 * std::f64::consts::PI * c * fc);
    let input = test_signal(2000);

    let mut filter = rc_low_pass_filter(fc, c, fs);
    let output = filter.process_signal(&input);

    assert_close(&output, &bilinear_rc_low_pass(&input, r, c, fs, 1.0));
}

#[test]
fn rc_low_pass_works_above_u16_sample_rates() {
    // regression: sample_rate used to be a u16 and could not hold 96 kHz
    for fs in [96_000.0, 192_000.0] {
        let (fc, c) = (2000.0, 1e-7);
        let r = 1.0 / (2.0 * std::f64::consts::PI * c * fc);
        let input = test_signal(1000);

        let mut filter = rc_low_pass_filter(fc, c, fs);
        let output = filter.process_signal(&input);

        assert_close(&output, &bilinear_rc_low_pass(&input, r, c, fs, 1.0));
    }
}

#[test]
fn step_response_settles_to_dc_gain_of_one() {
    let mut filter = rc_low_pass_filter(500.0, 1e-6, 48_000.0);
    let out = filter.process_signal(&vec![1.0; 20_000]);
    assert!((out.last().unwrap() - 1.0).abs() < 1e-9);
    assert!(
        out[0] > 0.0 && out[0] < 0.1,
        "should start near zero, got {}",
        out[0]
    );
}

#[test]
fn parallel_adaptor_matches_loaded_divider() {
    // Vs -- R1 -- (C || R2) -- gnd, output across C || R2:
    // H(s) = R2/(R1+R2) / (1 + s * C * (R1 || R2))
    let (r1, r2, c, fs) = (2200.0, 4700.0, 220e-9, 44_100.0);
    let series = SeriesAdaptor::new(
        Resistor::new(r1),
        ParallelAdaptor::new(Capacitor::new(c, fs), Resistor::new(r2)),
    );
    let tree = Inverter::new(series);
    let mut circuit = Circuit::new(IdealVoltageSource::new(tree), |t| t.child.child_b.voltage());

    let input = test_signal(2000);
    let output = circuit.process_signal(&input);

    let gain = r2 / (r1 + r2);
    let r_eq = r1 * r2 / (r1 + r2);
    assert_close(&output, &bilinear_rc_low_pass(&input, r_eq, c, fs, gain));
}

#[test]
fn series_adaptor_without_inverter_uses_passive_sign_convention() {
    // Port voltages of a series junction sum to zero, so with the source wired
    // straight onto the adaptor the capacitor voltage is -H(s) * Vs.
    let (r, c, fs) = (1000.0, 1e-6, 44_100.0);
    let tree = SeriesAdaptor::new(Resistor::new(r), Capacitor::new(c, fs));
    let mut circuit = Circuit::new(IdealVoltageSource::new(tree), |t| t.child_b.voltage());

    let input = test_signal(1000);
    let output = circuit.process_signal(&input);

    let expected: Vec<f64> = bilinear_rc_low_pass(&input, r, c, fs, 1.0)
        .into_iter()
        .map(|y| -y)
        .collect();
    assert_close(&output, &expected);
}

#[test]
fn changing_resistance_updates_impedances() {
    // Trapezoidal RC with R switched at sample `switch`; the capacitor's
    // current history carries over from the old resistance.
    let (r_a, r_b, c, fs) = (1000.0, 4700.0, 1e-6, 44_100.0);
    let n = 1500;
    let switch = 700;
    let input = test_signal(n);

    let mut circuit = rc_low_pass_filter(1.0 / (2.0 * std::f64::consts::PI * c * r_a), c, fs);
    let mut actual = Vec::with_capacity(n);
    for (i, &x) in input.iter().enumerate() {
        if i == switch {
            circuit.tree_mut().child.child_a.set_resistance(r_b);
            circuit.update_impedance();
        }
        actual.push(circuit.process_sample(x));
    }

    let t_over_2c = 1.0 / (2.0 * fs * c);
    let (mut v_prev, mut i_prev) = (0.0, 0.0);
    let mut expected = Vec::with_capacity(n);
    for (i, &x) in input.iter().enumerate() {
        let r = if i < switch { r_a } else { r_b };
        let v = (v_prev + t_over_2c * (i_prev + x / r)) / (1.0 + t_over_2c / r);
        let i_now = (x - v) / r;
        expected.push(v);
        v_prev = v;
        i_prev = i_now;
    }

    assert_close(&actual, &expected);
}

#[test]
fn reset_restores_initial_behaviour() {
    let mut filter = rc_low_pass_filter(1000.0, 1e-6, 44_100.0);
    let input = test_signal(500);
    let first = filter.process_signal(&input);
    filter.reset();
    let second = filter.process_signal(&input);
    assert_eq!(first, second);
}

#[test]
fn process_block_matches_process_signal() {
    let input = test_signal(256);
    let mut a = rc_low_pass_filter(800.0, 1e-6, 44_100.0);
    let mut b = rc_low_pass_filter(800.0, 1e-6, 44_100.0);
    let mut block = vec![0.0; input.len()];
    a.process_block(&input, &mut block);
    assert_eq!(block, b.process_signal(&input));
}

#[test]
fn component_waves_follow_conventions() {
    let mut r = Resistor::new(100.0);
    r.incident(2.0);
    assert_eq!(r.reflected(), 0.0);
    assert_eq!(r.voltage(), 1.0);
    assert_eq!(r.current(), 0.01); // v / R

    let mut c = Capacitor::new(1e-6, 48_000.0);
    assert_eq!(c.reflected(), 0.0);
    c.incident(0.7);
    assert_eq!(c.reflected(), 0.7); // one-sample delay of the incident wave
}

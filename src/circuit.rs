/*
class Circuit {
private:
  RootWDF *root;
  Source *source;
  BaseWDF *output;

public:
  Circuit(RootWDF *root, Source *source, BaseWDF *output) : root(root), source(source), output(output) {}

  virtual double process_sample(double sample) {
    this->source->set_voltage(sample);
    this->root->accept_incident_wave(this->root->next->propagate_reflected_wave());
    this->root->next->accept_incident_wave(this->root->propagate_reflected_wave());
    return this->output->get_voltage_from_wave();
  }

  std::vector<double> process_signal(const std::vector<double> &input_signal) {
    std::vector<double> output;
    double output_sample = 0.0f;
    for (const double &sample : input_signal) {
      output_sample = this->process_sample(sample);
      output.push_back(output_sample); // push new value onto vector
    }
    return output;
  }
*/

use crate::components::base_wdf::base_wdf::BaseWDF;
use crate::components::root_wdf::root_wdf::RootWDF;
use crate::components::sources::sources::IdealVoltageSource;
use crate::components::WDFComponent;
use std::cell::RefCell;
use std::rc::Rc;

// Define the Circuit struct
struct Circuit {
    root: Rc<RefCell<RootWDF>>,
    source: Rc<RefCell<IdealVoltageSource>>,
    output: Rc<RefCell<BaseWDF>>,
}

impl Circuit {
    pub fn new(
        root: Rc<RefCell<RootWDF>>,
        source: Rc<RefCell<IdealVoltageSource>>,
        output: Rc<RefCell<BaseWDF>>,
    ) -> Self {
        Circuit {
            root: root.clone(),
            source: source.clone(),
            output: output.clone(),
        }
    }

    pub fn process_sample(&self, sample: f64) -> f64 {
        let mut source_ref = self.source.borrow_mut();
        source_ref.set_voltage(sample);

        let mut root_ref = self.root.borrow_mut();
        root_ref.accept_incident_wave(
            root_ref
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .propagate_reflected_wave(),
        );

        let mut output_ref = self.output.borrow_mut();
        output_ref.a = root_ref.propagate_reflected_wave();

        output_ref.get_voltage_from_wave()
    }

    pub fn process_signal(&self, input_signal: Vec<f64>) -> Vec<f64> {
        let mut output = Vec::new();
        for sample in input_signal {
            output.push(self.process_sample(sample));
        }
        output
    }
}

// Example usage
fn main() {
    let root = Rc::new(RefCell::new(RootWDF::new(Rc::new(RefCell::new(
        BaseWDF::default(),
    )))));
    let source = Rc::new(RefCell::new(IdealVoltageSource::new(Some(&*root.borrow()))));
    let output = Rc::new(RefCell::new(BaseWDF::default()));

    let circuit = Circuit::new(root.clone(), source.clone(), output.clone());

    let input_signal = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let output_signal = circuit.process_signal(input_signal);

    println!("Output signal: {:?}", output_signal);
}

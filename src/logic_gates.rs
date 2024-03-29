use crate::circuit::Circuit;
use crate::circuit::CircuitElement;
use crate::circuit::CircuitState;
use crate::circuit::Source;
use crate::circuit::Wire;

pub struct NotGate {
    src: Source,
    output: Wire,
}

impl CircuitElement for NotGate {
    fn step(&self, evaluator: &CircuitState, recorder: &mut CircuitState) {
        let value = evaluator.get(self.src);
        recorder.set(self.output.sink(), !value);
    }
    fn set_input(&mut self, index: usize, input: Source) {
        assert_eq!(index, 0);
        self.src = input;
    }
    fn get_output(&self, index: usize) -> Source {
        assert_eq!(index, 0);
        return self.output.source();
    }
    fn get_input(&self, index: usize) -> Source {
        assert!(index < 1);
        return self.src;
    }
    fn input_count(&self) -> usize {
        return 1;
    }
    fn output_count(&self) -> usize {
        return 1;
    }
}

impl NotGate {
    pub fn new(circuit: &mut Circuit) -> Self {
        return NotGate {
            src: circuit.low(),
            output: circuit.add_wire(),
        };
    }
}

pub struct AndGate {
    sources: Vec<Source>,
    output: Wire,
}

impl AndGate {
    pub fn new(circuit: &mut Circuit, width: usize) -> Self {
        return Self {
            sources: vec![circuit.low(); width],
            output: circuit.add_wire(),
        };
    }
}

impl CircuitElement for AndGate {
    fn step(&self, evaluator: &CircuitState, recorder: &mut CircuitState) {
        let value = self.sources.iter().fold(true, |a, s| {
            return a && evaluator.get(*s);
        });
        recorder.set(self.output.sink(), value);
    }

    fn set_input(&mut self, index: usize, input: Source) {
        assert!(index < self.sources.len());
        self.sources[index] = input;
    }
    fn get_output(&self, index: usize) -> Source {
        assert_eq!(index, 0);
        return self.output.source();
    }
    fn get_input(&self, index: usize) -> Source {
        assert!(index < self.sources.len());
        return self.sources[index];
    }
    fn input_count(&self) -> usize {
        return self.sources.len();
    }
    fn output_count(&self) -> usize {
        return 1;
    }
}

pub struct NandGate {
    sources: Vec<Source>,
    output: Wire,
}
impl NandGate {
    pub fn new(circuit: &mut Circuit, width: usize) -> Self {
        return Self {
            sources: vec![circuit.low(); width],
            output: circuit.add_wire(),
        };
    }
}

impl CircuitElement for NandGate {
    fn step(&self, evaluator: &CircuitState, recorder: &mut CircuitState) {
        let value = self.sources.iter().fold(true, |a, s| {
            return a && evaluator.get(*s);
        });
        recorder.set(self.output.sink(), !value);
    }

    fn set_input(&mut self, index: usize, input: Source) {
        assert!(index < self.sources.len());
        self.sources[index] = input;
    }

    fn get_input(&self, index: usize) -> Source {
        assert!(index < self.sources.len());
        return self.sources[index];
    }

    fn get_output(&self, index: usize) -> Source {
        assert_eq!(index, 0);
        return self.output.source();
    }
    fn input_count(&self) -> usize {
        return self.sources.len();
    }
    fn output_count(&self) -> usize {
        return 1;
    }
}

pub struct XorGate {
    sources: [Source; 2],
    output: Wire,
}

impl XorGate {
    pub fn new(circuit: &mut Circuit) -> Self {
        return Self {
            sources: [circuit.low(); 2],
            output: circuit.add_wire(),
        };
    }
}

impl CircuitElement for XorGate {
    fn step(&self, evaluator: &CircuitState, recorder: &mut CircuitState) {
        let left = evaluator.get(self.sources[0]);
        let right = evaluator.get(self.sources[1]);
        println!("xor {}, {}", left, right);
        recorder.set(self.output.sink(), left ^ right);
    }

    fn set_input(&mut self, index: usize, input: Source) {
        assert!(index < self.sources.len());
        self.sources[index] = input;
    }
    fn get_input(&self, index: usize) -> Source {
        assert!(index < self.sources.len());
        return self.sources[index];
    }
    fn get_output(&self, index: usize) -> Source {
        assert_eq!(index, 0);
        return self.output.source();
    }
    fn input_count(&self) -> usize {
        return self.sources.len();
    }
    fn output_count(&self) -> usize {
        return 1;
    }
}

use crate::circuit::*;

pub struct CircuitSimulator {
    circuit: Circuit,
    state0: CircuitState,
    state1: CircuitState
}

impl CircuitSimulator {
    pub fn new(circuit: Circuit) -> Self {
        return Self {
            state0: circuit.createState(),
            state1: circuit.createState(),
            circuit: circuit
        }
    }

    pub fn step(&mut self) {
        for elem in self.circuit.elements() {
            elem.step(&self.state0, &mut self.state1);
        };
        std::mem::swap(&mut self.state0, &mut self.state1);
    }

    pub fn readPoint(&self, s: Source) -> bool {
        return self.state0.get(s);
    }
}

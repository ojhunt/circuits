use crate::circuit::*;
use crate::circuit_simulator::CircuitSimulator;
use crate::logic_gates::*;

#[test]
fn xor_test() {
    {
        let mut circuit = Circuit::new();
        let mut xor = XorGate::new(&mut circuit);
        xor.set_input(0, circuit.high());
        let output = xor.get_output(0);
        let g = circuit.addElement(xor);
        circuit.setElementInput(g, 0, circuit.high());
        let mut simulator = CircuitSimulator::new(circuit);
        simulator.step();
        simulator.step();
        assert!(simulator.readPoint(output));
    }
}

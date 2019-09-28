macro_rules! nary_gate_test {
    ($testname:ident, $gate_type:ident, $left: ident, $right:ident, $op:expr $(, $tail:expr)*) => {
        #[test]
        fn $testname() {
            {
                use crate::circuit::*;
                use crate::circuit_simulator::CircuitSimulator;
                use crate::logic_gates::*;
                for left in &[false, true] {
                    for right in &[false, true] {
                        let $left = *left;
                        let $right = *right;
                        let mut circuit = Circuit::new();
                        let low = circuit.low();
                        let high = circuit.high();
                        let mut gate = $gate_type::new(&mut circuit, $($tail)*);
                        gate.set_input(0, if $left { high } else { low });
                        gate.set_input(1, if $right { high } else { low });
                        let output = gate.get_output(0);
                        let g = circuit.addElement(gate);
                        let mut simulator = CircuitSimulator::new(circuit);
                        simulator.step();
                        simulator.step();
                        let result = simulator.readPoint(output);
                        assert_eq!(result, $op);
                    }
                }
            }
        }
    };
}

nary_gate_test!(test_xor, XorGate, left, right, left ^ right);
nary_gate_test!(test_and, AndGate, left, right, left && right, 2);
nary_gate_test!(test_nand, NandGate, left, right, !(left && right), 2);

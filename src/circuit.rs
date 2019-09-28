
type IndexType = u32;

#[derive(Clone, Copy)]
pub struct Sink(IndexType);
#[derive(Clone, Copy)]
pub struct Source(IndexType);
#[derive(Clone, Copy)]
pub struct Wire(IndexType);

pub trait CircuitElement {
    fn step(&self, evaluator: &CircuitState, new_state: &mut CircuitState);
    fn set_input(&mut self, index: usize, input: Source);
    fn get_output(&self, index: usize) -> Source;
    fn input_count(&self) -> usize;
    fn output_count(&self) -> usize;
}

pub struct CircuitState {
    state: Vec<bool>
}

impl CircuitState {
    fn new(size: u32) -> Self {
        return CircuitState {
            state: vec![false; size as usize]
        }
    }

    pub fn get(&self, Source(s): Source) -> bool {
        return self.state[s as usize];
    }
    pub fn set(&mut self, Sink(s): Sink, b: bool) {
        self.state[s as usize] = b;
    }
}

pub struct Circuit {
    m_elements: Vec<Box<dyn CircuitElement>>,
    m_value_count: IndexType,
    low: Source,
    high: Source,
}

impl Wire {
    pub fn source(self) -> Source { return Source(self.0) }
    pub fn sink(self) -> Sink { return Sink(self.0) }
}
struct Constant(Wire, bool);
impl CircuitElement for Constant {
    fn step(&self, _evaluator: &CircuitState, new_state: &mut CircuitState) {
        new_state.set(self.0.sink(), self.1);
    }
    fn set_input(&mut self, _index: usize, _input: Source) {
        panic!();
    }
    fn get_output(&self, index: usize) -> Source {
        assert_eq!(index, 0);
        return self.0.source();
    }
    fn input_count(&self) -> usize { return 0; }
    fn output_count(&self) -> usize {
        return 1;
    }
}

pub trait UnaryOutputElement : CircuitElement {
    fn new() -> Self;
}

#[derive(Clone, Copy)]
pub struct ElementRef(usize);

impl Circuit {
    pub fn new() -> Self {
        let low = Box::new(Constant(Wire(0), false));
        let high = Box::new(Constant(Wire(1), true));
        
        return Circuit {
            low: low.get_output(0),
            high: high.get_output(0),
            m_elements: vec![low, high],
            m_value_count: 2,
        };
    }

    pub fn createState(&self) -> CircuitState {
        return CircuitState::new(self.m_value_count);
    }

    pub fn addWire(&mut self) -> Wire {
        let result = Wire(self.m_value_count);
        self.m_value_count += 1;
        return result;
    }

    pub fn state_size(&self) -> usize {
        return self.m_value_count as usize;
    }

    pub fn low(&self) -> Source {
        return self.low
    }

    pub fn high(&self) -> Source {
        return self.high
    }

    pub fn elements(&self) -> &[Box<dyn CircuitElement>] {
        return &self.m_elements;
    }

    pub fn addElement<E: 'static + CircuitElement>(&mut self, elem: E) -> ElementRef {
        let result = (self.m_elements.len());
        self.m_elements.push(Box::new(elem));
        return ElementRef(result);
    }

    pub fn setElementInput(&mut self, ElementRef(e): ElementRef, index: usize, input:Source) {
        self.m_elements[e].set_input(index, input);
    }

    pub fn getElementOutput(&self, ElementRef(e):ElementRef, index: usize) -> Source {
        return self.m_elements[e].get_output(index);
    }
}


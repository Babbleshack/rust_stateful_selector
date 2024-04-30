#[derive(Debug, Copy, Clone)]
pub struct Backend {
    pub value: usize,
    pub weight: usize
}


#[allow(dead_code)]
impl Backend {
    pub fn new(value: usize, weight: usize) -> Self {
        Self {
            value,
            weight
        }
    }
}

pub type Backends = Vec<Backend>;

#[derive(Debug, Default)]
pub struct ROM {
    bytes: Vec<i8>,
}

impl ROM {
    pub fn new() -> Self {
        Self {
            bytes: vec![0; 256],
        }
    }

    pub fn bytes(&self) -> &[i8] {
        &self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut [i8] {
        &mut self.bytes
    }
}

#[derive(Debug, Default)]
pub struct Rom {
    bytes: Vec<i8>,
}

impl Rom {
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

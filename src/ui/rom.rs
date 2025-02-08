#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    pub fn new() -> Self {
        Self {
            bytes: vec![0; 256],
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

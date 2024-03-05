#[derive(Debug)]
pub struct Base10 {
    value: String,
}

impl Base10 {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl PartialEq for Base10 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

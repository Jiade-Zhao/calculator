pub struct Calculate {
    pub num: f64,
    pub result: f64,
}

#[derive(Default, PartialEq)]
pub enum Enum {
    #[default]NoOperation,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Calculate {
    pub fn calculate(&mut self, operation: &Enum) -> f64{
        match &operation {
            Enum::NoOperation => {
                self.result = self.num;
            },
            Enum::Addition => {
                self.result = self.result + self.num;
            },
            Enum::Subtraction => {
                self.result = self.result - self.num;
            },
            Enum::Multiplication => {
                self.result = self.result * self.num;
            },
            Enum::Division => {
                self.result = self.result / self.num;
            }
        }
        self.result
    }
}

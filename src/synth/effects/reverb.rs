pub struct Reverb {
    // ...
}

impl Reverb {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn process(input_l: f64, input_r: f64) -> (f64, f64) {
        return (input_l, input_r);
    }
}
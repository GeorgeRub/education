pub(crate) struct Car {
    pub(crate) model: String,
    pub(crate) color: String,
}

impl Car {
    pub(crate) fn string(&self) {
        println!("{} {}", self.model, self.color)
    }
}
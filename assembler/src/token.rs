pub enum Value {
    Immediate(u16),
    Symbol { name: String },
}

pub enum Token {
    A { value: Value },
}

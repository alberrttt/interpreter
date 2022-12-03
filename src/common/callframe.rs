use super::function::Function;

pub struct CallFrame {
    pub function: Function,
    pub slots: usize,
}


#[derive(Debug, PartialEq)]
pub enum Op {
    Increment,
    Decrement,
    Output,
    Right,
    Left,
    Input,
    Jump,
    JumpBack,
    Unknown,
}

pub trait ToOp {
    fn to_op(&self) -> Op;
}

impl ToOp for String {
    fn to_op(&self) -> Op {
        match self.as_slice() {
            "Bacon+" => Op::Increment,
            "Bacon-" => Op::Decrement,
            "Bacon." => Op::Output,
            "Bacon>" => Op::Right,
            "Bacon<" => Op::Left,
            "Bacon^" => Op::Jump,
            "Bacon?" => Op::JumpBack,
            "Bacon|" => Op::Input,
            _ => Op::Unknown,
        }
    }
}


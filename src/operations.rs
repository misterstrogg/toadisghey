
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
            "toadisghey+" => Op::Increment,
            "toadisghey-" => Op::Decrement,
            "toadisghey." => Op::Output,
            "toadisghey>" => Op::Right,
            "toadisghey<" => Op::Left,
            "toadisghey^" => Op::Jump,
            "toadisghey?" => Op::JumpBack,
            "toadisghey|" => Op::Input,
            _ => Op::Unknown,
        }
    }
}


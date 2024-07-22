use crate::op::Opcode;

#[derive(Debug, Clone)]
pub enum Data {
    Op(Opcode),
    Value(i64)
}
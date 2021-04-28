pub enum OpType {
    Eq,
}

impl OpType {
    pub fn to_str(&self) -> &str {
        match self {
            OpType::Eq => "=",
        }
    }
}

pub enum FieldType {
    Assignee,
}

impl FieldType {
    pub fn to_str(&self) -> &'static str {
        match self {
            FieldType::Assignee => "Assignee",
        }
    }
}

pub struct JqlNode {
    op: OpType,
}

pub fn parse(_query: &[&str]) -> JqlNode {
    JqlNode { op: OpType::Eq }
}

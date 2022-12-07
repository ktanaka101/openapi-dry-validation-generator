pub struct RootSchema {
    pub name: Option<String>,
    pub queries: Vec<Schema>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub ty: Type,
    pub required: bool,
    pub name: String,
    pub validates: Vec<Validate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Validate {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integer,
}

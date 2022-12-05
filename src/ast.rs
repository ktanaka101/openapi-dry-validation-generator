#[derive(Debug, Clone, PartialEq, Eq)]
struct Schema {
    ty: Type,
    required: bool,
    validates: Vec<Validate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Validate {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Integer,
}

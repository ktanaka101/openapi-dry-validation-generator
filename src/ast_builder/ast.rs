pub struct RootSchema {
    pub path_items: Vec<PathItem>,
}

pub struct PathItem {
    pub url: String,
    pub operations: Vec<Operation>,
}

pub struct Operation {
    pub id: Option<String>,
    pub queries: Vec<Schema>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub ty: Type,
    pub required: bool,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Validate {
    Max(i64),
    Min(i64),
    MaxF(f64),
    MinF(f64),
    MaxLength(usize),
    MinLength(usize),
    MaxItems(usize),
    MinItems(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer {
        validates: Vec<Validate>,
    },
    Number {
        validates: Vec<Validate>,
    },
    String {
        validates: Vec<Validate>,
    },
    Boolean,
    Array {
        validates: Vec<Validate>,
        item_ty: Option<Box<Type>>,
    },
    Object {
        validates: Vec<Validate>,
        properties: Vec<Property>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub required: bool,
    pub key: String,
    pub value: Type,
}

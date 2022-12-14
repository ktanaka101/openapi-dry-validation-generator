pub struct Def {
    pub name: String,
    pub class: SchemaClass,
    pub block: Vec<Stmt>,
}

pub enum SchemaClass {
    Params,
}

pub enum Stmt {
    Required { name: String, r#macro: Macro },
    Optional { name: String, r#macro: Macro },
}

pub enum Macro {
    Value { ty: Type, validates: Vec<Validate> },
}

pub enum Validate {
    Max(i64),
    Min(i64),
    MaxLength(usize),
    MinLength(usize),
}

pub enum Type {
    Integer,
    String,
}

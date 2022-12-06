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
    Value { ty: Type },
}

pub enum Type {
    Integer,
}

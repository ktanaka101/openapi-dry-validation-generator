pub struct Def {
    pub name: String,
    pub class: SchemaClass,
    pub block: Block,
}

pub enum SchemaClass {
    Params,
}

pub enum Stmt {
    Required { name: String, r#macro: Macro },
    Optional { name: String, r#macro: Macro },
    Schema { ty: Type, r#macro: Macro },
}

pub enum Macro {
    Value {
        ty: Type,
        validates: Vec<Validate>,
        macro_or_block: Option<Box<MacroOrBlock>>,
    },
    Each {
        ty: Type,
        validates: Vec<Validate>,
        block: Option<Block>,
    },
}

pub enum MacroOrBlock {
    Macro(Macro),
    Block(Block),
}

pub struct Block {
    pub stmts: Vec<Stmt>,
}
impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    pub fn new_single_stmt(stmt: Stmt) -> Self {
        Self { stmts: vec![stmt] }
    }
}

pub enum Validate {
    Max(i64),
    Min(i64),
    MaxSize(usize),
    MinSize(usize),
}

pub enum Type {
    Integer,
    String,
    Boolean,
    Array,
    Hash,
}

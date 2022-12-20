pub mod ir;

use crate::ast_builder::ast;

pub fn build(root_schema: &ast::RootSchema) -> IrResult {
    let builder = IrBuilder::new();
    builder.build(root_schema)
}

pub struct IrResult {
    pub ir: ir::Def,
}

struct IrBuilder {}
impl IrBuilder {
    fn new() -> Self {
        IrBuilder {}
    }

    fn build(&self, ast: &ast::RootSchema) -> IrResult {
        let mut stmts = vec![];
        for param in &ast.queries {
            let stmt = match &param.ty {
                ast::Type::Integer { validates } => {
                    let name = param.name.clone();
                    let r#macro = ir::Macro::Value {
                        ty: ir::Type::Integer,
                        validates: self.build_validates(validates),
                        r#macro: None,
                    };

                    if param.required {
                        ir::Stmt::Required { name, r#macro }
                    } else {
                        ir::Stmt::Optional { name, r#macro }
                    }
                }
                ast::Type::String { validates } => {
                    let name = param.name.clone();
                    let r#macro = ir::Macro::Value {
                        ty: ir::Type::String,
                        validates: self.build_validates(validates),
                        r#macro: None,
                    };

                    if param.required {
                        ir::Stmt::Required { name, r#macro }
                    } else {
                        ir::Stmt::Optional { name, r#macro }
                    }
                }
                ast::Type::Boolean => {
                    let name = param.name.clone();
                    let r#macro = ir::Macro::Value {
                        ty: ir::Type::Boolean,
                        validates: vec![],
                        r#macro: None,
                    };

                    if param.required {
                        ir::Stmt::Required { name, r#macro }
                    } else {
                        ir::Stmt::Optional { name, r#macro }
                    }
                }
                ast::Type::Array { validates, item_ty } => {
                    let name = param.name.clone();
                    let r#macro = ir::Macro::Value {
                        ty: ir::Type::Array,
                        validates: self.build_validates(validates),
                        r#macro: if let Some(item) = item_ty {
                            let item = *item.to_owned();
                            Some(Box::new(self.build_item(&item)))
                        } else {
                            None
                        },
                    };

                    if param.required {
                        ir::Stmt::Required { name, r#macro }
                    } else {
                        ir::Stmt::Optional { name, r#macro }
                    }
                }
            };

            stmts.push(stmt);
        }

        IrResult {
            ir: ir::Def {
                name: ast.name.clone().unwrap(),
                class: ir::SchemaClass::Params,
                block: ir::Block::new(stmts),
            },
        }
    }

    fn build_item(&self, item: &ast::Type) -> ir::Macro {
        match &item {
            ast::Type::Integer { validates } => ir::Macro::Each {
                ty: ir::Type::Integer,
                validates: self.build_validates(validates),
                block: None,
            },
            ast::Type::String { validates } => ir::Macro::Each {
                ty: ir::Type::String,
                validates: self.build_validates(validates),
                block: None,
            },
            ast::Type::Boolean => ir::Macro::Each {
                ty: ir::Type::Boolean,
                validates: vec![],
                block: None,
            },
            ast::Type::Array { validates, item_ty } => ir::Macro::Each {
                ty: ir::Type::Array,
                validates: self.build_validates(validates),
                block: item_ty.clone().map(|item_ty| {
                    ir::Block::new_single_stmt({
                        ir::Stmt::Schema {
                            ty: ir::Type::Array,
                            r#macro: self.build_item(&item_ty),
                        }
                    })
                }),
            },
        }
    }

    fn build_validates(&self, validates: &[ast::Validate]) -> Vec<ir::Validate> {
        let mut validates = validates
            .iter()
            .map(|validate| match validate {
                ast::Validate::Max(max) => ir::Validate::Max(*max),
                ast::Validate::Min(min) => ir::Validate::Min(*min),
                ast::Validate::MaxLength(max) | ast::Validate::MaxItems(max) => {
                    ir::Validate::MaxSize(*max)
                }
                ast::Validate::MinLength(min) | ast::Validate::MinItems(min) => {
                    ir::Validate::MinSize(*min)
                }
            })
            .collect::<Vec<_>>();
        validates.sort_by_cached_key(|validate| match validate {
            ir::Validate::Min(_) => 0,
            ir::Validate::MinSize(_) => 1,
            ir::Validate::Max(_) => 2,
            ir::Validate::MaxSize(_) => 3,
        });

        validates
    }
}

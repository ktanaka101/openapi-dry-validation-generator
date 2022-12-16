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
            let ty = match &param.ty {
                ast::Type::Integer { validates } => ir::Type::Integer {
                    validates: self.build_validates(validates),
                },
                ast::Type::String { validates } => ir::Type::String {
                    validates: self.build_validates(validates),
                },
                ast::Type::Array { validates, item_ty } => {
                    let each = if let Some(item) = item_ty {
                        let item = *item.to_owned();
                        Some(Box::new(self.build_item(&item)))
                    } else {
                        None
                    };
                    ir::Type::Array {
                        validates: self.build_validates(validates),
                        item: each,
                    }
                }
            };

            let stmt = if param.required {
                ir::Stmt::Required {
                    name: param.name.clone(),
                    r#macro: ir::Macro::Value { ty },
                }
            } else {
                ir::Stmt::Optional {
                    name: param.name.clone(),
                    r#macro: ir::Macro::Value { ty },
                }
            };

            stmts.push(stmt);
        }

        IrResult {
            ir: ir::Def {
                name: ast.name.clone().unwrap(),
                class: ir::SchemaClass::Params,
                block: stmts,
            },
        }
    }

    fn build_item(&self, item: &ast::Type) -> ir::Each {
        match &item {
            ast::Type::String { validates } => ir::Each {
                ty: ir::Type::String {
                    validates: self.build_validates(validates),
                },
            },
            ast::Type::Integer { validates } => ir::Each {
                ty: ir::Type::Integer {
                    validates: self.build_validates(validates),
                },
            },
            ast::Type::Array { validates, item_ty } => ir::Each {
                ty: ir::Type::Array {
                    validates: self.build_validates(validates),
                    item: item_ty.clone().map(|item| Box::new(self.build_item(&item))),
                },
            },
        }
    }

    fn build_validates(&self, validates: &[ast::Validate]) -> Vec<ir::Validate> {
        validates
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
            .collect()
    }
}

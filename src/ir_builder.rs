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
            let ty = match param.ty {
                ast::Type::Integer => ir::Type::Integer,
                ast::Type::String => ir::Type::String,
                ast::Type::Array { .. } => ir::Type::Array,
            };

            let validates = self.build_validates(&param.validates);
            let stmt = if param.required {
                ir::Stmt::Required {
                    name: param.name.clone(),
                    r#macro: ir::Macro::Value { ty, validates },
                }
            } else {
                ir::Stmt::Optional {
                    name: param.name.clone(),
                    r#macro: ir::Macro::Value { ty, validates },
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

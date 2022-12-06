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
        for param in &ast.parameters {
            let ty = match param.ty {
                ast::Type::Integer => ir::Type::Integer,
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
}

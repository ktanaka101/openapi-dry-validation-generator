pub mod ir;

use crate::ast_builder::ast;

pub fn build(root_schema: &ast::RootSchema) -> IrResult {
    let builder = IrBuilder::new();
    builder.build(root_schema)
}

pub struct IrResult {
    pub ir: ir::Defs,
}

struct IrBuilder {}
impl IrBuilder {
    fn new() -> Self {
        IrBuilder {}
    }

    fn build(&self, ast: &ast::RootSchema) -> IrResult {
        let mut defs = vec![];

        for path_item in &ast.path_items {
            for operation in &path_item.operations {
                let mut stmts = vec![];
                for param in &operation.queries {
                    stmts.push(self.build_property(param.name.clone(), param.required, &param.ty));
                }

                defs.push(ir::Def {
                    name: operation.id.clone().unwrap(),
                    class: ir::SchemaClass::Params,
                    block: ir::Block::new(stmts),
                });
            }
        }

        IrResult {
            ir: ir::Defs { defs },
        }
    }

    fn build_item(&self, item: &ast::Type) -> ir::Macro {
        match &item {
            ast::Type::Integer { validates } => ir::Macro::Each {
                ty: ir::Type::Integer,
                validates: self.build_validates(validates),
                block: None,
            },

            ast::Type::Number { validates } => ir::Macro::Each {
                ty: ir::Type::Float,
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
            ast::Type::Object {
                validates,
                properties,
            } => ir::Macro::Each {
                ty: ir::Type::Hash,
                validates: self.build_validates(validates),
                block: {
                    if properties.is_empty() {
                        None
                    } else {
                        Some(self.build_properties(properties))
                    }
                },
            },
        }
    }

    fn build_properties(&self, properties: &[ast::Property]) -> ir::Block {
        let mut stmts = vec![];
        for property in properties {
            stmts.push(self.build_property(
                property.key.clone(),
                property.required,
                &property.value,
            ));
        }

        ir::Block { stmts }
    }

    fn build_property(&self, name: String, required: bool, ty: &ast::Type) -> ir::Stmt {
        let r#macro = match ty {
            ast::Type::Integer { validates } => ir::Macro::Value {
                ty: ir::Type::Integer,
                validates: self.build_validates(validates),
                macro_or_block: None,
            },
            ast::Type::Number { validates } => ir::Macro::Value {
                ty: ir::Type::Float,
                validates: self.build_validates(validates),
                macro_or_block: None,
            },
            ast::Type::String { validates } => ir::Macro::Value {
                ty: ir::Type::String,
                validates: self.build_validates(validates),
                macro_or_block: None,
            },
            ast::Type::Boolean => ir::Macro::Value {
                ty: ir::Type::Boolean,
                validates: vec![],
                macro_or_block: None,
            },
            ast::Type::Array { validates, item_ty } => ir::Macro::Value {
                ty: ir::Type::Array,
                validates: self.build_validates(validates),
                macro_or_block: if let Some(item) = item_ty {
                    let item = *item.to_owned();
                    Some(Box::new(ir::MacroOrBlock::Macro(self.build_item(&item))))
                } else {
                    None
                },
            },
            ast::Type::Object {
                validates: _,
                properties,
            } => ir::Macro::Value {
                ty: ir::Type::Hash,
                validates: vec![],
                macro_or_block: {
                    if properties.is_empty() {
                        None
                    } else {
                        Some(Box::new(ir::MacroOrBlock::Block(
                            self.build_properties(properties),
                        )))
                    }
                },
            },
        };

        if required {
            ir::Stmt::Required { name, r#macro }
        } else {
            ir::Stmt::Optional { name, r#macro }
        }
    }

    fn build_validates(&self, validates: &[ast::Validate]) -> Vec<ir::Validate> {
        let mut validates = validates
            .iter()
            .map(|validate| match validate {
                ast::Validate::Max(max) => ir::Validate::Max(*max),
                ast::Validate::Min(min) => ir::Validate::Min(*min),
                ast::Validate::MaxF(max) => ir::Validate::MaxF(*max),
                ast::Validate::MinF(min) => ir::Validate::MinF(*min),
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
            ir::Validate::MinF(_) => 1,
            ir::Validate::MinSize(_) => 2,
            ir::Validate::Max(_) => 3,
            ir::Validate::MaxF(_) => 4,
            ir::Validate::MaxSize(_) => 5,
        });

        validates
    }
}

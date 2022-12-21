use convert_case::{Case, Casing};

use crate::ir_builder::ir;

pub fn generate(def: &ir::Def) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "{} = {}",
        gen_def_name(&def.name),
        gen_schema_class(&def.class)
    ));
    code.push_str(&gen_block(&def.block, 0));
    code.push('\n');

    code
}

fn gen_block(block: &ir::Block, nesting: usize) -> String {
    let mut out = " do\n".to_string();
    for stmt in &block.stmts {
        out.push_str(&format!(
            "{}{}\n",
            indent(nesting + 1),
            &gen_stmt(stmt, nesting + 1)
        ));
    }
    out.push_str(&format!("{}end", indent(nesting)));

    out
}

fn gen_stmt(stmt: &ir::Stmt, nesting: usize) -> String {
    match stmt {
        ir::Stmt::Required { name, r#macro } => {
            format!("required(:{name}){}", gen_macro(r#macro, nesting))
        }
        ir::Stmt::Optional { name, r#macro } => {
            format!("optional(:{name}){}", gen_macro(r#macro, nesting))
        }
        ir::Stmt::Schema { ty, r#macro } => {
            format!(
                "schema(:{}){}",
                gen_type_predicate(ty),
                gen_macro(r#macro, nesting)
            )
        }
    }
}

fn gen_macro(r#macro: &ir::Macro, nesting: usize) -> String {
    match r#macro {
        ir::Macro::Value {
            ty,
            validates,
            macro_or_block,
        } => {
            const LITERAL: &str = ".value";
            let mut out = if validates.is_empty() {
                format!("{}(:{})", LITERAL, gen_type_spec(ty))
            } else {
                format!(
                    "{}(:{}, {})",
                    LITERAL,
                    gen_type_spec(ty),
                    gen_validates(validates)
                )
            };
            if let Some(macro_or_block) = macro_or_block {
                match macro_or_block.as_ref() {
                    ir::MacroOrBlock::Macro(r#macro) => {
                        out.push_str(&gen_macro(r#macro, nesting));
                    }
                    ir::MacroOrBlock::Block(block) => {
                        out.push_str(&gen_block(block, nesting));
                    }
                }
            }

            out
        }
        ir::Macro::Each {
            ty,
            validates,
            block,
        } => {
            const LITERAL: &str = ".each";
            match ty {
                ir::Type::Integer | ir::Type::String | ir::Type::Boolean => {
                    if validates.is_empty() {
                        format!("{}(:{})", LITERAL, gen_type_predicate(ty))
                    } else {
                        format!(
                            "{}(:{}, {})",
                            LITERAL,
                            gen_type_predicate(ty),
                            gen_validates(validates)
                        )
                    }
                }
                ir::Type::Array => {
                    let mut out = if validates.is_empty() {
                        format!("{}(:{})", LITERAL, gen_type_predicate(ty))
                    } else {
                        format!(
                            "{}(:{}, {})",
                            LITERAL,
                            gen_type_predicate(ty),
                            gen_validates(validates)
                        )
                    };

                    if let Some(block) = block {
                        out.push_str(&gen_block(block, nesting));
                    }

                    out
                }
            }
        }
    }
}

fn gen_validates(validates: &[ir::Validate]) -> String {
    validates
        .iter()
        .map(gen_validate)
        .collect::<Vec<String>>()
        .join(", ")
}

fn gen_validate(validate: &ir::Validate) -> String {
    match validate {
        ir::Validate::Max(max) => format!("max: {max}"),
        ir::Validate::Min(min) => format!("min: {min}"),
        ir::Validate::MaxSize(max) => format!("max_size: {max}"),
        ir::Validate::MinSize(min) => format!("min_size: {min}"),
    }
}

fn indent(nesting: usize) -> String {
    const INDENT: &str = "  ";
    INDENT.repeat(nesting)
}

fn gen_def_name(name: &str) -> String {
    name.to_case(Case::Pascal)
}

fn gen_schema_class(schema_class: &ir::SchemaClass) -> String {
    match schema_class {
        ir::SchemaClass::Params => "Dry::Schema::Params".to_string(),
    }
}

fn gen_type_spec(ty: &ir::Type) -> String {
    match ty {
        ir::Type::Integer => "integer",
        ir::Type::String => "string",
        ir::Type::Boolean => "boolean",
        ir::Type::Array => "array",
    }
    .to_string()
}

fn gen_type_predicate(ty: &ir::Type) -> String {
    match ty {
        ir::Type::Integer => "int?",
        ir::Type::String => "str?",
        ir::Type::Boolean => "bool?",
        ir::Type::Array => "array?",
    }
    .to_string()
}

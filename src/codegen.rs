use convert_case::{Case, Casing};

use crate::ir_builder::ir;

pub fn generate(def: &ir::Def) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "{} = {}",
        gen_def_name(&def.name),
        gen_schema_class(&def.class)
    ));
    code.push(' ');
    code.push_str(&gen_block(&def.block, 1));

    code
}

fn indent(nesting: usize) -> String {
    const INDENT: &str = "  ";
    INDENT.repeat(nesting)
}

fn gen_block(block: &[ir::Stmt], nesting: usize) -> String {
    assert!(nesting > 0);

    let mut code = String::new();
    code.push_str("do\n");

    for stmt in block {
        code.push_str(&format!("{}{}\n", indent(nesting), gen_stmt(stmt)));
    }

    code.push_str(&format!("{}end\n", indent(nesting - 1)));

    code
}

fn gen_schema_class(schema_class: &ir::SchemaClass) -> String {
    match schema_class {
        ir::SchemaClass::Params => "Dry::Schema::Params".to_string(),
    }
}

fn gen_stmt(stmt: &ir::Stmt) -> String {
    match stmt {
        ir::Stmt::Required { name, r#macro } => {
            format!("required({name}).{}", gen_macro(r#macro))
        }
        ir::Stmt::Optional { name, r#macro } => {
            format!("optional({name}).{}", gen_macro(r#macro))
        }
    }
}

fn gen_macro(r#macro: &ir::Macro) -> String {
    match r#macro {
        ir::Macro::Value { ty, validates } => {
            let value = match ty {
                ir::Type::Integer => "value(:integer",
                ir::Type::String => "value(:string",
                ir::Type::Array => "value(:array",
            };

            if validates.is_empty() {
                format!("{})", value)
            } else {
                format!("{}, {})", value, gen_validates(validates))
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
        ir::Validate::MaxLength(max) => format!("max_size: {max}"),
        ir::Validate::MinLength(min) => format!("min_size: {min}"),
    }
}

fn gen_def_name(name: &str) -> String {
    name.to_case(Case::Pascal)
}

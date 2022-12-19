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
        code.push_str(&format!("{}{}\n", indent(nesting), gen_stmt(stmt, nesting)));
    }

    code.push_str(&format!("{}end\n", indent(nesting - 1)));

    code
}

fn gen_schema_class(schema_class: &ir::SchemaClass) -> String {
    match schema_class {
        ir::SchemaClass::Params => "Dry::Schema::Params".to_string(),
    }
}

fn gen_stmt(stmt: &ir::Stmt, nesting: usize) -> String {
    match stmt {
        ir::Stmt::Required { name, r#macro } => {
            format!("required(:{name}).{}", gen_macro(r#macro, nesting))
        }
        ir::Stmt::Optional { name, r#macro } => {
            format!("optional(:{name}).{}", gen_macro(r#macro, nesting))
        }
    }
}

fn gen_macro(r#macro: &ir::Macro, nesting: usize) -> String {
    match r#macro {
        ir::Macro::Value { ty } => match ty {
            ir::Type::Integer { validates } => {
                if validates.is_empty() {
                    "value(:integer)".to_string()
                } else {
                    format!("value(:integer, {})", gen_validates(validates))
                }
            }
            ir::Type::String { validates } => {
                if validates.is_empty() {
                    "value(:string)".to_string()
                } else {
                    format!("value(:string, {})", gen_validates(validates))
                }
            }
            ir::Type::Array { validates, item } => {
                let mut out = if validates.is_empty() {
                    "value(:array)".to_string()
                } else {
                    format!("value(:array, {})", gen_validates(validates))
                };

                if let Some(item) = item {
                    out.push_str(&gen_each(item, nesting));
                }

                out
            }
        },
    }
}

fn gen_each(each: &ir::Each, nesting: usize) -> String {
    match &each.ty {
        ir::Type::String { validates } => {
            if validates.is_empty() {
                ".each(:string)".to_string()
            } else {
                format!(".each(:string, {})", gen_validates(validates))
            }
        }
        ir::Type::Integer { validates } => {
            if validates.is_empty() {
                ".each(:integer)".to_string()
            } else {
                format!(".each(:integer, {})", gen_validates(validates))
            }
        }
        ir::Type::Array { validates, item } => {
            let mut out = if validates.is_empty() {
                ".each(:array)".to_string()
            } else {
                format!(".each(:array, {})", gen_validates(validates))
            };

            if let Some(item) = item {
                out.push_str(" do\n");
                out.push_str(&format!(
                    "{}{}\n",
                    indent(nesting + 1),
                    gen_schema_ty(&item.ty, nesting + 1)
                ));
                out.push_str(&format!("{}end", indent(nesting)));
            }

            out
        }
    }
}

fn gen_schema_ty(ty: &ir::Type, nesting: usize) -> String {
    match ty {
        ir::Type::Integer { validates } => {
            if validates.is_empty() {
                "schema(:int?)".to_string()
            } else {
                format!("schema(:int?, {})", gen_validates(validates))
            }
        }
        ir::Type::String { validates } => {
            if validates.is_empty() {
                "schema(:str?)".to_string()
            } else {
                format!("schema(:str?, {})", gen_validates(validates))
            }
        }
        ir::Type::Array { validates, item } => {
            if let Some(item) = item {
                let mut out = "schema(:array?)".to_string();
                if validates.is_empty() {
                    out.push_str(".each(:array?) do\n");
                } else {
                    out.push_str(&format!(
                        ".each(:array?, {}) do\n",
                        gen_validates(validates)
                    ));
                }

                out.push_str(&format!(
                    "{}{}\n",
                    indent(nesting + 1),
                    gen_schema_ty(&item.ty, nesting + 1)
                ));
                out.push_str(&format!("{}end", indent(nesting)));
                out
            } else {
                #[allow(clippy::collapsible_else_if)]
                if validates.is_empty() {
                    "schema(:array?)".to_string()
                } else {
                    format!("schema(:array?, {})", gen_validates(validates))
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

fn gen_def_name(name: &str) -> String {
    name.to_case(Case::Pascal)
}

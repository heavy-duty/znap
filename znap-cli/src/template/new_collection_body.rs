use heck::ToSnekCase;

pub fn template(name: &String) -> String {
    format!(
        r#"use znap::prelude::*;

#[collection]
pub mod {} {{
    use super::*;
}}
"#,
        name.to_snek_case(),
    )
}

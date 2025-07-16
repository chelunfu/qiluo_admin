use tera::{Context, Tera};

use super::error::Result;

pub fn render_string(tera_template: &str, locals: &serde_json::Value) -> Result<String> {
    let connt=Context::from_serialize(locals).unwrap();
    let text = Tera::one_off(tera_template, &connt, false).unwrap();
    Ok(text)
}

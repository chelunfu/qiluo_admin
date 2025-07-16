use crate::common::error::Result;
use crate::common::tera;
use fs_err as fs;
use std::env;

/// The filename for the subject template file.
const SUBJECT: &str = "subject.t";
/// The filename for the HTML template file.
const HTML: &str = "html.t";
/// The filename for the plain text template file.
const TEXT: &str = "text.t";

fn embedded_file(dir: String, name: &str) -> Result<String> {
    let path = env::current_dir().unwrap();
    let files = path.join(dir).join(name);
    let content = fs::read_to_string(files).expect("msg");
    Ok(content)
}

#[derive(Clone, Debug)]
pub struct Content {
    pub subject: String,
    pub text: String,
    pub html: String,
}

#[derive(Debug, Clone)]
pub struct Template {
    dir: String,
}

impl Template {
    pub const fn new(dir: String) -> Self {
        Self { dir }
    }

    pub fn render(&self, locals: &serde_json::Value) -> Result<Content> {
        let subject_t = embedded_file(self.dir.clone(), SUBJECT)?;
        let text_t = embedded_file(self.dir.clone(), TEXT)?;
        let html_t = embedded_file(self.dir.clone(), HTML)?;

        let text = tera::render_string(&text_t, locals);

        let text = text.unwrap();
        let html = tera::render_string(&html_t, locals)?;
        let subject = tera::render_string(&subject_t, locals)?;

        Ok(Content {
            subject,
            text,
            html,
        })
    }
}

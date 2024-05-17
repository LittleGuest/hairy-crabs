use crab_lib::lazy_static::lazy_static;
use tera::{Context, Tera};

fn init_templates() -> Tera {
    match Tera::new("templates/**/*") {
        Ok(mut ta) => {
            ta.autoescape_on(vec![".html", ".css", ".js"]);
            ta
        }
        Err(e) => panic!("模板初始化错误: {e}"),
    }
}

lazy_static! {
    static ref TMP: Tera = init_templates();
}

const TMP_INDEX: &str = "index.html";

pub fn render(data: serde_json::Value) -> String {
    TMP.render(TMP_INDEX, &Context::from_value(data).unwrap())
        .unwrap()
}

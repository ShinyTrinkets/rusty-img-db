use std::fs::{read_to_string, write};
use upon::{Engine, Value};

use crate::config::Config;
use crate::img::Img;

pub fn generate_gallery(imgs: Vec<Img>, cfg: &Config) {
    let tmpl = read_to_string("tmpl/img_gallery.html").unwrap();

    let mut engine = Engine::new();
    engine.add_filter("length", f_length);
    engine.add_filter("lower", str::to_lowercase);
    engine.add_filter("upper", str::to_uppercase);
    engine.add_filter("trim", |s: &str| s.trim().to_owned());

    let template = engine.compile(&tmpl).unwrap();
    let result = template
        .render(upon::value! {
            imgs: imgs,
            title: "img-DB gallery",
        })
        .unwrap();

    write(&cfg.output, result).unwrap();
}

fn f_length(value: &Value) -> Result<u32, String> {
    match value {
        Value::String(v) => Ok(v.len() as u32),
        Value::List(v) => Ok(v.len() as u32),
        v => Err(format!("unsupported type {v:?}")),
    }
}

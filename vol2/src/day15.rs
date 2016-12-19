extern crate markdown;
extern crate serde_json;
extern crate tera;

use std::collections::HashMap;
use tera::{Context, Tera, TeraResult, TeraError, Value, to_value};

macro_rules! try_get_value {
    ($filter_name:expr, $var_name:expr, $ty:ty, $val:expr) => {{
        match ::serde_json::value::from_value::<$ty>($val.clone()) {
            Ok(s) => s,
            Err(_) => {
                return Err(TeraError::FilterIncorrectArgType(
                    $filter_name.to_string(),
                    $var_name.to_string(),
                    $val,
                    stringify!($ty).to_string())
                );
            }
        }
    }};
}

pub fn markdown_filter(value: Value, _: HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("upper", "value", String, value);
    Ok(to_value(markdown::to_html(s.as_str())))
}

const LIPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
     labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
     laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
     voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
     cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";

fn main() {
    println!("24 days of Rust vol. 2 - tera");
    let mut tera = Tera::new("templates/**/*");
    let mut ctx = Context::new();
    ctx.add("title", &"hello world!");
    ctx.add("content", &LIPSUM);
    ctx.add("todos",
            &vec!["buy milk", "walk the dog", "write about tera"]);
    let rendered = tera.render("index.html", ctx).expect("Failed to render template");
    println!("{}", rendered);

    tera.register_filter("markdown", markdown_filter);
    let mut ctx = Context::new();
    ctx.add("content", &"**bold** and `beautiful`");
    let rendered = tera.render("blog.html", ctx).expect("Failed to render template");
    println!("{}", rendered);

    let mut config = HashMap::new();
    config.insert("hostname", "127.0.0.1");
    config.insert("user", "root");
    config.insert("email", "NAME@example.com");
    let mut ctx = Context::new();
    ctx.add("config", &config);
    let rendered = tera.render("config.ini", ctx).expect("Failed to render template");
    println!("{}", rendered);
}
# Day 15 - tera

[`tera`](https://crates.io/crates/tera) is a pure Rust templating engine.
Those familiar with [Django](https://www.djangoproject.com),
[Jinja2](http://jinja.pocoo.org) or [Twig](http://twig.sensiolabs.org/)
will feel right at home with `tera`, as its syntax is very similar.
`tera` is still very young and may not be as feature complete as, say, Jinja2.
However it is actively developed and the maintainer is very open to new
contributions, so feel free to
[tackle some issues](https://github.com/Keats/tera/issues)!

Basic templates
---------------

Let's use `tera` to render a simple HTML template.

Note: The templates are parsed when `Tera::new()` is called, so if any of
the templates contains invalid syntax, the function will panic right there.
Keep that in mind!

```rust
extern crate tera;

use tera::{Context, Tera};

const LIPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
     labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
     laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
     voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
     cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum";

fn main() {
    let tera = Tera::new("templates/**/*");
    let mut ctx = Context::new();
    ctx.add("title", &"hello world!");
    ctx.add("content", &LIPSUM);
    ctx.add("todos",
            &vec!["buy milk", "walk the dog", "write about tera"]);
    let rendered = tera.render("index.html", ctx).expect("Failed to render template");
    println!("{}", rendered);
}
```

We pass variables from Rust code to the template using a `Context` struct.
The `add()` method takes variable name and a reference to the value. Values
can be basic Rust types, vectors or custom structs (provided that they
implement the `Serialize` trait from `serde`.)

Our `index.html` looks like this:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <title>{{ title|title }}</title>
</head>
<body>
<h1>{{ title|title }}</h1>
<h2>{{ content|wordcount }} words</h2>
<p>{{ content }}</p>
{% if todos|length > 1 %}
    <ul>
    {% for todo in todos %}
        <li>{{ todo }}</li>
    {% endfor %}
    </ul>
{% endif %}
</body>
</html>
```

We use `{{ ... }}` in the templates to refer to variables, while `{% ... %}`
syntax is for flow control tags such as loops and blocks. The excellent
[Django docs](https://docs.djangoproject.com/en/1.10/ref/templates/language/)
describe this syntax in more detail. `tera` templates follow the same
conventions.

```text
$ cargo run
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <title>Hello World!</title>
</head>
<body>
<h1>Hello World!</h1>
<h2>69 words</h2>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum</p>
<ul>
    <li>buy milk</li>
    <li>walk the dog</li>
    <li>write about tera</li>
    </ul>
</body>
</html>
```

Custom template filters
-----------------------

Template filters are expressions that affect how variables are displayed.
We can apply them using the pipe character `|` after variable name, for
example `{{ content|lower }}`.
`tera` provides a selection of built-in template filters, such as `trim`,
`wordcount`, `pluralize` or `reverse`. But the power of template engines
lies in their extensibility. We can add our own template filters to cover
the specific needs of our application. (Or to make life easier for
frontend developers on the team.)

A template filter is just a Rust function that has the following type:

```rust
pub type FilterFn = fn(Value, HashMap<String, Value>) -> TeraResult<Value>;
```

Let's use the [`markdown`](https://crates.io/crates/markdown) crate
to render Markdown to HTML on the fly.

```rust
extern crate markdown;
extern crate tera;

use std::collections::HashMap;
use tera::{Context, Tera, TeraResult, Value, to_value};

pub fn markdown_filter(value: Value, _: HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("upper", "value", String, value);
    Ok(to_value(markdown::to_html(s.as_str())))
}
```

We're using here a `try_get_value!` macro, which isn't currently a part of
`tera`'s public API (because value serializing may change; at the moment `tera`
is using `serde_json`). I've copied it
[from `tera` sources](https://github.com/Keats/tera/blob/3471df41ab454c60a85ec271a945f7123705e49a/src/macros.rs#L6)
but decided to omit its definition from the example for clarity. We try
to read our input as a `String` and if that succeeds, we pass the string slice
to `markdown::to_html()`. The rest is just bookkeeping - convert the rendered
HTML back to a `Value` and wrap in `Ok`. Let's see it in action:

```rust
tera.register_filter("markdown", markdown_filter);
let mut ctx = Context::new();
ctx.add("content", &"**bold** and `beautiful`");
let rendered = tera.render("blog.html", ctx).expect("Failed to render template");
println!("{}", rendered);
```

And the template:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
</head>
<body>
    {{ content|markdown|safe }}
</body>
</html>
```

We can chain filters one after another. Here we're using the built-in `safe`
filter to disable auto-escaping of the generated HTML.

```text
$ cargo run
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
</head>
<body>
    <p><strong>bold</strong> and <code>beautiful</code></p>

</body>
</html>
```

Not only HTML
-------------

The Django template docs contain a following paragraph (near the beginning,
so that's important for them):

> Why use a text-based template instead of an XML-based one (like Zope’s TAL)?
> We wanted Django’s template language to be usable for more than just XML/HTML
> templates. At World Online, we use it for emails, JavaScript and CSV.
> You can use the template language for any text-based format.
>
> Oh, and one more thing: making humans edit XML is sadistic!

`tera` shares the same philosophy. We don't have to emit HTML and HTML only.
In the following example we're rendering a map of configuration values
into an `.ini` file:

```rust
let mut config = HashMap::new();
config.insert("hostname", "127.0.0.1");
config.insert("user", "root");
config.insert("email", "NAME@example.com");
let mut ctx = Context::new();
ctx.add("config", &config);
let rendered = tera.render("config.ini", ctx).expect("Failed to render template");
println!("{}", rendered);
```

`config.ini`:

```ini
[system]
{% if config.user != "anonymous" %}
user={{ config.user }}
{% endif %}

[network]
hostname={{ config.hostname }}
email={{ config.email|replace(from="NAME", to=config.user) }}
```

We can access `HashMap` members in the template using dot notation, as in
`config.user`. This example also demonstrates that filters can take extra
arguments, like `replace`.

And the result:

```text
$ cargo run
[system]
user=root

[network]
hostname=127.0.0.1
email=root@example.com
```


Further reading
---------------

 - [Introducing Tera, a template engine in Rust](https://blog.wearewizards.io/introducing-tera-a-template-engine-in-rust)
 - [Django templates documentation](https://docs.djangoproject.com/en/1.10/ref/templates/)
 - [Jinja2 documentation](http://jinja.pocoo.org/docs/dev/)
 - [dtl](https://crates.io/crates/dtl) - Django Template Language for Rust
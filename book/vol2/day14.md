# Day 14 - cursive

My first programming IDE back in the 90s was Borland Turbo Pascal. Since
the PC that I used at that time was running MS-DOS, it meant no graphical
interface. I was so surprised when I ran `turbo.exe` for the first time
and saw complex menus, dialog windows and an editor with code highlighting.

But the world of TUI (text-based user interfaces) doesn't mean only IDEs.
[Midnight Commander](https://www.midnight-commander.org/) is a very popular
and feature-rich file browser. There are even text-based web browsers,
such as [Lynx](http://lynx.invisible-island.net/) or
[ELinks](http://www.elinks.cz/).

TUI applications tend to use [ncurses](https://en.wikipedia.org/wiki/Ncurses)
as the abstraction layer over different terminals.
While there are Rust bindings to `ncurses`, there's another cool library built
on top of them. [`Cursive`](https://crates.io/crates/cursive) provides
high-level building blocks such as views, menus and layers. It also works
on Windows, when built with `features = ["pancurses"]`.

Application setup
-----------------

Every application using `Cursive` needs an event loop, implemented as the
`Cursive::run()` method. The example below shows a simple "Hello World"
program with `Cursive`:

```rust
extern crate cursive;

use cursive::Cursive;
use cursive::views::TextView;

fn main() {
    let mut app = Cursive::new();
    app.add_layer(TextView::new("Hello Rust"));
    app.add_global_callback('q', |a| a.quit());
    app.run();
}
```

The `TextView` is one of the simplest views in `Cursive`. You can set the
text directly in `new()` or later with `set_content()`. To quit our program,
the user has to press the `q` key which triggers a global callback (the
argument passed to callback is the `Cursive` object itself).

Let's run it!

![](http://wstaw.org/m/2016/12/14/hello.png)

File browser
------------

We're going to build a very minimalistic file browser with preview for text
files. Let's start with describing a general layout of the interface:

```rust
extern crate cursive;

use cursive::Cursive;
use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};

use std::fs::{self, DirEntry, File};
use std::io::Read;
use std::path::Path;

fn main() {
    let mut app = Cursive::new();
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker.fixed_size((30, 25)));
    panes.add_child(DummyView);
    panes.add_child(TextView::new("file contents")
        .with_id("contents")
        .fixed_size((50, 25)));
    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    layout.add_child(TextView::new("status")
        .scrollable(false)
        .with_id("status")
        .fixed_size((80, 1)));
    app.add_layer(Dialog::around(layout).button("Quit", |a| a.quit()));
    app.run();
}
```

Here's our file browser displaying contents of `Cargo.lock`:

![](http://wstaw.org/m/2016/12/14/browser.png)

The app consists of two vertical panes with a status bar below and a *Quit*
button in the bottom right corner. Left pane contains a file picker. On the
right there is a `TextView` to show preview of the selected file.
The `DummyView` acts only as a separator. Note the `with_id()` calls.
We give textual IDs to some controls so that we can find them later from
inside the event handlers.

And now the directory listing:

```rust
fn file_picker<D>(directory: D) -> SelectView<DirEntry>
    where D: AsRef<Path>
{
    let mut view = SelectView::new();
    for entry in fs::read_dir(directory).expect("can't read directory") {
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e);
        }
    }
    view.on_select(update_status).on_submit(load_contents)
}
```

The file picker returns a `SelectView` which is a list of items (directory
entries in this example). There are two events that can happen here. The user
can select an item (by navigating with arrow keys) or they can press Enter
to run the callback registered in `on_submit`. Here are the event handlers
for both of these events:

```rust
fn update_status(app: &mut Cursive, entry: &DirEntry) {
    let status_bar = app.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let file_size = entry.metadata().unwrap().len();
    let content = format!("{}: {} bytes", file_name, file_size);
    status_bar.set_content(content);
}

fn load_contents(app: &mut Cursive, entry: &DirEntry) {
    let text_view = app.find_id::<TextView>("contents").unwrap();
    let content = if entry.metadata().unwrap().is_dir() {
        "<DIR>".to_string()
    } else {
        let mut buf = String::new();
        let _ = File::open(entry.file_name())
            .and_then(|mut f| f.read_to_string(&mut buf))
            .map_err(|e| buf = format!("Error: {}", e));
        buf
    };
    text_view.set_content(content);
}
```

When the user selects a file, the status bar updates to show its name and size.
If the user presses Enter on a file, the preview pane changes its contents.
Each of the callbacks takes a `Cursive` object and an item from the
`SelectView` which caused the event. We're using `Cursive::find_id()`
to find controls by their IDs defined in the application layout.

There are several possible things to improve here:

 - change into selected directory instead of showing `<DIR>`
 - skip preview for binary files
 - avoid panics on errors - most of these `unwrap()` calls should be replaced
   with proper error handling. `load_contents()` attempts that when reading
   file contents, but not anywhere else.


Further reading
---------------

 - [Discovering ncurses, the GUI for the Linux Console](http://www.linuxplanet.com/linuxplanet/reviews/6964/1)
 - [Comparing Text-based and Graphic User Interfaces for Novice and Expert Users](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC2655855/)
 - [Urwid](http://urwid.org/) - a console UI framework (for Python)
 - [rustbox](https://crates.io/crates/rustbox) - a Rust wrapper for [termbox](https://github.com/nsf/termbox)

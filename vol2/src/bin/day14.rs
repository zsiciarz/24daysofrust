#[cfg(target_family = "unix")]
extern crate cursive;

#[cfg(target_family = "unix")]
use cursive::Cursive;
#[cfg(target_family = "unix")]
use cursive::traits::*;
#[cfg(target_family = "unix")]
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};

#[cfg(target_family = "unix")]
use std::fs::{self, DirEntry, File};
#[cfg(target_family = "unix")]
use std::io::Read;
#[cfg(target_family = "unix")]
use std::path::Path;

#[cfg(target_family = "windows")]
fn main() {
    println!("TODO");
}

#[cfg(target_family = "unix")]
fn file_picker<D>(directory: D) -> SelectView<DirEntry>
where
    D: AsRef<Path>,
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

#[cfg(target_family = "unix")]
fn update_status(app: &mut Cursive, entry: &DirEntry) {
    let mut status_bar = app.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let file_size = entry.metadata().unwrap().len();
    let content = format!("{}: {} bytes", file_name, file_size);
    status_bar.set_content(content);
}

#[cfg(target_family = "unix")]
fn load_contents(app: &mut Cursive, entry: &DirEntry) {
    let mut text_view = app.find_id::<TextView>("contents").unwrap();
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

#[cfg(target_family = "unix")]
fn main() {
    println!("24 Days of Rust vol. 2 - cursive");
    let mut app = Cursive::new();
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker.fixed_size((30, 25)));
    panes.add_child(DummyView);
    panes.add_child(
        TextView::new("file contents")
            .with_id("contents")
            .fixed_size((50, 25)),
    );
    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    layout.add_child(
        TextView::new("status")
            .scrollable(false)
            .with_id("status")
            .fixed_size((80, 1)),
    );
    app.add_layer(Dialog::around(layout).button("Quit", |a| a.quit()));
    app.run();
}

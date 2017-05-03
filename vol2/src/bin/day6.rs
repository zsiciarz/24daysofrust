#[macro_use]
extern crate derive_builder;

#[derive(Clone, Debug)]
struct Resolution {
    width: u32,
    height: u32,
}

impl Default for Resolution {
    fn default() -> Resolution {
        Resolution {
            width: 1920,
            height: 1080,
        }
    }
}

#[derive(Debug, Default, Builder)]
#[builder(field(private), setter(into))]
struct GameConfig {
    #[builder(default)]
    resolution: Resolution,
    save_dir: Option<String>,
    #[builder(default)]
    autosave: bool,
    fov: f32,
    render_distance: u32,
}

fn main() {
    println!("24 Days of Rust vol. 2 - derive_builder");
    let conf = GameConfigBuilder::default()
        .save_dir("saves".to_string())
        .fov(70.0)
        .render_distance(1000u32)
        .build()
        .unwrap();
    println!("{:?}", conf);
}

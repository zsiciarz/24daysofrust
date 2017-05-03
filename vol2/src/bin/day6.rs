#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate derive_builder;

#[derive(Debug)]
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

custom_derive! {
    #[derive(Debug, Default, Builder)]
    struct GameConfig {
        resolution: Resolution,
        save_dir: Option<String>,
        autosave: bool,
        fov: f32,
        render_distance: u32,
    }
}

fn main() {
    println!("24 Days of Rust vol. 2 - derive_builder");
    let mut conf = GameConfig::default();
    conf.save_dir("saves".to_string()).fov(70.0).render_distance(1000u32);
    println!("{:?}", conf);
}

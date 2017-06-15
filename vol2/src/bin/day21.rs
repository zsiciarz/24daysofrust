extern crate app_dirs;
extern crate preferences;

#[macro_use]
extern crate serde_derive;

use app_dirs::{AppDataType, AppInfo, app_dir, app_root, get_app_root};
use preferences::Preferences;
use std::path::PathBuf;

const APP_INFO: AppInfo = AppInfo {
    name: "24daysofrust",
    author: "Zbigniew Siciarz",
};

#[derive(Serialize, Deserialize, Debug, Default)]
struct GameConfig {
    save_dir: Option<PathBuf>,
    autosave: bool,
    fov: f32,
    render_distance: u32,
}

fn main() {
    println!("24 Days of Rust vol. 2 - app_dirs");
    println!("{:?}", get_app_root(AppDataType::UserConfig, &APP_INFO));
    println!("{:?}", app_root(AppDataType::UserConfig, &APP_INFO));
    let save_dir = app_dir(AppDataType::UserData, &APP_INFO, "game/saves")
        .expect("Couldn't create directory for game saves");
    println!("{}", save_dir.display());
    let mut config = match GameConfig::load(&APP_INFO, "game_config") {
        Ok(cfg) => cfg,
        Err(_) => GameConfig::default(),
    };
    println!("{:?}", config);
    config.save_dir = Some(save_dir);
    config.autosave = true;
    config.save(&APP_INFO, "game_config").expect(
        "Failed to save game config",
    );
}

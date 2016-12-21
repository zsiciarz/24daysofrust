# Day 21 - app_dirs and preferences

Today we're going to take a brief look at two crates from the same author -
[Andy Barron](https://github.com/AndyBarron). The first of them is
[`app_dirs`](https://crates.io/crates/app_dirs) - a useful library
to find platform-dependent directories, such as application configuration,
data directory or cache. The second crate for today is
[`preferences`](https://crates.io/crates/preferences), which provides a
simple way of managing user preferences and other data relevant to our program.

Application directories
-----------------------

Let's think for a moment about application-specific data. For example,
where on the filesystem should we put persistent files like game saves,
download cache or any non-default configuration? We could try storing them
in the same directory as the executable itself. While this approach
makes for portable installations (just copy the entire directory),
it runs into issues with permissions. If you've installed the program
with `sudo` on Linux, you certainly shouldn't be able to mess inside
`/usr/bin` as a regular non-root user. Fortunately every popular operating
system has a concept of per-user data and configuration directories.
On Windows this would be some subdirectory of `C:\Users\<username>\AppData`,
while Linux uses `/home/<username>/.config`
(see [XDG specification](https://developer.gnome.org/basedir-spec/) for
details).

The `app_dirs` crate makes it easy to access or create these directories
in a cross-platform way.

```rust
extern crate app_dirs;

use app_dirs::{AppDataType, AppInfo, app_root, get_app_root};

const APP_INFO: AppInfo = AppInfo {
    name: "24daysofrust",
    author: "Zbigniew Siciarz",
};

fn main() {
    println!("{:?}", get_app_root(AppDataType::UserConfig, &APP_INFO));
    println!("{:?}", app_root(AppDataType::UserConfig, &APP_INFO));
}
```

We need to create an `AppInfo` value up front. The docs for `app_dirs`
recommend a single const instance, so that's what we're doing here. The
application and author names may be used as path fragments. In the `main()`
function we're requesting directories for a specific type of data. It can be
one of `UserConfig`, `UserData`, `UserCache`, `SharedConfig` or `SharedData`.

```text
$ cargo run
Ok("C:\\Users\\USER\\AppData\\Roaming\\Zbigniew Siciarz\\24daysofrust")
Ok("C:\\Users\\USER\\AppData\\Roaming\\Zbigniew Siciarz\\24daysofrust")
```

Both function calls returned the same path. So what's the difference?

`get_` variants only build the path, but **do not** touch the filesystem at all.
There is no guarantee that the directory returned by `get_app_root()` exists.
To actually create the directory (if it doesn't exist), use `app_root()`.

We can also use `app_dir()` to directly create a subdirectory under one of the
app-specific data directories:

```rust
use app_dirs::app_dir;

let save_dir = app_dir(AppDataType::UserData, &APP_INFO, "game/saves")
    .expect("Couldn't create directory for game saves");
println!("{}", save_dir.display());
```

```text
$ cargo run
C:\Users\USER\AppData\Roaming\Zbigniew Siciarz\24daysofrust\game\saves
```

Application preferences
-----------------------

The `preferences` crate builds upon `app_dirs` to provide a somewhat
opinionated approach to storing program configuration. If we have a
serializable struct, it can be saved in an application-specific configuration
directory as a JSON-encoded file. Let's revisit the `GameConfig` example that
already appeared earlier in this series:

```rust
extern crate preferences;
extern crate rustc_serialize;

use std::path::PathBuf;
use preferences::Preferences;

#[derive(RustcEncodable, RustcDecodable, Debug, Default)]
struct GameConfig {
    save_dir: Option<PathBuf>,
    autosave: bool,
    fov: f32,
    render_distance: u32,
}

let mut config = match GameConfig::load(&APP_INFO, "game_config") {
    Ok(cfg) => cfg,
    Err(_) => GameConfig::default(),
};
println!("{:?}", config);
config.save_dir = Some(save_dir);
config.autosave = true;
config.save(&APP_INFO, "game_config").expect("Failed to save game config");
```

Deriving the serialization traits automatically implements another trait -
`Preferences`. It provides the `load()` and `save()` methods that we use
to persist our game configuration.
We're reusing the same `AppInfo` value as before, so that the underlying
calls to `app_dirs` API know where to find our application directories.

**Note**: `preferences` is going to move to `serde` once custom derive
lands in stable Rust. But that doesn't mean a lot of upgrade work for us. Only
the trait names will change to `Serialize` and `Deserialize`.

```text
$ cargo run
GameConfig { save_dir: None, autosave: false, fov: 0, render_distance: 0 }

$ cargo run
GameConfig { save_dir: Some("C:\\Users\\USER\\AppData\\Roaming\\Zbigniew Siciarz\\24daysofrust\\game\\saves"), autosave: false, fov: 0, render_distance: 0 }
```

Running the program for the first time ever used default values for the
configuration. However, in the second run the configuration file already exists
on disk, so it's loaded and printed out.

There's also a `PreferencesMap` type, which is just a `HashMap` that knows how
to load and store itself in a JSON file. It's particularily useful when you
need to persist key-value data.

Further reading
---------------

 - [XDG Base Directory Specification](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html)
 - [What's in the hidden Windows AppData Folder, and how to find it if you need it](http://www.pcworld.com/article/2690709/whats-in-the-hidden-windows-appdata-folder-and-how-to-find-it-if-you-need-it.html)
 - [`jconfig`](https://crates.io/crates/jconfig)  and [`toml-config`](https://crates.io/crates/toml-config) -
   similar crates to access settings stored as JSON or TOML
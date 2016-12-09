# Day 9 - winreg

Fact: I'm writing these articles and examples on a Windows machine and
so far everything compiles and works as expected. Just so you know, Rust
[supports Windows in the top tier](https://forge.rust-lang.org/platform-support.html).
I'm mentioning it here since a few people I talked to assumed Windows support
was sort of secondary, bolted-on later. This is **not** the case.

The library ecosystem also supports different operating systems fairly well.
There are even cross-platform crates for stuff usually associated with Linux,
such as [curses](https://github.com/ihalila/pancurses) or
[coreutils](https://github.com/uutils/coreutils).
However, some crates support only Linux or Windows by design. One of them is
[`winreg`](https://crates.io/crates/winreg) - a Rust API to access
Windows Registry.

Read a single value
-------------------

The Registry is a tree of keys, similar to a filesystem with folders.
Keys may contain other keys or values.

```rust
extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
let subkey =
    hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ)
        .expect("Failed to open subkey");
let product_name: String = subkey.get_value("ProductName")
    .expect("Failed to read product name");
println!("Windows version: {}", product_name);
```

```text
$ cargo run
Windows version: Windows 7 Home Premium
```

`HKEY_LOCAL_MACHINE` is one of the predefined root keys that we use as a
starting point to find `SOFTWARE\Microsoft\Windows NT\CurrentVersion`.
(We're using raw string literals here so that we don't have to escape
backslashes.) This key contains a whole lot of subkeys, but also several values
such as OS version, Service Pack version, build numer etc.

List installed programs
-----------------------

```rust
let subkey =
    hklm.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Uninstall"#,
                                KEY_READ)
        .expect("Failed to open subkey");
for key in subkey.enum_keys() {
    let _ = key.and_then(|key| subkey.open_subkey_with_flags(key, KEY_READ))
        .and_then(|program_key| program_key.get_value("DisplayName"))
        .and_then(|name: String| {
            println!("{}", name);
            Ok(())
        });
}
```

```text
$ cargo run
Git version 2.9.0
HashTab 4.0.0.2
Microsoft Security Essentials
Intel PROSet Wireless
Stellarium 0.12.0
... and a lot more
```

Note: this is an incomplete list. See this MS technet answer for a **much more**
detailed approach:
[Where does Add/Remove Programs get its information from in the registry](https://social.technet.microsoft.com/Forums/windows/en-US/d913471a-d7fb-448d-869b-da9025dcc943/where-does-addremove-programs-get-its-information-from-in-the-registry?forum=w7itprogeneral)

Tweaking the system
-------------------

**Note**: you'll probably want to
[back up your Registry](https://support.microsoft.com/en-us/kb/322756) before
making changes to it.

Windows 7 introduced thumbnail previews for taskbar icons, showing the contents
of corresponding windows. In the screenshot below I moved the mouse cursor over
Firefox icon and after a small delay a thumbnail appeared.

![](http://wstaw.org/m/2016/12/07/thumbnails.png)

Yes, there is a small delay. But we can tweak it using the Registry.
[Here's a short tutorial](http://www.askvg.com/how-to-adjust-taskbar-thumbnail-delay-time-in-windows-7/)
on how to do it by hand, but we're going to use Rust and `winreg`.

```rust
    let delay = 100u32; // in milliseconds
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let subkey =
        hkcu.open_subkey_with_flags(r#"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"#,
                                    KEY_WRITE)
            .expect("Failed to open subkey");
    subkey.set_value("ExtendedUIHoverTime", &delay).expect("Failed to change thumbnail timeout");
```

If we navigate using `regedit.exe` to the key mentioned above, we'll notice
there's now an `ExtendedUIHoverTime` value among others. That's the new timeout!
Restart `explorer.exe` or log out and back in to see the change in delay.

Further reading
---------------

 - [Installing Rust on Windows](https://facility9.com/2016/03/installing-rust-on-windows/) - includes setting up OpenSSL and a C/C++ compiler
 - [Useful Windows Registry Keys](http://www.jasinskionline.com/TechnicalWiki/Useful-Windows-Registry-Keys.ashx)


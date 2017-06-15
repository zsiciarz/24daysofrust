#[cfg(target_family = "windows")]
extern crate winreg;

#[cfg(target_family = "windows")]
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};

#[cfg(not(target_family = "windows"))]
fn main() {
    println!("winreg is Windows-only");
}

#[cfg(target_family = "windows")]
fn main() {
    println!("24 Days of Rust vol. 2 - winreg");
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey =
        hklm.open_subkey_with_flags(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, KEY_READ)
            .expect("Failed to open subkey");
    let product_name: String = subkey.get_value("ProductName").expect(
        "Failed to read product name",
    );
    println!("Windows version: {}", product_name);

    let subkey = hklm.open_subkey_with_flags(
        r#"Software\Microsoft\Windows\CurrentVersion\Uninstall"#,
        KEY_READ,
    ).expect("Failed to open subkey");
    for key in subkey.enum_keys() {
        let _ = key.and_then(|key| subkey.open_subkey_with_flags(key, KEY_READ))
            .and_then(|program_key| program_key.get_value("DisplayName"))
            .and_then(|name: String| {
                println!("{}", name);
                Ok(())
            });
    }

    let delay = 100u32; // in milliseconds
    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let subkey = hkcu.open_subkey_with_flags(
        r#"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"#,
        KEY_WRITE,
    ).expect("Failed to open subkey");
    subkey.set_value("ExtendedUIHoverTime", &delay).expect(
        "Failed to change thumbnail timeout",
    );
}

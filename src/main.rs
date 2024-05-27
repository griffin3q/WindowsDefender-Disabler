extern crate winreg;
use std::io;
use winreg::enums::*;
use winreg::RegKey;

fn disable_windows_defender() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SOFTWARE\\Policies\\Microsoft\\Windows Defender";
    let (key, disp) = hklm.create_subkey(path)?;
    
    key.set_value("DisableAntiSpyware", &1u32)?;
    
    println!("Windows Defender has been disabled. Please restart your computer for the changes to take effect.");
    Ok(())
}

fn main() {
    if let Err(e) = disable_windows_defender() {
        eprintln!("Failed to disable Windows Defender: {}", e);
    }
}

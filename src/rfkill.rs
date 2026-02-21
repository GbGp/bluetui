use std::fs;

use crate::app::AppResult;

pub fn check_blocked(name: &String) -> AppResult<bool> {
    if let Ok(entries) = fs::read_dir("/sys/class/rfkill/") {
        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if let Some(_file_name) = entry_path.file_name() {
                let devtype = fs::read_to_string(entry_path.join("type"))?;
                let devname = fs::read_to_string(entry_path.join("name"))?;

                if (devtype.trim() == "bluetooth") && (devname.trim() == name) {
                    let state_path = entry_path.join("state");
                    let state = fs::read_to_string(state_path)?.trim().parse::<u8>()?;

                    // https://www.kernel.org/doc/Documentation/ABI/stable/sysfs-class-rfkill
                    match state {
                        0 => {return Ok(true)}
                        2 => {return Ok(true)}
                        _ => {return Ok(false)}
                    }
                }
            }
        }
    }
    Ok(true)
}

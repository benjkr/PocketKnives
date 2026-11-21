//! The suite's own settings: the look, which blades start at sign-in, and the blade table.
//!
//! A preview (`pk --preview`, which sets `POCKETKNIVES_SETTINGS_PREVIEW=1` for everything it
//! starts) keeps all of them in `preview-settings.txt` next to the exe instead, and never stops a
//! blade: the blades' own stop commands would find the user's installed copies.
use crate::{Result, files, join, process, shell_link};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use windows_sys::Win32::System::Registry::HKEY_CURRENT_USER;
use windows_sys::Win32::UI::Shell::FOLDERID_Startup;

pub const TITLE: &str = "PocketKnives Settings";
pub const GENERAL_NAME: &str = "General";
pub const GENERAL_DESCRIPTION: &str = "Settings for the whole suite.";
pub const BLADES_LABEL: &str = "Blades";
pub const LOOK_LABEL: &str = "Look";
pub const PREVIEW_VARIABLE: &str = "POCKETKNIVES_SETTINGS_PREVIEW";
const SUITE_KEY: &str = "Software\\PocketKnives";
const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const PREVIEW_FILE: &str = "preview-settings.txt";
/// The time a blade gets to change its sign-in entry.
const AUTOSTART_TIMEOUT_MS: u32 = 5000;

/// The settings window's look.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Look {
    Xp = 0,
    Windows10 = 1,
    Windows11 = 2,
}

/// The looks' names, in [`Look`] order.
pub const LOOK_NAMES: [&str; 3] = ["XP", "Windows 10", "Windows 11"];
/// How each look is saved, in [`Look`] order.
const LOOK_VALUES: [&str; 3] = ["xp", "win10", "win11"];

impl Look {
    pub fn from_index(i: usize) -> Option<Look> {
        [Look::Xp, Look::Windows10, Look::Windows11].get(i).copied()
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// A saved value; anything unknown (or nothing) is XP.
    fn parse(value: Option<&str>) -> Look {
        LOOK_VALUES.iter().position(|v| Some(*v) == value).and_then(Look::from_index).unwrap_or(Look::Xp)
    }
}

/// A blade as the settings windows know it. The commands follow docs/specs/blade-settings.md.
pub struct Blade {
    pub name: &'static str,
    pub description: &'static str,
    /// Its icon's file name in the `icons` folder.
    pub icon: &'static str,
    pub exe: &'static str,
    pub stop_command: &'static str,
    pub settings_command: &'static str,
    pub set_command: &'static str,
}

/// The blades, in the order the windows list them. SnipSnap starts at sign-in through a Startup
/// shortcut, QuickTooth and NowPlaying through their own `--autostart` commands.
pub const BLADES: [Blade; 3] = [
    Blade {
        name: "SnipSnap",
        description: "Copies a rectangle, a window or the screen to the clipboard.",
        icon: "snipsnap.ico",
        exe: "snipsnap.exe",
        stop_command: "quit",
        settings_command: "settings",
        set_command: "set",
    },
    Blade {
        name: "QuickTooth",
        description: "Connects or disconnects your AirPods with one click.",
        icon: "quicktooth.ico",
        exe: "QuickTooth.exe",
        stop_command: "--exit",
        settings_command: "--settings",
        set_command: "--set",
    },
    Blade {
        name: "NowPlaying",
        description: "Shows the current media cover, seekable timeline and frequency meter in the taskbar.",
        icon: "nowplaying.ico",
        exe: "NowPlaying.exe",
        stop_command: "--exit",
        settings_command: "--settings",
        set_command: "--set",
    },
];
const SNIPSNAP: usize = 0;
const QUICKTOOTH: usize = 1;
const NOWPLAYING: usize = 2;

/// Which blades start at sign-in, in [`BLADES`] order.
pub type Startup = [bool; BLADES.len()];

/// The PocketKnives folder (the one with `pk.exe` and the blades) and whether this is a preview.
pub struct Suite {
    pub root: String,
    pub preview: bool,
}

impl Suite {
    pub fn new(root: String, preview: bool) -> Suite {
        Suite { root, preview }
    }

    /// The folder given, a preview when `POCKETKNIVES_SETTINGS_PREVIEW=1` is set (`pk --preview`
    /// sets it for everything it starts).
    pub fn from_env(root: String) -> Suite {
        Suite::new(root, pk::env::settings_preview())
    }

    pub fn blade_exe(&self, blade: &Blade) -> String {
        join(&self.root, blade.exe)
    }

    pub fn blade_icon(&self, blade: &Blade) -> String {
        join(&join(&self.root, "icons"), blade.icon)
    }

    /// The picture of one choice of a `picture` setting, `icons\<blade>\<key>.<value>.ico`: the
    /// blade writes it there when the suite is built (docs/specs/blade-settings.md).
    pub fn setting_picture(&self, blade: &Blade, key: &str, value: &str) -> String {
        join(&join(&join(&self.root, "icons"), blade.name), &alloc::format!("{key}.{value}.ico"))
    }

    /// The Windows 11 settings app.
    pub fn winui_exe(&self) -> String {
        join(&join(&self.root, "winui"), "PocketKnives.Settings.exe")
    }

    fn preview_path(&self) -> String {
        join(&self.root, PREVIEW_FILE)
    }

    /// The preview file's lines (look, then one per blade); defaults unless it has every line.
    fn read_preview(&self) -> Result<Vec<String>> {
        let expected = 1 + BLADES.len();
        match files::read_lines(&self.preview_path())? {
            Some(lines) if lines.len() == expected => Ok(lines),
            _ => {
                let mut out = vec![String::from(LOOK_VALUES[0])];
                for _ in 0..BLADES.len() {
                    out.push(String::from("on"));
                }
                Ok(out)
            }
        }
    }

    pub fn read_look(&self) -> Result<Look> {
        if self.preview {
            let preview = self.read_preview()?;
            return Ok(Look::parse(preview.first().map(String::as_str)));
        }
        Ok(Look::parse(pk::reg::string(HKEY_CURRENT_USER, Some(SUITE_KEY), "Look").as_deref()))
    }

    pub fn save_look(&self, look: Look) -> Result<()> {
        let value = LOOK_VALUES[look.index()];
        if self.preview {
            let mut preview = self.read_preview()?;
            preview[0] = String::from(value);
            return files::write_lines(&self.preview_path(), &preview);
        }
        let key = pk::reg::Key::create(HKEY_CURRENT_USER, SUITE_KEY).map_err(pk::error::message)?;
        key.set_string("Look", value).map_err(pk::error::message)
    }

    pub fn read_startup(&self) -> Result<Startup> {
        if self.preview {
            let preview = self.read_preview()?;
            return Ok([preview[1] == "on", preview[2] == "on", preview[3] == "on"]);
        }
        let qt_run = pk::reg::string(HKEY_CURRENT_USER, Some(RUN_KEY), BLADES[QUICKTOOTH].name);
        let np_run = pk::reg::string(HKEY_CURRENT_USER, Some(RUN_KEY), BLADES[NOWPLAYING].name);
        Ok([
            files::exists(&startup_shortcut()),
            qt_run.is_some_and(|v| !v.trim().is_empty()),
            np_run.is_some_and(|v| !v.trim().is_empty()),
        ])
    }

    /// Applies only the rows that changed, so an unchanged row doesn't overwrite a change made
    /// meanwhile (QuickTooth's own tray menu). `original` follows each row as it's applied.
    pub fn apply_startup(&self, original: &mut Startup, selected: &Startup) -> Result<()> {
        if original == selected {
            return Ok(());
        }
        if self.preview {
            let mut preview = self.read_preview()?;
            for i in 0..selected.len() {
                if original[i] != selected[i] {
                    preview[i + 1] = String::from(if selected[i] { "on" } else { "off" });
                }
            }
            files::write_lines(&self.preview_path(), &preview)?;
            *original = *selected;
            return Ok(());
        }

        if original[SNIPSNAP] != selected[SNIPSNAP] {
            if selected[SNIPSNAP] {
                let target = self.blade_exe(&BLADES[SNIPSNAP]);
                if !files::exists(&target) {
                    return Err(String::from("SnipSnap is missing from the PocketKnives folder."));
                }
                shell_link::save(&target, &startup_shortcut())?;
            } else {
                files::delete(&startup_shortcut())?;
            }
            original[SNIPSNAP] = selected[SNIPSNAP];
        }

        for idx in [QUICKTOOTH, NOWPLAYING] {
            if original[idx] != selected[idx] {
                let blade = &BLADES[idx];
                let exe = self.blade_exe(blade);
                let failed = || alloc::format!("{} could not update its sign-in setting.", blade.name);
                let child = process::start(&exe, &["--autostart", if selected[idx] { "on" } else { "off" }]).map_err(|_| failed())?;
                if child.wait(AUTOSTART_TIMEOUT_MS) != Some(0) {
                    return Err(failed());
                }
                original[idx] = selected[idx];
            }
        }
        Ok(())
    }

    /// Asks each blade to stop, through its own stop command (Exit in the tray menu). A preview
    /// never does: the commands find the running copy by window or mutex, not by path, so they
    /// would stop the user's installed blades. A blade that can't be asked keeps running.
    pub fn stop_blades(&self) {
        if self.preview {
            return;
        }
        for blade in &BLADES {
            let exe = self.blade_exe(blade);
            if files::exists(&exe) {
                let _ = process::start(&exe, &[blade.stop_command]);
            }
        }
    }
}

/// SnipSnap's sign-in shortcut, in the user's Startup folder.
fn startup_shortcut() -> String {
    join(&pk::env::known_folder(&FOLDERID_Startup).unwrap_or_default(), &alloc::format!("{}.lnk", BLADES[SNIPSNAP].name))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A preview never reaches the registry, the Startup folder or a blade: everything it changes
    /// is `preview-settings.txt` in its own folder.
    #[test]
    fn preview_stays_in_its_folder() {
        let dir = std::env::temp_dir().join(std::format!("runner-core-preview-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let suite = Suite::new(String::from(dir.to_str().unwrap()), true);
        let file = dir.join(PREVIEW_FILE);

        // Defaults without a file, and with one that hasn't all lines.
        assert_eq!(suite.read_look().unwrap(), Look::Xp);
        assert_eq!(suite.read_startup().unwrap(), [true, true, true]);
        std::fs::write(&file, "win11\r\noff\r\n").unwrap();
        assert_eq!(suite.read_look().unwrap(), Look::Xp);

        std::fs::write(&file, "win10\r\noff\r\non\r\noff\r\n").unwrap();
        assert_eq!(suite.read_look().unwrap(), Look::Windows10);
        assert_eq!(suite.read_startup().unwrap(), [false, true, false]);

        suite.save_look(Look::Windows11).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"win11\r\noff\r\non\r\noff\r\n");

        let mut original = [false, true, false];
        suite.apply_startup(&mut original, &[true, false, true]).unwrap();
        assert_eq!(original, [true, false, true]);
        assert_eq!(std::fs::read(&file).unwrap(), b"win11\r\non\r\noff\r\non\r\n");

        // Unchanged rows aren't written: the file keeps what's there.
        std::fs::write(&file, "xp\r\noff\r\noff\r\noff\r\n").unwrap();
        let mut original = [true, true, true];
        suite.apply_startup(&mut original, &[true, true, true]).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"xp\r\noff\r\noff\r\noff\r\n");
        let mut original = [true, true, true];
        suite.apply_startup(&mut original, &[true, false, true]).unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"xp\r\noff\r\noff\r\noff\r\n");

        // A preview's Exit leaves the blades alone, even with their exes in its folder.
        std::fs::write(dir.join("snipsnap.exe"), b"not a program").unwrap();
        std::fs::write(dir.join("QuickTooth.exe"), b"not a program").unwrap();
        std::fs::write(dir.join("NowPlaying.exe"), b"not a program").unwrap();
        suite.stop_blades();

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn looks() {
        assert_eq!(Look::parse(Some("win10")), Look::Windows10);
        assert_eq!(Look::parse(Some("Win10")), Look::Xp);
        assert_eq!(Look::parse(None), Look::Xp);
        assert_eq!(Look::from_index(2), Some(Look::Windows11));
        assert_eq!(Look::from_index(3), None);
        assert_eq!(LOOK_NAMES[Look::Windows10.index()], "Windows 10");
    }
}

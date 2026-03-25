#![windows_subsystem = "windows"]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::{env, thread, time};
use winreg::enums::*;
use winreg::RegKey;

const LAUNCHER_BYTES: &[u8] = include_bytes!(r"..\Launcher\target\release\Launcher.exe");
const LAUNCHER_FILE_NAME: &str = "launcher.exe";
const GAME_EXE_NAME: &str = "crispy-doom.exe";

fn make_install_path() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("Не удалось получить путь к AppData");
    path.push(r"Programs\Doom");
    path
}

fn main() {
    if check_registry() != Ok(()) {
        copy_to_startup().expect("Не удалось скопировать программу в автозагрузку");

        loop {
            checkfiles(&make_install_path()).expect("Не удалось обновить файлы приложения");
            thread::sleep(time::Duration::from_secs(120));
        }
    }
}

fn check_registry() -> Result<(), String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (subkey, _) = hklm
        .create_subkey(r"Software\MineController\Stop")
        .expect("Не удалось создать ключ");

    let value: String = subkey
        .get_value("Stop")
        .unwrap_or_else(|_| "Не найдено".to_string());

    if value == "True" {
        Ok(())
    } else {
        Err("All is good".to_string())
    }
}

fn checkfiles(path: &Path) -> Result<(), String> {
    if path.is_file() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }

    fs::create_dir_all(path).map_err(|e| e.to_string())?;

    let launcher_path = path.join(LAUNCHER_FILE_NAME);
    write_file_if_changed(&launcher_path, LAUNCHER_BYTES).map_err(|e| e.to_string())?;

    let desktop_launcher_path = desktop_launcher_path().map_err(|e| e.to_string())?;
    write_file_if_changed(&desktop_launcher_path, LAUNCHER_BYTES).map_err(|e| e.to_string())?;

    let source_doom_dir = resolve_source_doom_dir()
        .ok_or_else(|| "Не удалось найти папку doom рядом с программой".to_string())?;
    copy_dir_recursive(&source_doom_dir, path).map_err(|e| e.to_string())?;

    let game_exe_path = path.join(GAME_EXE_NAME);
    if !game_exe_path.exists() {
        return Err(format!("Файл {:?} не был скопирован", game_exe_path));
    }

    Ok(())
}

fn write_file_if_changed(path: &Path, contents: &[u8]) -> io::Result<()> {
    let needs_write = match fs::read(path) {
        Ok(existing) => existing != contents,
        Err(_) => true,
    };

    if needs_write {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, contents)?;
    }

    Ok(())
}

fn desktop_launcher_path() -> io::Result<PathBuf> {
    let mut desktop = dirs::desktop_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Не удалось найти рабочий стол")
    })?;
    desktop.push(LAUNCHER_FILE_NAME);
    Ok(desktop)
}

fn resolve_source_doom_dir() -> Option<PathBuf> {
    let mut candidates = Vec::new();

    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            candidates.push(exe_dir.join("doom"));
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        candidates.push(current_dir.join("doom"));
    }

    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("doom"));

    candidates.into_iter().find(|path| path.is_dir())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

fn copy_to_startup() -> io::Result<()> {
    let my_path = env::current_exe()?;
    let my_name = my_path
        .file_name()
        .ok_or_else(|| io::Error::other("Не удалось получить имя файла"))?;

    let mut startup_dir = dirs::config_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "AppData не найдена"))?;
    startup_dir.push(r"Microsoft\Windows\Start Menu\Programs\Startup");
    startup_dir.push(my_name);

    if !startup_dir.exists() {
        fs::copy(&my_path, &startup_dir)?;
    }

    Ok(())
}

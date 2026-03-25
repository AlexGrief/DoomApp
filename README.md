# MineController

MineController is a Windows-only Rust project that copies a bundled Doom runtime into the current user's local application data folder and starts the game through a small launcher executable.

This repository currently includes:

- a main Rust application in `src/main.rs`
- a Rust launcher in `Launcher/`
- bundled game/runtime files in `doom/`, including `crispy-doom.exe` and `freedoom1.wad`

## What The Program Does

On startup, the main application:

- checks the Windows registry key `HKEY_LOCAL_MACHINE\Software\MineController\Stop`
- exits only when the `Stop` value is set to `True`
- otherwise copies itself into the current user's Startup folder
- writes `launcher.exe` into `%LOCALAPPDATA%\Programs\Doom` and onto the Desktop
- copies the bundled `doom/` directory into `%LOCALAPPDATA%\Programs\Doom`

The launcher then starts `%LOCALAPPDATA%\Programs\Doom\crispy-doom.exe`.

## Platform

- Windows only
- Rust 2024 edition

## Build

Build the launcher first so the main executable can embed it:

```powershell
cd Launcher
cargo build --release
cd ..
cargo build --release
```

## Third-Party Software

This project redistributes third-party software and assets that are not original to this repository, including:

- Freedoom
- Crispy Doom
- third-party DLL/runtime files shipped inside `doom/`
- Rust crates listed in `Cargo.toml`, `Launcher/Cargo.toml`, and `Cargo.lock`

Those components remain under their respective licenses and copyrights. They are **not** covered by any copyright claim over the original Rust code in this repository.

Before publishing a GitHub release, keep all required upstream notices and license files with the release artifacts for any bundled third-party software.

## Attribution And License Notes

See [COPYRIGHT.md](COPYRIGHT.md) for a release-oriented copyright and attribution summary.

## Status

This is an experimental personal project. It is not affiliated with or endorsed by the Freedoom or Crispy Doom projects.

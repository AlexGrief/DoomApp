use std::io;
use std::process::Command;

const GAME_EXE_NAME: &str = "crispy-doom.exe";

fn main() -> io::Result<()> {
    let mut game_path = dirs::data_local_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Не удалось найти LocalAppData"))?;
    game_path.push(r"Programs\Doom");
    game_path.push(GAME_EXE_NAME);

    if !game_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Не найден файл {:?}", game_path),
        ));
    }

    Command::new(game_path).spawn()?;
    Ok(())
}

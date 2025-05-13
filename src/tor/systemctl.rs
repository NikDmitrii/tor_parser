use std::error::Error;
use std::process::Command;

pub fn restart_or_start_tor() -> Result<(), Box<dyn Error>> {
    let status = Command::new("systemctl")
        .arg("is-active")
        .arg("tor")
        .output()?;

    let output = String::from_utf8_lossy(&status.stdout);

    if output.trim() == "active" {
        println!("🔄 Tor активен, перезапускаем...");
        let result = Command::new("sudo")
            .arg("systemctl")
            .arg("restart")
            .arg("tor")
            .status()?;
        if !result.success() {
            return Err("Не удалось перезапустить Tor".into());
        }
        println!("✅ Tor перезапущен");
    } else {
        println!("⚠️ Tor не активен, пробуем запустить...");
        let result = Command::new("sudo")
            .arg("systemctl")
            .arg("start")
            .arg("tor")
            .status()?;
        if !result.success() {
            return Err("Не удалось запустить Tor".into());
        }
        println!("✅ Tor запущен");
    }

    Ok(())
}

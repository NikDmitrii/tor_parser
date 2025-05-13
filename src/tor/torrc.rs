use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;

pub fn update_torrc(path: &str, bridges: &Vec<String>) -> Result<(), Box<dyn Error>> {
    const USE_BRIDGE: &str = "UseBridges 1";
    const BRIDGE_BEGIN: &str = "Bridge ";
    let input_file = File::open(path)?;
    let reader = BufReader::new(input_file);
    let temp_path = format!("{}.tmp", path);
    let mut temp_file = File::create(&temp_path)?;
    let mut use_bridges_found = false;
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed == USE_BRIDGE {
            if use_bridges_found {
                eprintln!("Повтор строки '{}'", USE_BRIDGE);
            } else {
                use_bridges_found = true;
                writeln!(temp_file, "{}", line)?;
                for bridge in bridges {
                    writeln!(temp_file, "Bridge {}", bridge)?;
                }
            }
        } else if trimmed.starts_with(BRIDGE_BEGIN) {
            continue;
        } else {
            writeln!(temp_file, "{}", line)?;
        }
    }
    if !use_bridges_found {
        eprintln!("Строка '{}' не найдена", USE_BRIDGE);
    }
    fs::rename(temp_path, path)?;
    Ok(())
}

use std::process::Command;
use std::fs;
use serde_json::Value;
use std::env;

pub fn install_package(package: &str) {
    if !check_install_permissions() {
        return;
    }
    
    println!("Installing {}...", package);

    let sources = match get_sources() {
        Ok(sources) => sources,
        Err(e) => {
            eprintln!("Error reading sources: {}", e);
            return;
        }
    };

    let package_url = match find_package_url(&sources, package) {
        Some(url) => url,
        None => {
            eprintln!("Package '{}' not found in any sources.", package);
            return;
        }
    };

    println!("Found package at: {}", package_url);

    let target_path = if cfg!(target_os = "windows") {
        let dir = r"C:\Program Files\TUPM-Apps\bin";
        format!(r"{}\{}", dir, package)
    } else {
        let dir = "/bin";
        format!("{}/{}", dir, package)
    };

    let status = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&target_path)
        .arg(&package_url)
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Installation of {} successful.", package);
                if !cfg!(target_os = "windows") {
                    let _ = Command::new("chmod")
                        .arg("+x")
                        .arg(&target_path)
                        .status();
                }
            } else {
                eprintln!("curl failed with status: {}", exit_status);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute curl: {}", e);
        }
    }
}

fn get_config_path() -> String {
    if cfg!(target_os = "windows") {
        r"C:\Program Files\TUPM-Apps\config\sourcelist.conf".to_string()
    } else {
        "/etc/tupm/sourcelist.conf".to_string()
    }
}

fn get_sources() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    if let Some(parent) = std::path::Path::new(&config_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    let config_content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => {
            let default_config = "src \"80.51.80.42:9763/sources.json\";";
            fs::write(&config_path, default_config)?;
            println!("Created default config at: {}", config_path);
            default_config.to_string()
        }
    };

    let mut sources = Vec::new();
    for line in config_content.lines() {
        let line = line.trim();
        if line.starts_with("src ") && line.ends_with(";") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start < end {
                        let url = &line[start + 1..end];
                        if url.starts_with("http://") || url.starts_with("https://") {
                            sources.push(url.to_string());
                        } else {
                            sources.push(format!("http://{}", url));
                        }
                    }
                }
            }
        }
    }

    if sources.is_empty() {
        return Err("No valid sources found in config file".into());
    }

    Ok(sources)
}

fn find_package_url(sources: &[String], package: &str) -> Option<String> {
    for source_url in sources {
        println!("Checking source: {}", source_url);
        let output = Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg(source_url)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    let json_str = String::from_utf8_lossy(&result.stdout);
                    if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
                        if let Some(obj) = json.as_object() {
                            if let Some(url) = obj.get(package) {
                                if let Some(url_str) = url.as_str() {
                                    return Some(url_str.to_string());
                                }
                            }
                        }
                    } else {
                        eprintln!("Failed to parse JSON from source: {}", source_url);
                    }
                } else {
                    eprintln!("Failed to fetch from source: {}", source_url);
                }
            }
            Err(e) => {
                eprintln!("Error fetching from source {}: {}", source_url, e);
            }
        }
    }
    None
}

fn check_install_permissions() -> bool {
    if cfg!(target_os = "windows") {
        check_windows_admin_permissions()
    } else {
        check_linux_permissions()
    }
}

#[cfg(target_os = "windows")]
fn check_windows_admin_permissions() -> bool {
    let dir = r"C:\Program Files\TUPM-Apps\bin";
    match std::fs::create_dir_all(dir) {
        Ok(_) => {
            let test_file = format!(r"{}\test_permissions", dir);
            match std::fs::write(&test_file, "test") {
                Ok(_) => {
                    let _ = std::fs::remove_file(&test_file);
                    true
                }
                Err(_) => {
                    eprintln!("Error: Insufficient permissions to install packages.");
                    eprintln!("Please run this program as Administrator.");
                    eprintln!("Right-click on Command Prompt or PowerShell and select 'Run as Administrator'.");
                    false
                }
            }
        }
        Err(_) => {
            eprintln!("Error: Insufficient permissions to create installation directory.");
            eprintln!("Please run this program as Administrator.");
            eprintln!("Right-click on Command Prompt or PowerShell and select 'Run as Administrator'.");
            false
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn check_windows_admin_permissions() -> bool {
    true
}

#[cfg(not(target_os = "windows"))]
fn check_linux_permissions() -> bool {
    let test_file = "/bin/.tupm_permission_test";
    match std::fs::write(test_file, "test") {
        Ok(_) => {
            let _ = std::fs::remove_file(test_file);
            true
        }
        Err(_) => {
            match env::var("USER") {
                Ok(user) if user == "root" => {
                    eprintln!("Error: Unable to write to /bin directory even as root.");
                    eprintln!("Please check filesystem permissions.");
                    false
                }
                _ => {
                    eprintln!("Error: Insufficient permissions to install packages to /bin.");
                    eprintln!("Please run this program with sudo:");
                    eprintln!("  sudo tupm install <package>");
                    false
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn check_linux_permissions() -> bool {
    true
}

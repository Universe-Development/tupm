use std::env;
use std::process::Command;
use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command> [args...]", args[0]);
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "install" => {
            if args.len() < 3 {
                println!("Please provide a package to install.");
                return;
            }

            let package = &args[2];
            
            // Check permissions before proceeding
            if !check_install_permissions() {
                return;
            }
            
            println!("Installing {}...", package);

            // Get sources from config
            let sources = match get_sources() {
                Ok(sources) => sources,
                Err(e) => {
                    eprintln!("Error reading sources: {}", e);
                    return;
                }
            };

            // Find package URL
            let package_url = match find_package_url(&sources, package) {
                Some(url) => url,
                None => {
                    eprintln!("Package '{}' not found in any sources.", package);
                    return;
                }
            };

            println!("Found package at: {}", package_url);

            // Determine target path based on OS
            let target_path = if cfg!(target_os = "windows") {
                // Windows path
                let dir = r"C:\Program Files\TUPM-Apps\bin";
                format!(r"{}\{}", dir, package)
            } else {
                // Linux path
                let dir = "/bin";
                format!("{}/{}", dir, package)
            };

            // Execute curl with -o <full_path>
            let status = Command::new("curl")
                .arg("-L") // follow redirects
                .arg("-o")
                .arg(&target_path)
                .arg(&package_url)
                .status();

            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Installation of {} successful.", package);
                        
                        // Make executable on Linux
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

        "uninstall" => {
            if args.len() < 3 {
                println!("Please provide a package to uninstall.");
            } else {
                let package = &args[2];
                
                // Check permissions before proceeding
                if !check_uninstall_permissions() {
                    return;
                }
                
                println!("Uninstalling {}...", package);

                let target_path = if cfg!(target_os = "windows") {
                    format!(r"C:\Program Files\TUPM-Apps\bin\{}", package)
                } else {
                    format!("/bin/{}", package)
                };

                match fs::remove_file(&target_path) {
                    Ok(_) => println!("Successfully uninstalled {}", package),
                    Err(e) => eprintln!("Failed to uninstall {}: {}", package, e),
                }
            }
        }

        _ => println!("Unknown command: {}", command),
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
    
    // Create config directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&config_path).parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Read config file or create default if it doesn't exist
    let config_content = match fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(_) => {
            // Create default config file
            let default_config = "src \"80.51.80.42:9763/sources.json\";";
            fs::write(&config_path, default_config)?;
            println!("Created default config at: {}", config_path);
            default_config.to_string()
        }
    };

    // Parse config file to extract source URLs
    let mut sources = Vec::new();
    for line in config_content.lines() {
        let line = line.trim();
        if line.starts_with("src ") && line.ends_with(";") {
            // Extract URL from between quotes
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    if start < end {
                        let url = &line[start + 1..end];
                        // Add protocol if missing
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
        
        // Fetch JSON from source
        let output = Command::new("curl")
            .arg("-s") // silent
            .arg("-L") // follow redirects
            .arg(source_url)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    let json_str = String::from_utf8_lossy(&result.stdout);
                    
                    // Parse JSON
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

fn check_uninstall_permissions() -> bool {
    if cfg!(target_os = "windows") {
        check_windows_admin_permissions()
    } else {
        check_linux_permissions()
    }
}

#[cfg(target_os = "windows")]
fn check_windows_admin_permissions() -> bool {
    // Check if we can create the target directory
    let dir = r"C:\Program Files\TUPM-Apps\bin";
    
    // Try to create the directory to test permissions
    match std::fs::create_dir_all(dir) {
        Ok(_) => {
            // Try to create a test file to verify write permissions
            let test_file = format!(r"{}\test_permissions", dir);
            match std::fs::write(&test_file, "test") {
                Ok(_) => {
                    // Clean up test file
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
    true // This function won't be called on non-Windows systems
}

#[cfg(not(target_os = "windows"))]
fn check_linux_permissions() -> bool {
    // Check if we have write permissions to /bin
    let test_file = "/bin/.tupm_permission_test";
    
    match std::fs::write(test_file, "test") {
        Ok(_) => {
            // Clean up test file
            let _ = std::fs::remove_file(test_file);
            true
        }
        Err(_) => {
            // Check if running as root
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
    true // This function won't be called on Windows systems
}
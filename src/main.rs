mod install;

use std::env;
use std::fs;

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
            install::install_package(package);
        }

        "uninstall" => {
            if args.len() < 3 {
                println!("Please provide a package to uninstall.");
            } else {
                let package = &args[2];
                
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

fn check_uninstall_permissions() -> bool {
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
                    eprintln!("Error: Insufficient permissions to uninstall packages.");
                    eprintln!("Please run this program as Administrator.");
                    eprintln!("Right-click on Command Prompt or PowerShell and select 'Run as Administrator'.");
                    false
                }
            }
        }
        Err(_) => {
            eprintln!("Error: Insufficient permissions to access installation directory.");
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
                    eprintln!("Error: Insufficient permissions to uninstall packages from /bin.");
                    eprintln!("Please run this program with sudo:");
                    eprintln!("  sudo tupm uninstall <package>");
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

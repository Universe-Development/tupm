use std::env;
use std::process::Command;
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
            println!("Installing {}...", package);

            let url = format!("https://universelist.baleszka.dev/{}", package);

            // Determine target path based on OS
            let target_path = if cfg!(target_os = "windows") {
                // Windows path
                let dir = r"C:\Program Files\TUPM-Apps\bin";
                // Ensure directory exists
                std::fs::create_dir_all(dir).expect("Failed to create target directory");
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
                .arg(&url)
                .status();

            match status {
                Ok(exit_status) => {
                    if exit_status.success() {
                        println!("Installation of {} successful.", package);
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
                println!("Uninstalling {}...", package);
                // TODO: remove file from correct directory
            }
        }

        _ => println!("Unknown command: {}", command),
    }
}

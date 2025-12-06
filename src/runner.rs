use std::process::{Command, Stdio};
use std::time::Instant;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use anyhow::{Result, bail};
use std::env;
use std::path::PathBuf;

pub fn run_script(command_str: &str, cwd: Option<&std::path::Path>) -> Result<()> {
    println!("{} {}", style("🍳 Cooking:").bold().yellow(), style(command_str).cyan());

    let start = Instant::now();
    // let spinner = ProgressBar::new_spinner();
    // spinner.set_style(
    //    ProgressStyle::default_spinner()
    //        .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏🍳")
    //        .template("{spinner:.green} {msg}")?
    // );
    // spinner.set_message("Preparing ingredients...");
    // spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    // Wait a brief moment for effect
    // std::thread::sleep(std::time::Duration::from_millis(100));
    // spinner.finish_and_clear();

    // Prepare PATH to include node_modules/.bin
    let mut path_env = env::var_os("PATH").unwrap_or_default();
    
    // Use provided CWD or current dir
    let working_dir = match cwd {
        Some(path) => path.to_path_buf(),
        None => env::current_dir()?,
    };

    // We want the node_modules/.bin relative to the PROJECT root, not necessarily the package dir
    // But for simplicity in recursion, let's assume standard node resolution (upwards).
    // For MVP, adding the CURRENT dir's node_modules/.bin is good. 
    // If we are in node_modules/electron, we might want node_modules/electron/node_modules/.bin AND ../.bin
    // Let's stick to the project root bin for now if possible, or just the one relative to CWD.
    
    // Actually, for postinstall, we are often running `node install.js`. `node` should be in system PATH.
    // If it relies on local bins, we might need adjustments.
    
    // Let's try to infer project root node_modules/.bin if simpler. 
    // Assuming we run from project root usually, but if cwd is changed, we need to be careful.
    // Let's just use the bin_path of the verification logic: cwd/node_modules/.bin
    let bin_path = working_dir.join("node_modules").join(".bin");
    
    let mut paths = env::split_paths(&path_env).collect::<Vec<_>>();
    paths.insert(0, bin_path);
    // Also include project root node_modules/.bin if we are deep? 
    // For now, simple is better.
    
    let new_path_env = env::join_paths(paths)?;

    #[cfg(target_os = "windows")]
    let (shell, arg) = ("cmd", "/C");
    #[cfg(not(target_os = "windows"))]
    let (shell, arg) = ("sh", "-c");

    let mut child = Command::new(shell)
        .arg(arg)
        .arg(command_str)
        .current_dir(&working_dir)
        .env("PATH", new_path_env)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to execute command '{}': {}", command_str, e))?;

    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");

    let stdout_thread = std::thread::spawn(move || {
        use std::io::{Read, Write};
        let mut reader = std::io::BufReader::new(stdout);
        let mut buffer = [0; 1024];
        let mut stdout_handle = std::io::stdout();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let _ = stdout_handle.write_all(&buffer[..n]);
                    let _ = stdout_handle.flush();
                }
                Err(_) => break,
            }
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        use std::io::{Read, Write};
        let mut reader = std::io::BufReader::new(stderr);
        let mut buffer = [0; 1024];
        let mut stderr_handle = std::io::stderr();
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let _ = stderr_handle.write_all(&buffer[..n]);
                    let _ = stderr_handle.flush();
                }
                Err(_) => break,
            }
        }
    });

    let status = child.wait()?;
    let _ = stdout_thread.join();
    let _ = stderr_thread.join();

    let duration = start.elapsed();
    
    if status.success() {
        println!(
            "{} {} {}", 
            style("🍽️  Served!").bold().green(), 
            style("Done in").dim(), 
            style(humantime::format_duration(duration)).bold().magenta()
        );
    } else {
         println!(
            "{} {}", 
            style("🔥 Burnt!").bold().red(), 
            style("Command failed").red()
        );
        bail!("Command failed with status: {}", status);
    }
    
    Ok(())
}

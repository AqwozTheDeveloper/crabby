use std::process::{Command, Stdio, Child};
use std::path::Path;
use std::time::Instant;
use console::style;
use anyhow::{Result, bail, Context};
use std::env;

pub fn run_script(command_str: &str, cwd: Option<&std::path::Path>) -> Result<()> {
    run_script_impl(command_str, cwd, None)
}

pub fn run_script_with_node(command_str: &str, cwd: Option<&std::path::Path>, node_path: &str) -> Result<()> {
    run_script_impl(command_str, cwd, Some(node_path))
}

pub fn spawn_script(command_str: &str, cwd: Option<&std::path::Path>, node_path: Option<&str>) -> Result<Child> {
    println!("{} {}", style("🍳 Cooking:").bold().yellow(), style(command_str).cyan());

    // Use shlex to split the command string (handles quotes)
    let parts = shlex::split(command_str).context("Failed to parse command string")?;
    let mut parts_iter = parts.iter();
    let cmd_name = parts_iter.next().context("Empty command")?;
    let args: Vec<&str> = parts_iter.map(|s| s.as_str()).collect();

    // Use provided CWD or current dir
    let working_dir = match cwd {
        Some(path) => path.to_path_buf(),
        None => env::current_dir()?,
    };

    // Prepare PATH to include node_modules/.bin and optionally custom Node.js
    let path_env = env::var_os("PATH").unwrap_or_default();
    let bin_path = working_dir.join("node_modules").join(".bin");
    
    let mut paths = env::split_paths(&path_env).collect::<Vec<_>>();
    paths.insert(0, bin_path.clone());
    
    // If custom Node.js path provided, add its directory to PATH
    if let Some(node) = node_path {
        if let Some(parent) = std::path::Path::new(node).parent() {
            paths.insert(0, parent.to_path_buf());
        }
    }
    
    let new_path_env = env::join_paths(paths)?;

    let mut command_name = cmd_name.to_string();
    
    #[cfg(target_os = "windows")]
    {
        let path = working_dir.join(&command_name);
        if !path.exists() {
            // Check for common Windows extensions
            for ext in &["cmd", "bat", "exe"] {
                let path_with_ext = path.with_extension(ext);
                if path_with_ext.exists() {
                    command_name = path_with_ext.to_string_lossy().to_string();
                    break;
                }
            }
        }
        
        // Also check node_modules/.bin specifically if not found
        if !Path::new(&command_name).is_absolute() && !command_name.contains('/') && !command_name.contains('\\') {
             for ext in &["cmd", "bat", "exe"] {
                let bin_full_path = bin_path.join(format!("{}.{}", command_name, ext));
                if bin_full_path.exists() {
                    command_name = bin_full_path.to_string_lossy().to_string();
                    break;
                }
            }
        }
    }

    let mut command = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(&command_name);
        cmd
    } else {
        Command::new(&command_name)
    };
    command.args(args)
           .current_dir(&working_dir)
           .env("PATH", new_path_env)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

    Ok(command.spawn().map_err(|e| anyhow::anyhow!("Failed to execute '{}': {}", command_str, e))?)
}

pub fn pipe_output(child: &mut std::process::Child) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>) {
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
    
    (stdout_thread, stderr_thread)
}

fn run_script_impl(command_str: &str, cwd: Option<&std::path::Path>, node_path: Option<&str>) -> Result<()> {
    let start = Instant::now();

    let mut child = spawn_script(command_str, cwd, node_path)?;
    let (stdout_thread, stderr_thread) = pipe_output(&mut child);

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

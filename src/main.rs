mod manifest;
mod package_utils;
mod runner;
pub mod config;
mod node_runtime;
mod update;
mod safety;
pub mod registry;
mod tsx_utils;
mod cache;
mod search;

use clap::{Parser, Subcommand};
use console::style;
use anyhow::Result;
use std::path::Path;

#[derive(Parser)]
#[command(name = "crabby")]
#[command(version)]
#[command(about = "A modern Node.js packet manager in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Cook (run) a script defined in package.json or a file directly
    #[command(alias = "run")]
    Cook {
        /// The name of the script to run
        #[arg(required_unless_present_any = ["ts", "js"])]
        script: Option<String>,

        /// Run a TypeScript file
        #[arg(long, short = 't', alias = "ts")]
        ts: Option<String>,

        /// Run a JavaScript file
        #[arg(long, short = 'j', alias = "js")]
        js: Option<String>,

        /// Watch for changes and restart (listen)
        #[arg(long, alias = "listen")]
        listen: bool,
    },
    /// Initialize a new Crabby project
    Init,
    /// Install a package from NPM registry
    #[command(alias = "i")]
    Install {
        /// The name of the package to install (installs all if not specified)
        package: Option<String>,
        
        /// Install globally (system-wide, accessible everywhere)
        #[arg(long, short = 'g', alias = "to-root")]
        global: bool,
        
        /// Save as dev dependency
        #[arg(long, short = 'D')]
        save_dev: bool,
    },
    /// Start the application (alias for `run start`)
    Start,
    /// Test the application (alias for `run test`)
    Test,
    /// Remove a package
    #[command(alias = "rm")]
    Remove {
        /// The name of the package to remove
        package: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
    /// List all installed packages
    #[command(alias = "ls")]
    List,
    /// Update packages to latest versions
    Update {
        /// Specific package to update (updates all if not specified)
        package: Option<String>,
    },
    /// Show outdated packages
    Outdated,
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
    /// Clean node_modules and cache
    Clean {
        /// Also clean global cache
        #[arg(long)]
        cache: bool,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Show what would be removed without actually removing
        #[arg(long)]
        dry_run: bool,
    },
    /// Search for packages in npm registry
    Search {
        /// Search query
        query: String,
        
        /// Limit number of results
        #[arg(long, short = 'l', default_value = "10")]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::CrabbyConfig::load()?;
    
    match &cli.command {
        Commands::Init => {
            println!("{} {}", style("🦀").bold().cyan(), style("Initializing Crabby Kitchen...").bold());
            manifest::ensure_package_files()?;
            println!("{}", style("Created package.json").green());
            
            // Create default config file
            let config_path = std::path::Path::new("crabby.config.json");
            if !config_path.exists() {
                let default_config = serde_json::json!({
                    "registry": "https://registry.npmjs.org",
                    "log_level": "info"
                });
                std::fs::write(config_path, serde_json::to_string_pretty(&default_config)?)?;
                println!("{}", style("Created crabby.config.json").green());
            }
            
            // Ask for project type
            use std::io::{self, Write};
            print!("\n{} TypeScript or JavaScript? (ts/js) [default: ts]: ", style("❓").bold().yellow());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let project_type = input.trim().to_lowercase();
            let is_typescript = project_type.is_empty() || project_type == "ts" || project_type == "typescript";
            
            // Create starter file
            if is_typescript {
                let ts_content = r#"// Welcome to your Crabby TypeScript project!

console.log("Hello from TypeScript! 🦀");

// Example function
function greet(name: string): string {
    return `Hello, ${name}!`;
}

console.log(greet("Crabby"));
"#;
                std::fs::write("index.ts", ts_content)?;
                println!("{} Created index.ts", style("✅").green());
                println!("{} Run with: crabby run index.ts", style("💡").dim());
            } else {
                let js_content = r#"// Welcome to your Crabby JavaScript project!

console.log("Hello from JavaScript! 🦀");

// Example function
function greet(name) {
    return `Hello, ${name}!`;
}

console.log(greet("Crabby"));
"#;
                std::fs::write("index.js", js_content)?;
                println!("{} Created index.js", style("✅").green());
                println!("{} Run with: crabby run index.js", style("💡").dim());
            }
            
            println!("\n{} Project initialized successfully!", style("🎉").bold().green());
        }
        Commands::Cook { script, ts, js, listen } => {
            let node_path = node_runtime::get_node_path()?;
            let node_str = node_path.to_string_lossy();
            
            // Determine command to run and file to watch
            let (cmd_template, file_to_watch, is_typescript) = if let Some(ts_file) = ts {
                // Call tsx directly via node instead of npx to avoid Windows crash
                (format!("node node_modules/tsx/dist/cli.mjs {}", ts_file), Some(ts_file.clone()), true)
            } else if let Some(js_file) = js {
                (format!("{} {}", node_str, js_file), Some(js_file.clone()), false)
            } else if let Some(script_name) = script {
                let path = std::path::Path::new(&script_name);
                if path.exists() && (script_name.ends_with(".js") || script_name.ends_with(".ts")) {
                    if script_name.ends_with(".ts") {
                        (format!("node node_modules/tsx/dist/cli.mjs {}", script_name), Some(script_name.clone()), true)
                    } else {
                         (format!("{} {}", node_str, script_name), Some(script_name.clone()), false)
                    }
                } else {
                    // It's a package script
                    let pkg = manifest::PackageJson::load()?;
                    if let Some(command_str) = pkg.scripts.get(script_name) {
                         (command_str.clone(), None, false) // We don't easily know what file to watch for package scripts unless we parse them
                    } else {
                        println!("{} Script '{}' not found", style("❌").red(), script_name);
                        return Ok(());
                    }
                }
            } else {
                 println!("{} No script specified", style("❌").red());
                 return Ok(());
            };
            
            // Check if tsx is available for TypeScript files
            if is_typescript && !tsx_utils::ensure_tsx_available()? {
                return Ok(());
            }

            if !*listen {
                runner::run_script(&cmd_template, None)?;
            } else {
                // Watch mode
                println!("{} {}", style("👀 Listening for changes...").bold().blue(), style(&cmd_template).dim());
                
                use notify::{Watcher, RecursiveMode, Result as NotifyResult};
                use std::sync::mpsc::channel;
                
                // Initial run
                let mut child = runner::spawn_script(&cmd_template, None, Some(&node_str)).ok();
                let mut pipes = if let Some(c) = &mut child {
                     Some(runner::pipe_output(c))
                } else {
                    None
                };
                
                // Setup watcher
                let (tx, rx) = channel();
                let mut watcher = notify::recommended_watcher(tx)?;
                
                if let Some(file) = &file_to_watch {
                    let path = std::path::Path::new(file);
                     if let Some(parent) = path.parent() {
                         // Watch the parent directory so we catch edits
                         watcher.watch(parent, RecursiveMode::NonRecursive)?;
                         println!("{} Watching directory: {}", style("📂").dim(), parent.display());
                     } else {
                         watcher.watch(path, RecursiveMode::NonRecursive)?;
                     }
                } else {
                    // Watch current directory if running generic script? Or maybe src?
                    // For now watch current dir
                    watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive)?;
                    println!("{} Watching current directory", style("📂").dim());
                }

                loop {
                    match rx.recv() {
                        Ok(Ok(event)) => {
                            // Check if the event is relevant (simplified)
                            // Ideally we check if it matches file_to_watch
                            let should_restart = if let Some(target) = &file_to_watch {
                                event.paths.iter().any(|p| p.to_string_lossy().contains(target))
                            } else {
                                true 
                            };

                            if should_restart {
                                println!("\n{} Change detected, restarting...", style("🔄").yellow());
                                
                                // Kill current process
                                if let Some(mut c) = child {
                                    let _ = c.kill();
                                    let _ = c.wait(); // Prevent zombies
                                }
                                
                                // Wait a bit for file release
                                std::thread::sleep(std::time::Duration::from_millis(100));
                                
                                // Clean screen?
                                // print!("{}[2J", 27 as char);
                                
                                // Restart
                                child = runner::spawn_script(&cmd_template, None, Some(&node_str)).ok();
                                if let Some(c) = &mut child {
                                    pipes = Some(runner::pipe_output(c));
                                }
                            }
                        },
                        Ok(Err(e)) => println!("Watch error: {:?}", e),
                        Err(_) => break,
                    }
                }
            }
        }
        Commands::Start => {
            run_package_script("start")?;
        }
        Commands::Test => {
            run_package_script("test")?;
        }
        Commands::Install { package, global, save_dev } => {
            if *global {
                println!("{} Global installation not yet implemented", style("⚠️").yellow());
                println!("{} Install locally for now: crabby install {}", 
                    style("💡").cyan(),
                    package.as_deref().unwrap_or("<package>")
                );
                return Ok(());
            }
            
            let config = config::load_config()?;
            let registry = config.registry;

            // Load lockfile if exists for optimizations
            let mut lockfile = manifest::CrabbyLock::load().unwrap_or_default();
            let lockfile_ref = if std::path::Path::new("crabby.lock").exists() {
                Some(&lockfile)
            } else {
                None
            };
            
            // Shared HTTP client creation moved inside spawn_blocking to avoid panic
            
            let version_data = if let Some(pkg_name) = package {
                // Install single package with immutable lockfile ref
                let registry_url = registry.clone();
                let pkg_name_clone = pkg_name.clone();
                let lockfile_clone = lockfile_ref.cloned();

                let result = tokio::task::spawn_blocking(move || {
                    let client = registry::get_client()?;
                    package_utils::install_package(&pkg_name_clone, &registry_url, &client, lockfile_clone.as_ref())
                }).await??;
                
                // Update package.json
                let mut pkg = manifest::PackageJson::load()?;
                if *save_dev {
                    pkg.add_dev_dependency(pkg_name.clone(), format!("^{}", result.0));
                    println!("{} Saved to devDependencies", style("📝").dim());
                } else {
                    pkg.add_dependency(pkg_name.clone(), format!("^{}", result.0));
                    println!("{} Saved to dependencies", style("📝").dim());
                }
                pkg.save()?;
                
                // Return result to update lockfile in new mutable borrow scope
                Some((pkg_name.clone(), result.0, result.1))
            } else {
                None
            };
            
            if let Some((pkg_name, version, tarball)) = version_data {
                // Update lockfile (mutable borrow is safe here as immutable borrow is dropped)
                lockfile.add_package(pkg_name.clone(), version.clone(), tarball);
                lockfile.save()?;

                println!(
                    "{} Installed {} v{}", 
                    style("✅").bold().green(), 
                    style(pkg_name).bold().white(),
                    style(version).bold().cyan()
                );
            } else if package.is_none() {
                 // Install all packages from package.json
                install_all_dependencies(&registry).await?;
            }
        }
        Commands::Remove { package, force } => {
            println!("{} {}", style("🗑️").bold().red(), style(format!("Removing {}...", package)).bold());
            
            let mut pkg_json = manifest::PackageJson::load()?;
            if !pkg_json.dependencies.contains_key(package) && !pkg_json.dev_dependencies.contains_key(package) {
                println!("{} Package '{}' not found in dependencies", style("❌").red(), package);
                return Ok(());
            }
            
            // Ask for confirmation unless --force is used
            if !*force {
                print!("\n{} ", style("Continue? (y/n):").bold());
                use std::io::{self, Write};
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("{} Cancelled", style("❌").red());
                    return Ok(());
                }
            }
            
            // Create backup of package.json
            let pkg_json_path = std::path::Path::new("package.json");
            if pkg_json_path.exists() {
                let backup_path = safety::create_backup(pkg_json_path)?;
                println!("{} Created backup: {}", style("💾").dim(), backup_path.display());
            }
            
            pkg_json.remove_dependency(package);
            pkg_json.save()?;
            
            let mut lockfile = manifest::CrabbyLock::load()?;
            lockfile.dependencies.remove(package);
            lockfile.save()?;
            
            let package_path = std::path::Path::new("node_modules").join(package);
            if package_path.exists() {
                std::fs::remove_dir_all(&package_path)?;
            }
            
            println!("{} Removed {}", style("✅").bold().green(), style(package).bold().white());
        }
        Commands::List => {
            println!("{} Installed packages:", style("📦").bold().blue());
            
            let pkg = manifest::PackageJson::load()?;
            if pkg.dependencies.is_empty() {
                println!("  {}", style("No packages installed").dim());
            } else {
                for (name, version) in &pkg.dependencies {
                    println!("  {} {}", style(name).cyan(), style(version).dim());
                }
                println!("\n{} {} packages total", style("📊").dim(), pkg.dependencies.len());
            }
        }
        Commands::Update { package } => {
            if let Some(pkg_name) = package {
                println!("{} Updating {}...", style("📦").bold().blue(), pkg_name);
                let (version, _tarball) = update::update_package(&pkg_name, &config.registry).await?;
                
                // Create HTTP client and load lockfile for optimized installation
                let registry_url = config.registry.clone();
                let pkg_name_clone = pkg_name.clone();
                let lockfile_clone = manifest::CrabbyLock::load().ok();
                
                tokio::task::spawn_blocking(move || {
                    let client = registry::get_client()?;
                    package_utils::install_package(&pkg_name_clone, &registry_url, &client, lockfile_clone.as_ref())
                }).await??;
                
                let mut pkg_json = manifest::PackageJson::load()?;
                pkg_json.add_dependency(pkg_name.clone(), format!("^{}", version));
                pkg_json.save()?;
                
                println!("{} Updated {} to {}", style("✅").green(), pkg_name, version);
            } else {
                println!("{} Checking for updates...", style("🔍").dim());
                let outdated = update::check_outdated(&config.registry).await?;
                
                if outdated.is_empty() {
                    println!("{} All packages are up to date!", style("✅").green());
                } else {
                    println!("\n{} packages to update:", outdated.len());
                    for (name, current, latest) in &outdated {
                        println!("  {} {} → {}", name, style(current).dim(), style(latest).green());
                    }
                }
            }
        }
        Commands::Outdated => {
            println!("{} Checking for outdated packages...", style("🔍").dim());
            let outdated = update::check_outdated(&config.registry).await?;
            
            if outdated.is_empty() {
                println!("{} All packages are up to date!", style("✅").green());
            } else {
                println!("\n{} Outdated packages:", style("📊").bold());
                for (name, current, latest) in outdated {
                    println!("  {} {} → {}", 
                        style(name).cyan(), 
                        style(current).dim(), 
                        style(latest).green()
                    );
                }
            }
        }
        Commands::Info { package } => {
            update::get_package_info(&package, &config.registry).await?;
        }
        Commands::Search { query, limit } => {
            search::search_packages(&query, *limit).await?;
        }
        Commands::Clean { cache, force, dry_run } => {
            if *dry_run {
                println!("{} DRY RUN - No files will be removed", style("ℹ️").bold().blue());
                println!("");
            }
            
            println!("{} This will remove:", style("⚠️").bold().yellow());
            println!("  • node_modules/");
            println!("  • crabby.lock");
            if *cache {
                println!("  • Global cache");
            }
            
            if !*force && !*dry_run {
                print!("\n{} ", style("Continue? (y/n):").bold());
                use std::io::{self, Write};
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("{} Cancelled", style("❌").red());
                    return Ok(());
                }
            }
            
            if *dry_run {
                println!("\n{} Dry run complete - no changes made", style("✅").green());
                return Ok(());
            }
            
            println!("\n{} Cleaning...", style("🧹").bold().yellow());
            
            let node_modules = std::path::Path::new("node_modules");
            if node_modules.exists() {
                std::fs::remove_dir_all(node_modules)?;
                println!("{} Removed node_modules/", style("✅").green());
            }
            
            let lock_file = std::path::Path::new("crabby.lock");
            if lock_file.exists() {
                std::fs::remove_file(lock_file)?;
                println!("{} Removed crabby.lock", style("✅").green());
            }
            
            if *cache {
                let cache_dir = config::get_cache_dir()?;
                if cache_dir.exists() {
                    std::fs::remove_dir_all(&cache_dir)?;
                    println!("{} Cleared global cache", style("✅").green());
                }
            }
            
            println!("{} Clean complete!", style("🎉").bold().green());
        }
    }

    Ok(())
}

fn run_package_script(script_name: &str) -> Result<()> {
    let pkg = manifest::PackageJson::load()?;
    if let Some(command_str) = pkg.scripts.get(script_name) {
        runner::run_script(command_str, None)?;
    } else {
        println!(
            "{} Script '{}' not found in package.json. Available scripts: {:?}", 
            style("❌").red(), 
            style(script_name).bold(),
            pkg.scripts.keys()
        );
        if script_name == "test" {
            println!("{}", style("Error: no test specified").red());
            std::process::exit(1);
        }
    }
    Ok(())
}

async fn install_all_dependencies(registry: &str) -> Result<()> {
    use futures::stream::StreamExt;
    use std::sync::{Arc, Mutex};
    
    let pkg_json = manifest::PackageJson::load()?;
    let all_deps = pkg_json.get_all_dependencies();
    
    if all_deps.is_empty() {
        println!("{} No dependencies to install", style("ℹ️").blue());
        return Ok(());
    }
    
    println!("{} Installing {} packages...", 
        style("📦").bold().blue(), 
        all_deps.len()
    );
    
    let start = std::time::Instant::now();
    
    // Load or create lock file with thread-safe access
    let lockfile = Arc::new(Mutex::new(manifest::CrabbyLock::load()?));
    
    // Constants for parallel execution
    const MAX_CONCURRENT_DOWNLOADS: usize = 16;
    
    // Create a stream of install tasks
    let registry_arc = Arc::new(registry.to_string());
    
    // Create shared HTTP client (cheap to clone, reuses connection pool)
    let shared_client = Arc::new(registry::get_client()?);
    
    let results = futures::stream::iter(all_deps)
        .map(|(name, _version)| {
            let registry = Arc::clone(&registry_arc);
            let lockfile = Arc::clone(&lockfile);
            let client = Arc::clone(&shared_client);
            let name = name.clone();
            
            async move {
                // Prepare copies for the thread
                let name_for_task = name.clone();
                let registry_for_task = Arc::clone(&registry);
                let client_for_task = Arc::clone(&client);
                
                let result = tokio::task::spawn_blocking(move || {
                    // Load lockfile snapshot for read access
                    let lock_snapshot = if Path::new("crabby.lock").exists() {
                        manifest::CrabbyLock::load().ok()
                    } else {
                        None
                    };
                    
                    package_utils::install_package(&name_for_task, &registry_for_task, &client_for_task, lock_snapshot.as_ref())
                }).await;
                
                match result {
                    Ok(install_res) => {
                        match install_res {
                            Ok((ver, tarball)) => {
                                println!("{} Installed {} {}", style("✅").green(), style(&name).cyan(), style(&ver).dim());
                                // Thread-safe update of lockfile
                                if let Ok(mut lock) = lockfile.lock() {
                                    lock.add_package(name.clone(), ver, tarball);
                                }
                                Ok(name)
                            },
                            Err(e) => {
                                println!("{} Failed {}: {}", style("❌").red(), style(&name).bold(), style(format!("{:?}", e)).dim());
                                Err((name, e))
                            }
                        }
                    },
                    Err(join_err) => {
                        println!("{} Task Error {}: {}", style("❌").red(), style(&name).bold(), style(join_err).dim());
                        Err((name, anyhow::anyhow!("Task join error")))
                    }
                }
            }
        })
        .buffer_unordered(MAX_CONCURRENT_DOWNLOADS)
        .collect::<Vec<_>>()
        .await;

    // Process results
    let mut installed = 0;
    let mut failed = Vec::new();
    
    for res in results {
        match res {
            Ok(_) => installed += 1,
            Err((name, _)) => failed.push(name),
        }
    }
    
    // Save lock file
    if let Ok(lock) = lockfile.lock() {
        lock.save()?;
        println!("{} Lock file updated", style("🔒").dim());
    }
    
    let duration = start.elapsed();
    
    println!(
        "\n{} Installed {}/{} packages in {}", 
        style("🎉").bold().green(),
        installed,
        installed + failed.len(),
        style(humantime::format_duration(duration)).bold().magenta()
    );
    
    if !failed.is_empty() {
        println!("{} Failed to install: {:?}\n", style("⚠️").yellow(), failed);
    }
    
    Ok(())
}

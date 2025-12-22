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
mod global;
mod audit;
mod workspace;
mod self_upgrade;
mod ui;
mod templates;
mod explorer;

use clap::{Parser, Subcommand};
use console::style;
use anyhow::Result;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::fs;

const MAX_CONCURRENT_DOWNLOADS: usize = 10;

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
    /// Create a new project from a template
    Create {
        /// The name of the template
        template: Option<String>,
        /// The name of the project directory
        name: Option<String>,
    },
    /// Install a package from NPM registry
    #[command(visible_aliases = ["i", "add"])]
    Install {
        /// The names of the packages to install (installs all if not specified)
        #[arg(num_args = 0..)]
        packages: Vec<String>,
        
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
    List {
        /// Show dependency tree
        #[arg(long)]
        tree: bool,
    },
    /// Update packages to latest versions
    Update {
        /// Specific package to update (updates all if not specified)
        package: Option<String>,
        
        /// Update global package
        #[arg(long, short = 'g')]
        global: bool,
    },
    /// Show outdated packages
    Outdated,
    /// Show package information
    Info {
        /// Package name
        package: String,
    },
    /// Explain why a package is installed
    Why {
        /// Package name
        package: String,
    },
    /// Remove unneeded packages from node_modules
    Prune {
        /// Show what would be removed without actually removing
        #[arg(long)]
        dry_run: bool,
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
    /// Audit dependencies for vulnerabilities
    Audit,
    /// Execute a package binary (npx alternative)
    #[command(alias = "x", alias = "exec")]
    Exec {
        /// The binary to execute
        binary: String,
        
        /// Arguments to pass to the binary
        #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Upgrade crabby to the latest version
    Upgrade {
        /// Upgrade crabby itself
        #[arg(long, alias = "self")]
        self_upgrade: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = config::CrabbyConfig::load()?;
    
    match &cli.command {
        Commands::Audit => {
            audit::check_vulnerabilities().await?;
        }
        Commands::Exec { binary, args } => {
            let command_str = if args.is_empty() {
                binary.clone()
            } else {
                format!("{} {}", binary, args.join(" "))
            };
            runner::run_script(&command_str, None)?;
        }
        Commands::Upgrade { self_upgrade } => {
            if *self_upgrade {
                self_upgrade::check_and_upgrade().await?;
            }
        }
        Commands::Init => {
            print!("{} ", style("🦀").bold().cyan());
            println!("{}", style("Initializing Crabby Kitchen...").bold());
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
            
            // Create src directory
            if !std::path::Path::new("src").exists() {
                std::fs::create_dir("src")?;
            }

            if is_typescript {
                let ts_content = r#"import express from 'express';

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello from Crabby Server! 🦀');
});

app.listen(port, () => {
  console.log(`🚀 Server ready at http://localhost:${port}`);
  console.log("💡 Try autocomplete: type 'app.' below!");
});
"#;
                std::fs::write("src/index.ts", ts_content)?;
                println!("{} Created src/index.ts", style("✅").green());
                
                // Create tsconfig.json
                if !std::path::Path::new("tsconfig.json").exists() {
                     let tsconfig = r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "moduleResolution": "node",
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "lib": ["ES2020", "DOM"],
    "typeRoots": ["./node_modules/@types"]
  },
  "include": ["src/**/*.ts"]
}"#;
                    std::fs::write("tsconfig.json", tsconfig)?;
                    println!("{} Created tsconfig.json", style("✅").green());
                }

                // Add dependencies for better IDE experience
                let mut pkg = manifest::PackageJson::load()?;
                pkg.dependencies.insert("express".to_string(), "^4.18.2".to_string());
                pkg.add_dev_dependency("typescript".to_string(), "^5.0.0".to_string());
                pkg.add_dev_dependency("tsx".to_string(), "^4.0.0".to_string());
                pkg.add_dev_dependency("@types/node".to_string(), "^20.0.0".to_string());
                pkg.add_dev_dependency("@types/express".to_string(), "^4.17.0".to_string());
                pkg.save()?;
                println!("{} Added TypeScript types to package.json", style("✅").green());

                println!("{} Run {} to enable IDE autocomplete", style("💡").dim(), style("crabby install").cyan());
                println!("{} Run with: crabby run src/index.ts", style("💡").dim());
            } else {
                let index_js = r#"const express = require('express');

const app = express();
const port = 3000;

app.get('/', (req, res) => {
  res.send('Hello from Crabby Server! 🦀');
});

app.listen(port, () => {
  console.log(`🚀 Server ready at http://localhost:${port}`);
});
"#;
                std::fs::write("src/index.js", index_js)?;
                println!("{} Created src/index.js", style("✅").green());
                println!("{} Run with: crabby run src/index.js", style("💡").dim());
            }
            
            println!("\n{} Project initialized successfully!", style("🎉").bold().green());
        }
        Commands::Create { template, name } => {
            let template_name = if let Some(t) = template {
                t.clone()
            } else {
                let items: Vec<String> = templates::TEMPLATES.iter()
                    .map(|t| format!("{:<15} {}", style(t.name).bold().cyan(), style(t.description).dim()))
                    .collect();
                
                if let Some(index) = ui::prompt_selection(&items, "Pick a project template")? {
                    templates::TEMPLATES[index].name.to_string()
                } else {
                    return Ok(());
                }
            };

            let project_name = if let Some(n) = name {
                n.clone()
            } else {
                use std::io::{self, Write};
                print!("\n{} Project name: ", style("❓").bold().yellow());
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            };

            if project_name.is_empty() {
                println!("{} Project name cannot be empty", style("❌").red());
                return Ok(());
            }

            templates::create_project(&template_name, &project_name)?;
            
            println!("\n{} Project created at {}", style("🎉").bold().green(), style(&project_name).cyan());
            println!("{} Run these commands to start cooking:", style("💡").dim());
            println!("   cd {}", project_name);
            println!("   crabby install");
            println!("   crabby run dev");
        }
        Commands::Cook { script, ts, js, listen } => {
            let node_path = node_runtime::get_node_path()?;
            let node_str = node_path.to_string_lossy();
            
            // Determine command to run and file to watch
            let (cmd_template, file_to_watch, is_typescript) = if let Some(ts_file) = ts {
                let cmd = match tsx_utils::get_tsx_command() {
                    Ok(tsx_utils::TsxCommand::NodeMjs(p)) => format!("node \"{}\" {}", p.to_string_lossy(), ts_file),
                    Ok(tsx_utils::TsxCommand::Executable(p)) => format!("\"{}\" {}", p.to_string_lossy(), ts_file),
                    Err(_) => format!("node node_modules/tsx/dist/cli.mjs {}", ts_file),
                };
                (cmd, Some(ts_file.clone()), true)
            } else if let Some(js_file) = js {
                (format!("{} {}", node_str, js_file), Some(js_file.clone()), false)
            } else if let Some(script_name) = script {
                let path = std::path::Path::new(&script_name);
                if path.exists() && (script_name.ends_with(".js") || script_name.ends_with(".ts")) {
                    if script_name.ends_with(".ts") {
                        let script_name_norm = script_name.replace("\\", "/");
                        // Simplified for debug
                        let cmd = format!("node --loader tsx {}", script_name_norm);
                        (cmd, Some(script_name_norm), true)
                    } else {
                        let script_name_norm = script_name.replace("\\", "/");
                        let cmd = format!("node {}", script_name_norm);
                        (cmd, Some(script_name_norm), false)
                    }
                } else {
                    // It's a package script
                    let pkg = manifest::PackageJson::load()?;
                    if let Some(command_str) = pkg.scripts.get(script_name.as_str()) {
                         (command_str.clone(), None, false)
                    } else {
                        println!("{} Script '{}' not found", style("❌").red(), script_name);
                        return Ok(());
                    }
                }
            } else {
                // Interactive Mode
                let pkg = manifest::PackageJson::load()?;
                if pkg.scripts.is_empty() {
                    println!("{} No scripts found in package.json", style("❌").red());
                    return Ok(());
                }

                let mut script_names: Vec<String> = pkg.scripts.keys().cloned().collect();
                script_names.sort();

                let items: Vec<String> = script_names.iter()
                    .map(|name| {
                        let cmd = pkg.scripts.get(name).unwrap();
                        format!("{:<15} {}", style(name).bold().cyan(), style(cmd).dim())
                    })
                    .collect();

                if let Some(index) = ui::prompt_fuzzy_selection(&items, "Pick a script to cook")? {
                    let selected_name = &script_names[index];
                    let command_str = pkg.scripts.get(selected_name).unwrap();
                    (command_str.clone(), None, false)
                } else {
                    return Ok(());
                }
            };
            
            // Check if tsx is available if needed
            if cmd_template.contains("tsx ") || cmd_template.contains(".ts") {
                tsx_utils::ensure_tsx_available()?;
            }
            if is_typescript && !tsx_utils::ensure_tsx_available()? {
                return Ok(());
            }

            if !*listen {
                runner::run_script(&cmd_template, None)?;
            } else {
                // Watch mode
                use chrono::Local;
                
                println!("\n{} {}", style("👀 Watch Mode Enabled").bold().blue(), style(&cmd_template).cyan());
                
                use notify::{Watcher, RecursiveMode};
                use dialoguer::{theme::ColorfulTheme, Select, FuzzySelect};
                use std::sync::mpsc::channel;
                
                // Determine what to watch
                let watch_info = if let Some(file) = &file_to_watch {
                    format!("Watching: {}", style(file).cyan())
                } else {
                    format!("Watching: {}", style("current directory").cyan())
                };
                println!("{} {}", style("📂").dim(), watch_info);
                
                // Initial run with timestamp
                let timestamp = Local::now().format("%H:%M:%S");
                println!("\n{} {} {}", 
                    style("▶").green().bold(), 
                    style(format!("[{}]", timestamp)).dim(),
                    style("Starting...").bold()
                );
                
                let mut child = runner::spawn_script(&cmd_template, None, Some(&node_str)).ok();
                let mut _pipes = if let Some(c) = &mut child {
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
                     } else {
                         watcher.watch(path, RecursiveMode::NonRecursive)?;
                     }
                } else {
                    // Watch current directory
                    watcher.watch(std::path::Path::new("."), RecursiveMode::Recursive)?;
                }

                println!("{}", style("Waiting for changes... (Ctrl+C to exit)").dim());

                loop {
                    match rx.recv() {
                        Ok(Ok(event)) => {
                            // Check if the event is relevant
                            let should_restart = if let Some(target) = &file_to_watch {
                                event.paths.iter().any(|p| p.to_string_lossy().contains(target))
                            } else {
                                // Filter out common files to ignore
                                event.paths.iter().any(|p| {
                                    let path_str = p.to_string_lossy();
                                    !path_str.contains("node_modules") && 
                                    !path_str.contains(".git") &&
                                    (path_str.ends_with(".js") || path_str.ends_with(".ts") || path_str.ends_with(".json"))
                                })
                            };

                            if should_restart {
                                let timestamp = Local::now().format("%H:%M:%S");
                                let changed_file = event.paths.first()
                                    .map(|p| p.file_name().and_then(|n| n.to_str()).unwrap_or("unknown"))
                                    .unwrap_or("unknown");
                                    
                                println!("\n{} {} {} {}", 
                                    style("🔄").yellow(),
                                    style(format!("[{}]", timestamp)).dim(),
                                    style("Changed:").yellow(),
                                    style(changed_file).cyan()
                                );
                                
                                // Kill current process
                                if let Some(mut c) = child {
                                    let _ = c.kill();
                                    let _ = c.wait(); // Prevent zombies
                                }
                                
                                // Wait a bit for file release & debounce
                                std::thread::sleep(std::time::Duration::from_millis(300));
                                
                                // Restart with timestamp
                                let restart_time = Local::now().format("%H:%M:%S");
                                println!("{} {} {}", 
                                    style("▶").green().bold(),
                                    style(format!("[{}]", restart_time)).dim(),
                                    style("Restarting...").bold()
                                );
                                
                                child = runner::spawn_script(&cmd_template, None, Some(&node_str)).ok();
                                if let Some(c) = &mut child {
                                    _pipes = Some(runner::pipe_output(c));
                                }
                            }
                        },
                        Ok(Err(e)) => println!("{} Watch error: {:?}", style("⚠️").yellow(), e),
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
        Commands::Install { packages, global, save_dev } => {
            if *global {
                if packages.is_empty() {
                    println!("{} Please specify one or more packages to install globally", style("⚠️").yellow());
                    return Ok(());
                }

                for pkg_name in packages {
                    match global::install_global(pkg_name).await {
                        Ok(_) => {}
                        Err(e) => println!("{} Global install failed for {}: {}", style("❌").red(), pkg_name, e),
                    }
                }

                let bin_dir = global::get_global_bin_dir()?;
                println!("\n{} Global installation complete!", style("✨").bold().green());
                println!("   {} Ensure {} is in your PATH", style("💡").dim(), style(bin_dir.display()).cyan());
                return Ok(());
            }

            if !packages.is_empty() {
                let mut lockfile = manifest::CrabbyLock::load().unwrap_or_default();
                let config = config::load_config()?;
                let registry_url = config.registry.clone();
                let mut pkg_json = manifest::PackageJson::load()?;
                
                for pkg_name in packages {
                    println!("{} Installing {}...", style("📦").bold().blue(), style(pkg_name).cyan());
                    
                    let pkg_name_clone = pkg_name.clone();
                let registry_url_clone = config.registry.clone();
                let mut lockfile_clone = manifest::CrabbyLock::load().unwrap_or_default();
                
                let client = registry::get_client()?;
                // install_package now returns (version, tarball, updated_lockfile)
                let (version_str, _, updated_lock) = package_utils::install_package(&pkg_name_clone, &registry_url_clone, &client, lockfile_clone).await?;

                lockfile = updated_lock;
                
                if *save_dev {
                    pkg_json.add_dev_dependency(pkg_name.clone(), format!("^{}", version_str));
                } else {
                    pkg_json.add_dependency(pkg_name.clone(), format!("^{}", version_str));
                }
                
                println!("{} Installed {} v{}", style("✅").green(), style(pkg_name).bold(), style(&version_str).dim());
                }
                
                lockfile.save()?;
                pkg_json.save()?;
            } else {
                // Check if this is a workspace root
                let root_path = std::env::current_dir()?;
                let workspaces = workspace::find_workspaces(&root_path)?;
                
                if !workspaces.is_empty() {
                    println!("{} Found {} workspaces", style("🏢").bold().blue(), workspaces.len());
                    workspace::link_workspaces(&root_path, &workspaces)?;
                    
                    // Install dependencies for each workspace
                    println!("{} Installing workspace dependencies...", style("📦").bold().blue());
                    let config = config::load_config()?;
                    
                    for ws in workspaces {
                        println!("   Processing {}", style(&ws.name).cyan());
                        let registry_url = config.registry.clone();
                        let ws_path = ws.path.clone();
                        
                        let original_cwd = std::env::current_dir()?;
                        std::env::set_current_dir(&ws_path)?;
                        
                        let mut pkg = manifest::PackageJson::load()?;
                        let lockfile = manifest::CrabbyLock::load().unwrap_or_default();
                        let all_deps = pkg.get_all_dependencies();
                        
                        if !all_deps.is_empty() {
                            let client = registry::get_client()?;
                            let updated_lock = package_utils::install_all_packages(&all_deps, &registry_url, &client, lockfile).await?;
                            updated_lock.save()?;
                        }
                        
                        std::env::set_current_dir(original_cwd)?;
                    }
                     println!("{} Workspace installation complete", style("✅").bold().green());
                } else {
                     // Standard install all from package.json
                     println!("{} Installing dependencies...", style("📦").bold().blue());
                     let pkg_json = manifest::PackageJson::load()?;
                     let all_deps = pkg_json.get_all_dependencies();
                     let config = config::load_config()?;
                     let registry_url = config.registry.clone();
                     
                     let lockfile = manifest::CrabbyLock::load().unwrap_or_default();
                     
                     let client = registry::get_client()?;
                     let updated_lockfile = package_utils::install_all_packages(&all_deps, &registry_url, &client, lockfile).await?;

                     updated_lockfile.save()?;
                     println!("{} Done!", style("✅").bold().green());
                }
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
        Commands::List { tree } => {
            let pkg = manifest::PackageJson::load()?;
            println!("{} Installed packages:", style("📦").bold().blue());
            
            if *tree {
                let lockfile = manifest::CrabbyLock::load().ok();
                print_dependency_tree(&pkg, lockfile.as_ref())?;
            } else {
                if pkg.dependencies.is_empty() {
                    println!("  {}", style("No packages installed").dim());
                } else {
                    for (name, version) in &pkg.dependencies {
                        println!("  {} {}", style(name).cyan(), style(version).dim());
                    }
                    println!("\n{} {} packages total", style("📊").dim(), pkg.dependencies.len());
                }
            }
        }
        Commands::Update { package, global } => {
            if *global {
                 if let Some(pkg) = package {
                    match global::update_global(pkg).await {
                        Ok(_) => println!("{} Global update complete!", style("✨").bold().green()),
                        Err(e) => println!("{} Global update failed: {}", style("❌").red(), e),
                    }
                 } else {
                     println!("{} Please specify a global package to update", style("⚠️").yellow());
                 }
                 return Ok(());
            }

            if let Some(pkg_name) = package {
                println!("{} Updating {}...", style("📦").bold().blue(), pkg_name);
                let (version, _tarball) = update::update_package(&pkg_name, &config.registry).await?;
                
                 let lockfile = manifest::CrabbyLock::load().unwrap_or_default();
                 let registry_url = config.registry.clone();
                 
                 let client = registry::get_client()?;
                 let (_, _, updated_lock) = package_utils::install_package(&pkg_name, &registry_url, &client, lockfile).await?;
                 updated_lock.save()?;
                 let installed_version = version.clone();
                 let _tarball = "".to_string(); 
                
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
        Commands::Why { package } => {
            let lockfile = manifest::CrabbyLock::load()?;
            let pkg = manifest::PackageJson::load()?;
            
            println!("{} Finding reason for {}...", style("🔍").dim(), style(package).bold().cyan());
            
            let mut found = false;
            if pkg.dependencies.contains_key(package) {
                println!("{} Direct dependency in {}", style("•").green(), style("package.json").dim());
                found = true;
            }
            if pkg.dev_dependencies.contains_key(package) {
                println!("{} Direct devDependency in {}", style("•").green(), style("package.json").dim());
                found = true;
            }
            
            let paths = explorer::find_dependency_paths(&lockfile, &pkg, package);
            for path in paths {
                println!("{} {}", style("•").green(), path.join(style(" → ").dim().to_string().as_str()));
                found = true;
            }

            if !found {
                println!("{} Package {} not found in dependency graph", style("❌").red(), package);
            }
        }
        Commands::Prune { dry_run } => {
            let pkg = manifest::PackageJson::load()?;
            let lockfile = manifest::CrabbyLock::load()?;
            
            println!("{} Pruning unneeded dependencies...", style("🧹").bold().yellow());
            
            // Collect all reachable dependencies
            let mut reachable = HashSet::new();
            let all_deps = pkg.get_all_dependencies();
            
            for (name, _) in all_deps {
                collect_reachable(&name, &lockfile, &mut reachable);
            }
            
            if *dry_run {
                println!("{} DRY RUN - No files will be removed\n", style("ℹ️").bold().blue());
            }

            let node_modules = Path::new("node_modules");
            if !node_modules.exists() {
                println!("{} node_modules does not exist", style("ℹ️").dim());
                return Ok(());
            }

            let mut pruned_count = 0;
            
            // Helper to visit directories recursively (for scopes)
            fn visit_dirs(dir: &Path, reachable: &HashSet<String>, base: &Path, dry_run: bool, count: &mut usize) -> Result<()> {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if !path.is_dir() { continue; }
                    
                    let relative = path.strip_prefix(base)?;
                    let pkg_name = relative.to_string_lossy().replace("\\", "/");
                    
                    if pkg_name.starts_with(".") { continue; } // Skip .bin, .cache etc
                    
                    if pkg_name.starts_with("@") {
                        // It's a scope, look inside
                        visit_dirs(&path, reachable, base, dry_run, count)?;
                    } else if !reachable.contains(&pkg_name) {
                        println!("{} Pruning {}", style("🗑️").red(), pkg_name);
                        if !dry_run {
                            fs::remove_dir_all(&path)?;
                        }
                        *count += 1;
                    }
                }
                Ok(())
            }

            visit_dirs(node_modules, &reachable, node_modules, *dry_run, &mut pruned_count)?;

            if pruned_count == 0 {
                println!("{} No unneeded packages found", style("✅").green());
            } else {
                println!("\n{} {} packages", if *dry_run { "Would prune" } else { "Pruned" }, pruned_count);
            }
        }
    }

    Ok(())
}

// Helper removed as it's unused and we use hashing directly in package_utils
/*
fn calculate_checksum(_file_path: &Path) -> Result<String> {
    Ok("".to_string())
}
*/
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



fn print_dependency_tree(pkg: &manifest::PackageJson, _lockfile: Option<&manifest::CrabbyLock>) -> Result<()> {
    // Collect all dependencies
    let mut all_deps = Vec::new();
    for (name, version) in &pkg.dependencies {
        all_deps.push((name, version, false));
    }
    for (name, version) in &pkg.dev_dependencies {
        all_deps.push((name, version, true));
    }
    
    if all_deps.is_empty() {
        println!("  {}", style("No packages installed").dim());
        return Ok(());
    }
    
    // Sort for consistent output
    all_deps.sort_by(|a, b| a.0.cmp(b.0));
    
    let total = all_deps.len();
    for (i, (name, version, is_dev)) in all_deps.iter().enumerate() {
        let is_last = i == total - 1;
        let prefix = if is_last { "└─" } else { "├─" };
        let dev_mark = if *is_dev { style(" (dev)").yellow().dim() } else { style("").dim() };
        
        println!("{} {} {}{}", 
            style(prefix).dim(), 
            style(name).cyan(), 
            style(version).dim(),
            dev_mark
        );
        
        if let Some(lock) = _lockfile {
            print_tree_recursive(name, lock, if is_last { "   " } else { "│  " }, 1)?;
        }
    }
    
    Ok(())
}

fn print_tree_recursive(name: &str, lock: &manifest::CrabbyLock, prefix: &str, depth: usize) -> Result<()> {
    if depth > 5 { return Ok(()); } // Limit depth to keep it readable

    if let Some(dep_info) = lock.dependencies.get(name) {
        let sub_deps: Vec<_> = dep_info.dependencies.iter().collect();
        let total = sub_deps.len();
        
        for (i, (sub_name, sub_version)) in sub_deps.into_iter().enumerate() {
            let is_last = i == total - 1;
            let current_prefix = if is_last { "└─" } else { "├─" };
            
            println!("{}{} {} {}", 
                style(prefix).dim(),
                style(current_prefix).dim(),
                style(sub_name).cyan(),
                style(sub_version).dim()
            );
            
            let next_prefix = format!("{}{}", prefix, if is_last { "   " } else { "│  " });
            print_tree_recursive(sub_name, lock, &next_prefix, depth + 1)?;
        }
    }
    Ok(())
}

fn collect_reachable(name: &str, lock: &manifest::CrabbyLock, reachable: &mut HashSet<String>) {
    if reachable.contains(name) { return; }
    reachable.insert(name.to_string());
    
    if let Some(dep_info) = lock.dependencies.get(name) {
        for sub_dep in dep_info.dependencies.keys() {
            collect_reachable(sub_dep, lock, reachable);
        }
    }
}

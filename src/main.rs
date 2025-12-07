mod manifest;
mod package_utils;
mod runner;
mod config;
mod node_runtime;
mod update;
mod safety;

use clap::{Parser, Subcommand};
use console::style;
use anyhow::Result;

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
    },
    /// Initialize a new Crabby project
    Init,
    /// Install a package from NPM registry
    #[command(alias = "i")]
    Install {
        /// The name of the package to install (installs all if not specified)
        package: Option<String>,
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
        }
        Commands::Cook { script, ts, js } => {
            let node_path = node_runtime::get_node_path()?;
            let node_str = node_path.to_string_lossy();
            
            if let Some(ts_file) = ts {
                let cmd = format!("npx -y tsx {}", ts_file);
                runner::run_script_with_node(&cmd, None, &node_str)?;
            } else if let Some(js_file) = js {
                let cmd = format!("{} {}", node_str, js_file);
                runner::run_script(&cmd, None)?;
            } else if let Some(script_name) = script {
                let path = std::path::Path::new(&script_name);
                if path.exists() && (script_name.ends_with(".js") || script_name.ends_with(".ts")) {
                    if script_name.ends_with(".ts") {
                        let cmd = format!("npx -y tsx {}", script_name);
                        runner::run_script_with_node(&cmd, None, &node_str)?;
                    } else {
                        let cmd = format!("{} {}", node_str, script_name);
                        runner::run_script(&cmd, None)?;
                    }
                } else {
                    run_package_script(&script_name)?;
                }
            }
        }
        Commands::Start => {
            run_package_script("start")?;
        }
        Commands::Test => {
            run_package_script("test")?;
        }
        Commands::Install { package, save_dev } => {
            if let Some(pkg_name) = package {
                // Install single package
                let start = std::time::Instant::now();
                println!("{} {}", style("📦").bold().blue(), style(format!("Installing {}...", pkg_name)).bold());

                let (version, tarball) = package_utils::install_package(pkg_name, &config.registry)?;
                
                let mut pkg_json = manifest::PackageJson::load()?;
                if *save_dev {
                    pkg_json.add_dev_dependency(pkg_name.clone(), format!("^{}", version));
                    println!("{} Saved to devDependencies", style("📝").dim());
                } else {
                    pkg_json.add_dependency(pkg_name.clone(), format!("^{}", version));
                    println!("{} Saved to dependencies", style("📝").dim());
                }
                pkg_json.save()?;

                let mut lockfile = manifest::CrabbyLock::load()?;
                lockfile.add_package(pkg_name.clone(), version.clone(), tarball);
                lockfile.save()?;

                let duration = start.elapsed();
                println!(
                    "{} Installed {} v{} in {}", 
                    style("✅").bold().green(), 
                    style(pkg_name).bold().white(),
                    style(version).bold().cyan(),
                    style(humantime::format_duration(duration)).bold().magenta()
                );
            } else {
                // Install all dependencies from package.json
                install_all_dependencies(&config.registry).await?;
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
                
                package_utils::install_package(&pkg_name, &config.registry)?;
                
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
    let mut installed = 0;
    let mut failed = Vec::new();
    
    // Install packages with progress
    for (name, _version) in &all_deps {
        print!("  {} Installing {}... ", style("⬇️").dim(), style(name).cyan());
        std::io::Write::flush(&mut std::io::stdout())?;
        
        match package_utils::install_package(name, registry) {
            Ok((ver, _)) => {
                println!("{} {}", style("✅").green(), style(ver).dim());
                installed += 1;
            }
            Err(e) => {
                println!("{} {}", style("❌").red(), style(format!("{}", e)).dim());
                failed.push(name.clone());
            }
        }
    }
    
    let duration = start.elapsed();
    
    println!("\n{} Installed {} packages in {}", 
        style("🎉").bold().green(),
        installed,
        style(humantime::format_duration(duration)).bold().magenta()
    );
    
    if !failed.is_empty() {
        println!("{} Failed to install: {:?}", style("⚠️").yellow(), failed);
    }
    
    Ok(())
}

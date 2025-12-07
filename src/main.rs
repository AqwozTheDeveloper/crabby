mod manifest;
mod package_utils;
mod runner;
mod config;
mod node_runtime;

use clap::{Parser, Subcommand};
use console::style;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "crabby")]
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
        /// The name of the package to install
        package: String,
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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Load config (defaults to standard registry if file missing or invalid)
    let config = config::CrabbyConfig::load()?;
    match &cli.command {
        Commands::Init => {
            println!("{} {}", style("🦀").bold().red(), style("Initializing Crabby Kitchen...").bold());
            manifest::ensure_package_files()?;
            println!("{}", style("Created package.json").green());
        }
        Commands::Cook { script, ts, js } => {
            // Get Node.js path (system or downloaded)
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
        Commands::Install { package } => {
             let start = std::time::Instant::now();
             println!("{} {}", style("📦").bold().blue(), style(format!("Installing {}...", package)).bold());

             // Use the new recursive installer
             // This will also run postinstall scripts
             let (version, tarball) = package_utils::install_package(package, &config.registry)?;
             
             // spinner.finish_and_clear();
             
             let mut pkg_json = manifest::PackageJson::load()?;
             pkg_json.add_dependency(package.clone(), format!("^{}", version));
             pkg_json.save()?;

             let mut lockfile = manifest::CrabbyLock::load()?;
             lockfile.add_package(package.clone(), version.clone(), tarball);
             lockfile.save()?;

             let duration = start.elapsed();

             println!(
                "{} Installed {} v{} in {}", 
                style("✅").bold().green(), 
                style(package).bold().white(),
                style(version).bold().cyan(),
                style(humantime::format_duration(duration)).bold().magenta()
            );
        }
        Commands::Remove { package } => {
             println!("{} {}", style("🗑️").bold().red(), style(format!("Removing {}...", package)).bold());
             
             // Remove from package.json
             let mut pkg_json = manifest::PackageJson::load()?;
             if !pkg_json.dependencies.contains_key(package) {
                 println!("{} Package '{}' not found in dependencies", style("❌").red(), package);
                 return Ok(());
             }
             pkg_json.remove_dependency(package);
             pkg_json.save()?;
             
             // Remove from lockfile
             let mut lockfile = manifest::CrabbyLock::load()?;
             lockfile.dependencies.remove(package);
             lockfile.save()?;
             
             // Remove from node_modules
             let package_path = std::path::Path::new("node_modules").join(package);
             if package_path.exists() {
                 std::fs::remove_dir_all(&package_path)?;
             }
             
             println!("{} Removed {}", style("✅").bold().green(), style(package).bold().white());
        }
    }

    Ok(())
}

fn run_package_script(script_name: &str) -> Result<()> {
    let pkg = manifest::PackageJson::load()?;
    if let Some(command_str) = pkg.scripts.get(script_name) {
        runner::run_script(command_str, None)?; // None = use current dir
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

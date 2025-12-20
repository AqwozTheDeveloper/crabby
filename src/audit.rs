use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{config, manifest, registry};

#[derive(Debug, Serialize)]
struct AuditRequest {
    name: String,
    version: String,
    install: Vec<String>,
    #[serde(rename = "remove")]
    remove: Vec<String>,
    metadata: HashMap<String, String>,
    requires: HashMap<String, String>,
    dependencies: HashMap<String, AuditDependency>,
}

#[derive(Debug, Serialize)]
struct AuditDependency {
    version: String,
}

#[derive(Debug, Deserialize, Default)]
struct AuditResponse {
    #[serde(default)]
    advisories: HashMap<String, Advisory>,
    #[serde(default)]
    metadata: AuditMetadata,
}

#[derive(Debug, Deserialize, Default)]
struct AuditMetadata {
    vulnerabilities: HashMap<String, u64>,
}

#[derive(Debug, Deserialize, Default)]
struct Advisory {
    _id: u64,
    title: String,
    #[serde(rename = "module_name")]
    module_name: String,
    severity: String,
    _overview: String,
    _recommendation: String,
    url: String,
}

pub fn check_vulnerabilities() -> Result<()> {
    println!("{} running security audit...", style("🛡️").bold().blue());

    let config = config::load_config()?;
    let client = registry::get_client()?;
    let lockfile = manifest::CrabbyLock::load().unwrap_or_default();
    let pkg_json = manifest::PackageJson::load().unwrap_or_default();

    if lockfile.dependencies.is_empty() {
        println!("{} No packages found in lockfile.", style("ℹ").blue());
        return Ok(());
    }

    // Construct Audit Payload
    // We treating all locked packages as direct dependencies for the audit check
    // This allows us to check all versions we have installed.
    let mut dependencies = HashMap::new();
    let mut requires = HashMap::new();

    for (name, dep) in &lockfile.dependencies {
        dependencies.insert(name.clone(), AuditDependency {
            version: dep.version.clone(),
        });
        requires.insert(name.clone(), dep.version.clone());
    }

    let payload = AuditRequest {
        name: pkg_json.name.clone(),
        version: pkg_json.version.clone(),
        install: vec![],
        remove: vec![],
        metadata: HashMap::new(), // npm_config_user_agent etc?
        requires,
        dependencies,
    };
    
    // println!("Audit payload size: {} entries", payload.dependencies.len());

    let registry_url = config.registry.trim_end_matches('/');
    let audit_url = format!("{}/-/npm/v1/security/audits", registry_url);

    let resp = client.post(&audit_url)
        .json(&payload)
        .send()
        .context("Failed to contact audit endpoint")?;

    if !resp.status().is_success() {
         println!("{} Audit request failed: {}", style("⚠️").yellow(), resp.status());
         // Try to print body for debugging
         // let body = resp.text()?;
         // println!("Body: {}", body);
         return Ok(());
    }

    let report: AuditResponse = resp.json()
        .context("Failed to parse audit response")?;

    // Print Report
    let total_vulns: u64 = report.metadata.vulnerabilities.values().sum();

    if total_vulns == 0 {
        println!("\n{} No known vulnerabilities found", style("✅").bold().green());
    } else {
        println!("\n{} Found {} vulnerabilities", style("🚨").bold().red(), total_vulns);
        
        let mut sorted_advisories: Vec<&Advisory> = report.advisories.values().collect();
        sorted_advisories.sort_by(|a, b| b.severity.cmp(&a.severity)); // TODO: Better severity sort (critical > high)

        for advisory in sorted_advisories {
            println!("\n{}", style(format!("severity: {}", advisory.severity)).bold().red());
            println!("  Package: {}", style(&advisory.module_name).bold());
            println!("  Title:   {}", advisory.title);
            println!("  Path:    {}", advisory.module_name); // Simplified, we don't have full path in flat structure
            println!("  More:    {}", style(&advisory.url).dim());
        }
        
        println!("\nRun `crabby update <package>` to fix.", );
    }

    Ok(())
}

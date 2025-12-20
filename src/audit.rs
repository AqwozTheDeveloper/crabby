use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{manifest, registry};

#[derive(Debug, Serialize)]
struct OsvPackage {
    name: String,
    ecosystem: String,
}

#[derive(Debug, Serialize)]
struct OsvQuery {
    package: OsvPackage,
    version: String,
}

#[derive(Debug, Serialize)]
struct OsvBatchRequest {
    queries: Vec<OsvQuery>,
}

#[derive(Debug, Deserialize)]
struct OsvBatchResponse {
    #[serde(default)]
    results: Vec<OsvResult>,
}

#[derive(Debug, Deserialize)]
struct OsvResult {
    #[serde(default)]
    vulns: Vec<OsvVulnerability>,
}

#[derive(Debug, Deserialize)]
struct OsvVulnerability {
    id: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    details: String,
    #[serde(default)]
    database_specific: HashMap<String, serde_json::Value>,
}

pub fn check_vulnerabilities() -> Result<()> {
    println!("{} {} scanning dependencies via OSV.dev...", style("🦀").bold().cyan(), style("🛡️").bold().blue());

    let client = registry::get_client()?;
    let lockfile = manifest::CrabbyLock::load().unwrap_or_default();

    if lockfile.dependencies.is_empty() {
        println!("{} No packages found in lockfile.", style("ℹ").blue());
        return Ok(());
    }

    let mut queries = Vec::new();
    let mut name_map = Vec::new(); // To track which result belongs to which package

    for (name, dep) in &lockfile.dependencies {
        queries.push(OsvQuery {
            package: OsvPackage {
                name: name.clone(),
                ecosystem: "npm".to_string(),
            },
            version: dep.version.clone(),
        });
        name_map.push((name.clone(), dep.version.clone()));
    }

    // OSV allows up to 1000 queries per batch
    let batch_request = OsvBatchRequest { queries };

    let resp = client.post("https://api.osv.dev/v1/querybatch")
        .json(&batch_request)
        .send()
        .context("Failed to contact OSV.dev API")?;

    if !resp.status().is_success() {
         println!("{} Security audit failed: OSV API returned {}", style("⚠️").yellow(), resp.status());
         return Ok(());
    }

    let batch_resp: OsvBatchResponse = resp.json()
        .context("Failed to parse OSV response")?;

    let mut total_vulns = 0;
    let mut found_any = false;

    for (i, result) in batch_resp.results.iter().enumerate() {
        if result.vulns.is_empty() {
            continue;
        }

        if !found_any {
            println!("\n{}", style("Vulnerability Report:").bold().underlined());
            found_any = true;
        }

        let (pkg_name, pkg_version) = &name_map[i];
        total_vulns += result.vulns.len();

        for vuln in &result.vulns {
            let severity = vuln.database_specific.get("severity")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN");

            println!("\n{}", style(format!("severity: {}", severity)).bold().red());
            println!("  Package: {}@{}", style(pkg_name).bold(), pkg_version);
            println!("  ID:      {}", style(&vuln.id).cyan());
            println!("  Summary: {}", if vuln.summary.is_empty() { &vuln.details } else { &vuln.summary });
            println!("  More:    {}", style(format!("https://osv.dev/vulnerability/{}", vuln.id)).dim());
        }
    }

    if !found_any {
        println!("\n{} No known vulnerabilities found across {} dependencies.", style("✅").bold().green(), lockfile.dependencies.len());
    } else {
        println!("\n{} Found {} vulnerabilities across {} dependencies.", style("🚨").bold().red(), total_vulns, lockfile.dependencies.len());
        println!("\nRun `crabby update <package>` to fix or research better alternatives.", );
    }

    Ok(())
}

use anyhow::Result;
use console::style;
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResponse {
    objects: Vec<SearchObject>,
    total: usize,
}

#[derive(Deserialize)]
struct SearchObject {
    package: PackageInfo,
}

#[derive(Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    #[serde(rename = "links")]
    _links: Option<serde_json::Value>,
}

/// Search for packages in npm registry
pub async fn search_packages(query: &str, limit: usize) -> Result<()> {
    println!("{} Searching for '{}'...\n", style("🔍").cyan(), query);
    
    let url = format!(
        "https://registry.npmjs.org/-/v1/search?text={}&size={}",
        urlencoding::encode(query),
        limit
    );
    
    let response = reqwest::get(&url)
        .await?
        .error_for_status()?
        .json::<SearchResponse>()
        .await?;
    
    if response.objects.is_empty() {
        println!("{} No packages found", style("❌").red());
        return Ok(());
    }
    
    println!("{} Found {} packages:\n", 
        style("✓").green(), 
        response.total
    );
    
    for obj in response.objects.iter().take(limit) {
        let pkg = &obj.package;
        
        println!("{} {}", 
            style(&pkg.name).bold().cyan(),
            style(format!("v{}", pkg.version)).dim()
        );
        
        if let Some(desc) = &pkg.description {
            println!("  {}", style(desc).dim());
        }
        
        if let Some(keywords) = &pkg.keywords {
            if !keywords.is_empty() {
                println!("  {} {}", 
                    style("Keywords:").dim(),
                    keywords.join(", ")
                );
            }
        }
        
        println!();
    }
    
    if response.total > limit {
        println!("{} Showing {}/{} results", 
            style("ℹ").blue(),
            limit,
            response.total
        );
        println!("{} Refine your search for more specific results\n",
            style("💡").cyan()
        );
    }
    
    Ok(())
}

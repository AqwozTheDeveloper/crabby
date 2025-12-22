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
    crate::ui::print_step(crate::ui::Icons::SEARCH, &format!("Searching for '{}'...", query));
    println!();
    
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
        crate::ui::print_error(&format!("No packages found for '{}'", query));
        return Ok(());
    }
    
    for obj in response.objects.iter().take(limit) {
        let pkg = &obj.package;
        
        // We don't have download count in this simple search response yet, 
        // but we can pass None or find it if we wanted to.
        crate::ui::print_package_card(
            &pkg.name, 
            &pkg.version, 
            pkg.description.as_deref(),
            None
        );
    }
    
    if response.total > limit {
        println!();
        crate::ui::print_info(&format!("Showing {}/{} results", limit, response.total));
        crate::ui::print_info("Refine your search for more specific results");
    }
    
    Ok(())
}

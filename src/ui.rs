use dialoguer::{theme::ColorfulTheme, Select, FuzzySelect};
use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

// ========== Icon Constants ==========

pub struct Icons;

#[allow(dead_code)]
impl Icons {
    // Package Operations
    pub const PACKAGE: &'static str = "📦";
    pub const INSTALL: &'static str = "📥";
    pub const REMOVE: &'static str = "🗑️";
    pub const UPDATE: &'static str = "⬆️";
    pub const DOWNLOAD: &'static str = "⬇️";
    pub const SEARCH: &'static str = "🔍";
    pub const LIST: &'static str = "📋";
    pub const INFO: &'static str = "ℹ️";
    pub const LOCK: &'static str = "🔒";
    pub const WARNING: &'static str = "⚠️";
    pub const SUCCESS: &'static str = "✅";
    pub const ERROR: &'static str = "❌";
    pub const RUN: &'static str = "🚀";
    pub const COOK: &'static str = "🍳";
    pub const BUILD: &'static str = "🏗️";
    pub const CLEAN: &'static str = "🧹";
    pub const AUDIT: &'static str = "🔐";
    
    // Context Icons
    pub const TIP: &'static str = "💡";
    pub const TARGET: &'static str = "🎯";
    pub const FOLDER: &'static str = "📂";
    pub const FILE: &'static str = "📄";
    pub const TIME: &'static str = "⏱️";
    pub const LINK: &'static str = "🔗";
    pub const NETWORK: &'static str = "🌐";
    pub const CACHE: &'static str = "💾";
    pub const CONFIG: &'static str = "🔧";
    pub const ROCKET: &'static str = "🚀";
    pub const PARTY: &'static str = "🎉";
    pub const CHECKMARK: &'static str = "✓";
    pub const ARROW_RIGHT: &'static str = "▸";
    pub const TREE_BRANCH: &'static str = "├─";
    pub const TREE_LAST: &'static str = "└─";
}

// ========== Basic Output Functions ==========

pub fn print_step(emoji: &str, message: &str) {
    println!("{} {}", style(emoji).bold(), style(message).bold());
}

pub fn print_success(message: &str) {
    println!("{} {}", style(Icons::SUCCESS).green(), style(message).green());
}

pub fn print_error(message: &str) {
    println!("{} {}", style(Icons::ERROR).red(), style(message).red());
}

pub fn print_info(message: &str) {
    println!("{} {}", style(Icons::TIP).dim(), style(message).dim());
}

pub fn print_warning(message: &str) {
    println!("{} {}", style(Icons::WARNING).yellow(), style(message).yellow());
}

// ========== Formatted Output ==========

pub fn print_header(title: &str) {
    println!("\n{}", style(title).bold().cyan().underlined());
}

pub fn print_section(title: &str) {
    println!("\n{}", style(title).bold());
}

pub fn print_item(icon: &str, name: &str, version: &str) {
    println!("  {} {}  {}", 
        style(icon).dim(),
        style(name).cyan().bold(),
        style(version).dim()
    );
}

pub fn print_tree_item(is_last: bool, name: &str, version: Option<&str>) {
    let branch = if is_last { Icons::TREE_LAST } else { Icons::TREE_BRANCH };
    if let Some(v) = version {
        println!("  {} {} {}", 
            style(branch).dim(),
            style(name).cyan(),
            style(format!("@{}", v)).dim()
        );
    } else {
        println!("  {} {}", 
            style(branch).dim(),
            style(name).cyan()
        );
    }
}

// ========== Progress Bars ==========

pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.cyan} {msg} [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("█▓░")
    );
    pb.set_message(message.to_string());
    pb
}

pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

// ========== Box Drawing ==========

pub fn print_box(content: &[String]) {
    if content.is_empty() {
        return;
    }
    
    let max_width = content.iter().map(|s| s.len()).max().unwrap_or(0);
    let top = format!("╭─{}─╮", "─".repeat(max_width));
    let bottom = format!("╰─{}─╯", "─".repeat(max_width));
    
    println!("{}", style(top).dim());
    for line in content {
        println!("{} {:<width$} {}", 
            style("│").dim(),
            line,
            style("│").dim(),
            width = max_width
        );
    }
    println!("{}", style(bottom).dim());
}

pub fn print_package_card(name: &str, version: &str, description: Option<&str>, downloads: Option<&str>) {
    let mut lines = vec![
        format!("{} {}  {}", Icons::PACKAGE, style(name).bold().cyan(), style(version).dim()),
    ];
    
    if let Some(desc) = description {
        lines.push(format!("   {}", style(desc).dim()));
    }
    
    if let Some(dl) = downloads {
        lines.push(format!("   {} {}", Icons::DOWNLOAD, style(dl).dim()));
    }
    
    print_box(&lines);
}

// ========== Table Formatting ==========

pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        return;
    }
    
    // Calculate column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }
    
    // Print header
    print!("  ");
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$}  ", style(header).bold(), width = widths[i]);
    }
    println!();
    
    // Print separator
    print!("  ");
    for width in &widths {
        print!("{}  ", "─".repeat(*width));
    }
    println!();
    
    // Print rows
    for row in rows {
        print!("  ");
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                if i == 0 {
                    print!("{:<width$}  ", style(cell).cyan(), width = widths[i]);
                } else {
                    print!("{:<width$}  ", cell, width = widths[i]);
                }
            }
        }
        println!();
    }
}

// ========== Formatting Helpers ==========

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{} {}", size as u64, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

pub fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let mins = ms / 60000;
        let secs = (ms % 60000) / 1000;
        format!("{}m {}s", mins, secs)
    }
}

pub fn format_number(num: u64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

// ========== Interactive Selection ==========

pub fn prompt_selection(items: &[String], prompt: &str) -> Result<Option<usize>> {
    if items.is_empty() {
        return Ok(None);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact_opt()?;

    Ok(selection)
}

pub fn prompt_fuzzy_selection(items: &[String], prompt: &str) -> Result<Option<usize>> {
    if items.is_empty() {
        return Ok(None);
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact_opt()?;

    Ok(selection)
}

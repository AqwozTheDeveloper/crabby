use dialoguer::{theme::ColorfulTheme, Select, FuzzySelect};
use anyhow::Result;
use console::style;

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

pub fn print_step(emoji: &str, message: &str) {
    println!("{} {}", style(emoji).bold(), style(message).bold());
}

pub fn print_success(message: &str) {
    println!("{} {}", style("✅").green(), style(message).green());
}

pub fn print_error(message: &str) {
    println!("{} {}", style("❌").red(), style(message).red());
}

pub fn print_info(message: &str) {
    println!("{} {}", style("💡").dim(), style(message).dim());
}

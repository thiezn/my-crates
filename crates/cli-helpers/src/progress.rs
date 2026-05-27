//! Shared indicatif styles for CLI progress reporting.

pub use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn progress_style(template: &str) -> ProgressStyle {
    match ProgressStyle::with_template(template) {
        Ok(style) => style,
        Err(_) => ProgressStyle::default_bar(),
    }
}

/// Returns the style for a top-level progress bar.
pub fn overall_style() -> ProgressStyle {
    progress_style("{prefix:.bold.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .progress_chars("━╸─")
}

/// Returns the style for a child progress bar.
pub fn item_style() -> ProgressStyle {
    progress_style("  {prefix:.bold.green} [{bar:30.green/dim}] {pos}/{len} {msg}")
        .progress_chars("━╸─")
}

/// Returns the style for an indeterminate spinner.
pub fn spinner_style() -> ProgressStyle {
    progress_style("  {prefix:.bold.yellow} {spinner:.yellow} {msg}")
}

/// Creates an item-level spinner inside a multi-progress display.
pub fn item_spinner(progress: &MultiProgress, prefix: &str) -> ProgressBar {
    let progress_bar = progress.add(ProgressBar::new_spinner());
    progress_bar.set_style(spinner_style());
    progress_bar.set_prefix(prefix.to_string());
    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));
    progress_bar
}

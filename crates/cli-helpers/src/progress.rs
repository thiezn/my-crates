pub use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn progress_style(template: &str) -> ProgressStyle {
    match ProgressStyle::with_template(template) {
        Ok(style) => style,
        Err(_) => ProgressStyle::default_bar(),
    }
}

/// Style for an overall (top-level) progress bar.
pub fn overall_style() -> ProgressStyle {
    progress_style("{prefix:.bold.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .progress_chars("━╸─")
}

/// Style for a child / item-level progress bar (determinate).
pub fn item_style() -> ProgressStyle {
    progress_style("  {prefix:.bold.green} [{bar:30.green/dim}] {pos}/{len} {msg}")
        .progress_chars("━╸─")
}

/// Style for a spinner (indeterminate progress).
pub fn spinner_style() -> ProgressStyle {
    progress_style("  {prefix:.bold.yellow} {spinner:.yellow} {msg}")
}

/// Create an item-level spinner (indeterminate) inside a MultiProgress.
pub fn item_spinner(progress: &MultiProgress, prefix: &str) -> ProgressBar {
    let progress_bar = progress.add(ProgressBar::new_spinner());
    progress_bar.set_style(spinner_style());
    progress_bar.set_prefix(prefix.to_string());
    progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));
    progress_bar
}

use std::fmt;

use dialoguer::console::Style;
use dialoguer::theme::Theme;
use indicatif::ProgressStyle;

pub struct CLITheme {
    pub dimmed: Style,
    pub normal: Style,
    pub highlight: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
}

impl CLITheme {
    pub fn spinner() -> ProgressStyle {
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
    }

    pub fn failed_spinner() -> ProgressStyle {
        ProgressStyle::with_template("{prefix:.red} {msg:.red}")
            .unwrap()
    }

    pub fn success_spinner() -> ProgressStyle {
        ProgressStyle::with_template("{prefix:.green} {msg:.green}")
            .unwrap()
    }
}

impl Default for CLITheme {
    fn default() -> Self {
        Self {
            dimmed: Style::new().dim(),
            normal: Style::new(),
            highlight: Style::new().cyan(),
            success: Style::new().green(),
            warning: Style::new().yellow(),
            error: Style::new().red(),
        }
    }
}

impl Theme for CLITheme {
    fn format_confirm_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        if !prompt.is_empty() {
            write!(
                f,
                "{} {}: ",
                self.dimmed.apply_to("question"),
                self.normal.apply_to(prompt)
            )?;
        }

        match default {
            None => write!(f, "{}", self.highlight.apply_to("[y/n]"),),
            Some(true) => write!(f, "{}", self.highlight.apply_to("[Y/n]"),),
            Some(false) => write!(f, "{}", self.highlight.apply_to("[y/N]"),),
        }
    }

    /// Formats a confirm prompt after selection.
    fn format_confirm_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: Option<bool>,
    ) -> fmt::Result {
        if !prompt.is_empty() {
            write!(
                f,
                "{} {} ",
                self.dimmed.apply_to("question"),
                self.normal.apply_to(prompt)
            )?;
        }
        let selection = selection.map(|b| {
            if b {
                self.success.apply_to("yes")
            } else {
                self.error.apply_to("no")
            }
        });

        match selection {
            Some(selection) => write!(f, "{}", selection,),
            None => write!(f, "",),
        }
    }

    /// Formats a select prompt.
    fn format_select_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        if prompt.is_empty() {
            write!(f, "{}: ", self.dimmed.apply_to("question"))
        } else {
            write!(f, "{}: ", self.normal.clone().bold().apply_to(prompt))
        }
    }

    /// Formats a select prompt after selection.
    fn format_select_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        if prompt.is_empty() {
            write!(f, "{}: ", self.dimmed.apply_to("question"))?;
        } else {
            write!(f, "{}: ", self.normal.clone().bold().apply_to(prompt))?;
        }
        write!(f, "{}", self.success.apply_to(sel))
    }

    /// Formats a select prompt item.
    fn format_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        active: bool,
    ) -> fmt::Result {
        if active {
            write!(
                f,
                "{} {}",
                self.success.apply_to("❯"),
                self.normal.apply_to(text)
            )
        } else {
            write!(
                f,
                "{} {}",
                self.dimmed.apply_to(" "),
                self.dimmed.apply_to(text)
            )
        }
    }
}

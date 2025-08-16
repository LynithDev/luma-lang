use std::{collections::HashMap, fmt::Debug, sync::{Arc, Mutex}};

use luma_core::CodeInput;
use owo_colors::OwoColorize;

use crate::{DiagnosticReport, DiagnosticKind};

#[derive(Debug, Clone)]
pub struct Reporter {
    pub source_name: String,
    inner: Arc<Mutex<ReporterInner>>,
}

impl Reporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Reporter {
            source_name: format!("T{}", std::thread::current().name().unwrap_or("main")),
            inner: Arc::new(Mutex::new(ReporterInner::new())),
        }
    }

    pub fn with_name(&self, source_name: &str) -> Self {
        Reporter {
            source_name: format!("{}:{}", self.source_name, source_name),
            inner: Arc::clone(&self.inner),
        }
    }

    pub fn report(&self, diagnostic: DiagnosticReport) {
        let mut guard = self.lock();
        guard.report(&self.source_name, diagnostic);
    }

    pub fn is_clean(&self) -> bool {
        let guard = self.lock();
        guard.is_clean(&self.source_name)
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        let guard = self.lock();
        guard.diagnostic_count(kind)
    }

    pub fn formatted(&self, source: &CodeInput) -> String {
        let guard = self.lock();
        let mut f = String::new();

        let warning_count = guard.diagnostic_count(DiagnosticKind::Warning);
        let error_count = guard.diagnostic_count(DiagnosticKind::Error);

        for diagnostic in guard.diagnostics.get(&self.source_name).unwrap_or(&Vec::new()) {
            let lines: std::iter::Skip<std::str::Lines<'_>> = source.source()
                .lines()
                .skip(diagnostic.cursor.line.saturating_sub(1));

            // we don't know how long the line count is, so we iterate through the lines
            // until we reach the length of the span
            let byte_len = diagnostic.span.end.saturating_sub(diagnostic.span.start);
            
            let mut code: Vec<&str> = Vec::new();
            let mut scanned_bytes: usize = 0;
            let mut last_line = diagnostic.cursor.line;

            // Calculate and append all the code lines
            for line in lines {
                if scanned_bytes > byte_len {
                    break;
                }
                scanned_bytes += line.len() + 1; // +1 for the newline character
                
                code.push(line);

                last_line += 1;
            }
            
            // Finally begin formatting the output
            let line_num_size = last_line.to_string().len();

            // write the diagnostic header
            f.push_str(&format!("{}:", diagnostic.message.kind()).red().to_string());
            f.push(' ');
            f.push_str(&diagnostic.message.bright_red().bold().to_string());
            f.push('\n');

            // write the <file>:line:column of the diagnostic
            f.push_str(&(" ".repeat(line_num_size) + "--> ").bright_black().to_string());
            f.push_str(&format!("{}:{}:{}\n", source.path(), diagnostic.cursor.line, diagnostic.cursor.column).bright_black().to_string());
            
            write_bordered(&mut f, None, "\n", line_num_size);

            scanned_bytes = 0;
            for (index, line) in code.into_iter().enumerate() {
                write_bordered(&mut f, Some(diagnostic.cursor.line + index), line, line_num_size);
                f.push('\n');

                scanned_bytes += line.len() + 1; // +1 for the newline character

                let underline_start = if index == 0 {
                    diagnostic.cursor.column.saturating_sub(1)
                } else {
                    0
                };

                let underline_end = underline_start + byte_len;
                let underline = " ".repeat(underline_start) + &"^".repeat(underline_end.saturating_sub(underline_start));

                write_bordered(&mut f, None, &underline, line_num_size);
                f.push('\n');
            }
            
            write_bordered(&mut f, None, "\n", line_num_size);

            f.push('\n');
        }

        if warning_count > 0 {
            f.push_str(&format!("warning: found {warning_count} warnings\n").yellow().to_string());
        }

        if error_count > 0 {
            f.push_str(&format!("error: could not compile due to {error_count} error(s)\n").red().to_string());
        }

        f.trim().to_string()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ReporterInner> {
        self.inner.lock().expect("couldn't lock reporter mutex")
    }

}

fn write_bordered(f: &mut String, pre_border: Option<usize>, line: &str, longest_line_num: usize) {
    f.push_str(&format!("{:>longest_line_num$}", pre_border.map_or_else(String::new, |num| num.blue().to_string())));
    f.push_str(&" | ".blue().to_string());

    f.push_str(line);
}

struct ReporterInner {
    pub (crate) diagnostics: HashMap<String, Vec<DiagnosticReport>>,
    pub (crate) kind_count: HashMap<DiagnosticKind, usize>,
}

impl Debug for ReporterInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReporterInner")
            .field("diagnostics", &self.diagnostics.len())
            .field("kind_count", &self.kind_count)
            .finish()
    }
}

unsafe impl Send for ReporterInner {}
unsafe impl Sync for ReporterInner {}

impl ReporterInner {
    pub fn new() -> Self {
        ReporterInner {
            diagnostics: HashMap::new(),
            kind_count: HashMap::new(),
        }
    }

    pub fn report(&mut self, source_file: &str, diagnostic: DiagnosticReport) {
        *self.kind_count.entry(diagnostic.message.kind()).or_default() += 1;
        self.diagnostics.entry(source_file.to_string()).or_default().push(diagnostic);
    }

    pub fn is_clean(&self, source_file: &str) -> bool {
        self.diagnostics.get(source_file).map(|d| d.is_empty()).unwrap_or(true)
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        self.kind_count.get(&kind).cloned().unwrap_or(0)
    }
}

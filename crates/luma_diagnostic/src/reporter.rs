use std::{collections::HashMap, fmt::Debug, rc::Rc, sync::{Arc, Mutex}};

use luma_core::{CodeSource, CodeSourceKind};
use owo_colors::OwoColorize;

use crate::{DiagnosticReport, DiagnosticKind};

type ReporterName = Rc<String>;

#[derive(Debug, Clone)]
pub struct Reporter {
    source: CodeSourceKind,
    reporter_name: ReporterName,
    inner: Arc<Mutex<ReporterInner>>,
}

impl Reporter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Reporter {
            source: CodeSourceKind::Virtual,
            reporter_name: Rc::new(format!("T{}", std::thread::current().name().unwrap_or("main"))),
            inner: Arc::new(Mutex::new(ReporterInner::new())),
        }
    }

    pub fn source(&self) -> &CodeSourceKind {
        &self.source
    }

    pub fn name(&self) -> Rc<String> {
        Rc::clone(&self.reporter_name)
    }

    pub fn with_name(&self, name: &str) -> Self {
        Reporter {
            source: self.source.clone(),
            reporter_name: Rc::new(format!("{}:{}", self.reporter_name, name)),
            inner: Arc::clone(&self.inner),
        }
    }

    pub fn report(&self, diagnostic: DiagnosticReport) {
        let mut guard = self.lock();
        guard.report(&self.source, self.name(), diagnostic);
    }

    pub fn report_all(&self, diagnostics: Vec<DiagnosticReport>) {
        let mut guard = self.lock();
        for diagnostic in diagnostics {
            guard.report(&self.source, self.name(), diagnostic);
        }
    }

    pub fn is_clean(&self) -> bool {
        let guard = self.lock();
        guard.is_clean(&self.name())
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        let guard = self.lock();
        guard.diagnostic_count(kind)
    }

    pub fn formatted_for(&self, source: &CodeSource) -> String {
        let guard = self.lock();
        let mut f = String::new();

        let warning_count = guard.diagnostic_count(DiagnosticKind::Warning);
        let error_count = guard.diagnostic_count(DiagnosticKind::Error);

        for entry in guard.diagnostics.get(&self.source().source_name()).unwrap_or(&Vec::new()) {
            let ReportedEntry { diagnostic, reporter_name } = entry;

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
            f.push_str(&(" at ").bright_black().to_string());
            f.push_str(&format!("{}:{}:{}", source.source_name(), diagnostic.cursor.line, diagnostic.cursor.column).white().to_string());

            // write the reporter name
            f.push_str(&(" (reported by ").black().to_string());
            f.push_str(&reporter_name.bright_black().bold().to_string());
            f.push(')');
            f.push('\n');
            
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
    pub (crate) diagnostics: HashMap<String, Vec<ReportedEntry>>,
    pub (crate) kind_count: HashMap<DiagnosticKind, usize>,
}

struct ReportedEntry {
    pub (crate) reporter_name: ReporterName,
    pub (crate) diagnostic: DiagnosticReport,
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

    pub fn report(&mut self, source: &CodeSourceKind, reporter_name: ReporterName, diagnostic: DiagnosticReport) {
        *self.kind_count.entry(diagnostic.message.kind()).or_default() += 1;
        self.diagnostics.entry(source.source_name()).or_default().push(ReportedEntry {
            reporter_name,
            diagnostic,
        });
    }

    pub fn is_clean(&self, source_file: &str) -> bool {
        self.diagnostics.get(source_file).map(|d| d.is_empty()).unwrap_or(true)
    }

    pub fn diagnostic_count(&self, kind: DiagnosticKind) -> usize {
        self.kind_count.get(&kind).cloned().unwrap_or(0)
    }
}

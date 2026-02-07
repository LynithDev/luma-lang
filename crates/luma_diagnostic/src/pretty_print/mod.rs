use annotate_snippets::{AnnotationKind, Group, Level, Origin, Renderer, Snippet};
use luma_core::SourceManager;

use crate::{Diagnostic, DiagnosticContextKind, DiagnosticLevel};

pub struct Printer<'a> {
    sources: &'a SourceManager,
}

impl<'a> Printer<'a> {
    pub fn print(sources: &'a SourceManager, diagnostics: &[Diagnostic]) -> String {
        let printer = Self { sources };

        let mut reports = Vec::new();

        for diagnostic in diagnostics {
            reports.extend(printer.build_report(diagnostic));
        }

        Renderer::styled().render(&reports)
    }

    fn build_report<'report>(&'report self, diagnostic: &Diagnostic) -> Vec<Group<'report>> {
        let mut report = Vec::new();

        let primary_group = if let Some(root_span) = diagnostic.span
            && let Some(source) = self.sources.get_source(root_span.source_id)
        {
            let mut snippet = Snippet::source(&source.content)
                .path(source.source_file())
                .annotation(
                    AnnotationKind::Primary
                        .span(root_span.as_range())
                        .label(diagnostic.annotation.clone())
                        .highlight_source(true),
                );

            // iterate over additional contexts and add them to the report
            for ctx in diagnostic.additional_contexts.iter() {
                if let Some(span) = ctx.span {
                    snippet = snippet.annotation(
                        AnnotationKind::from(ctx.kind)
                            .span(span.as_range())
                            .label(ctx.annotation.clone()),
                    );
                };
            }

            Level::from(diagnostic.level)
                .primary_title(diagnostic.title.clone())
                .element(snippet)
        } else {
            let title = Level::from(diagnostic.level).primary_title(diagnostic.title.clone());

            if let Some(annotation) = &diagnostic.annotation {
                title.element(Level::NOTE.message(annotation.clone()))
            } else {
                title.element(Level::NOTE.message("no source provided"))
            }
        };

        report.push(primary_group);

        #[cfg(debug_assertions)]
        {
            report.push(
                Level::HELP
                    .with_name("thrower")
                    .secondary_title("")
                    .element(
                        Origin::path(diagnostic.thrower.file)
                            .line(diagnostic.thrower.line as usize)
                            .char_column(diagnostic.thrower.column as usize),
                    ),
            );
        }

        report
    }
}

impl<'a> From<DiagnosticLevel> for Level<'a> {
    fn from(level: DiagnosticLevel) -> Level<'a> {
        match level {
            DiagnosticLevel::Error => Level::ERROR,
            DiagnosticLevel::Warning => Level::WARNING,
            DiagnosticLevel::Note => Level::NOTE,
        }
    }
}

impl From<DiagnosticContextKind> for AnnotationKind {
    fn from(kind: DiagnosticContextKind) -> AnnotationKind {
        match kind {
            DiagnosticContextKind::Primary => AnnotationKind::Primary,
            DiagnosticContextKind::Context => AnnotationKind::Context,
            DiagnosticContextKind::Unannotated => AnnotationKind::Visible,
        }
    }
}

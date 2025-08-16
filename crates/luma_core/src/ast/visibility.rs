#[derive(crate::Display, Default, Debug, Clone, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum Visibility {
    Public, // pub
    #[default]
    Private, // pub(this) or nothing
    Inherit, // pub(inherit)
    Module, // pub(module)
}

impl Visibility {
    pub fn scoped(scope: &str) -> Option<Visibility> {
        Some(match scope {
            "inherit" => Visibility::Inherit,
            "module" => Visibility::Module,
            "" | "this" => Visibility::Private,
            _ => return None,
        })
    }
}

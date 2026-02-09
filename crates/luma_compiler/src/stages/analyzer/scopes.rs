use crate::ScopeId;

pub struct ScopeManager {
    scopes: Vec<Scope>,

    current: ScopeId,
}

pub struct Scope {
    pub parent: Option<ScopeId>,
}

impl ScopeManager {
    pub fn new() -> Self {
        ScopeManager {
            scopes: vec![Scope { parent: None }],
            current: 0,
        }
    }

    pub fn enter_scope(&mut self) -> ScopeId {
        self.scopes.push(Scope {
            parent: Some(self.current),
        });

        self.current = self.scopes.len() - 1;
        self.current
    }

    pub fn exit_scope(&mut self) -> ScopeId {
        self.current = self.scopes[self.current]
            .parent
            .expect("cannot exit global scope");
        self.current
    }

    pub fn current_scope(&self) -> usize {
        self.current
    }

    pub fn parent(&self, scope: ScopeId) -> Option<ScopeId> {
        self.scopes[scope].parent
    }
}

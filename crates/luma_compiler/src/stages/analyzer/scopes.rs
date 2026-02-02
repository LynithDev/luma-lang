pub type ScopeId = usize;

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

    pub fn enter_scope(&mut self) -> usize {
        self.scopes.push(Scope { 
            parent: Some(self.current)
        });

        self.current = self.scopes.len() - 1;
        self.current
    }

    pub fn exit_scope(&mut self) {
        self.current = self.scopes[self.current]
            .parent
            .expect("cannot exit global scope");
    }

    pub fn current_scope(&self) -> usize {
        self.current
    }

    pub fn parent(&self, scope: ScopeId) -> Option<ScopeId> {
        self.scopes[scope].parent
    }
}
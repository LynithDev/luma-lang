use std::{collections::HashMap, fmt::Debug};

use luma_core::{types::TypeKind, SymbolId};

pub trait Symbol {
    fn name(&self) -> &str;
    fn shadow_id(&self) -> Option<SymbolId>;

    fn new(id: SymbolId, name: String, shadow_id: Option<SymbolId>) -> Self where Self: Sized;
}

macro_rules! create_symbol {
    ($symbol_struct:ident { $($field:ident:$field_type:ty = $field_default:expr),* }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $symbol_struct {
            pub id: SymbolId,
            pub name: String,
            pub shadow_id: Option<SymbolId>,
            $(pub $field: $field_type),*
        }

        impl Symbol for $symbol_struct {
            fn name(&self) -> &str {
                &self.name
            }

            fn shadow_id(&self) -> Option<SymbolId> {
                self.shadow_id
            }

            fn new(id: SymbolId, name: String, shadow_id: Option<SymbolId>) -> Self {
                Self {
                    id,
                    name,
                    shadow_id,
                    $($field: $field_default),*
                }
            }
        }
    };
}

create_symbol! {
    ValueSymbol {
        ty: TypeKind = TypeKind::Unknown
    }
}

create_symbol!(TypeSymbol {});
create_symbol!(ControlFlowSymbol {});

#[derive(Debug)]
pub struct SymbolsTable {
    pub value_table: SymbolTable<ValueSymbol>,
    pub type_table: SymbolTable<TypeSymbol>,
    pub control_flow_table: SymbolTable<ControlFlowSymbol>,

    pub depth: u16,
}

#[derive(Debug)]
pub struct SymbolTable<T: Symbol> {
    symbols: Vec<T>,
    lookup_map: HashMap<String, SymbolId>,
    scope_stack: Vec<usize>,
}

impl SymbolsTable {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            control_flow_table: SymbolTable::with_capacity(0),
            type_table: SymbolTable::with_capacity(0),
            value_table: SymbolTable::new(),
            depth: 0,
        }
    }
    
    pub fn enter_scope(&mut self) {
        self.depth += 1;
        self.control_flow_table.enter_scope();
        self.type_table.enter_scope();
        self.value_table.enter_scope();
    }

    pub fn leave_scope(&mut self) {
        self.depth -= 1;
        self.control_flow_table.leave_scope();
        self.type_table.leave_scope();
        self.value_table.leave_scope();
    }


    pub fn declare_value(&mut self, name: String, ty: TypeKind) -> SymbolId {
        let symbol_id = self.value_table.declare(name);
        self.value_table.symbols[symbol_id].ty = ty;
        symbol_id
    }


    pub fn declare_type(&mut self, name: String) -> SymbolId {
        self.type_table.declare(name)
    }


    pub fn declare_control_flow(&mut self, name: String) -> SymbolId {
        self.control_flow_table.declare(name)
    }
}

impl<T: Symbol> SymbolTable<T> {
    pub(super) fn new() -> Self {
        Self {
            symbols: Vec::new(),
            lookup_map: HashMap::new(),
            scope_stack: Vec::new(),
        }
    }

    pub(super) fn with_capacity(capacity: usize) -> Self {
        Self {
            symbols: Vec::with_capacity(capacity),
            lookup_map: HashMap::with_capacity(capacity),
            scope_stack: Vec::new(),
        }
    }

    pub fn declare(&mut self, name: String) -> SymbolId {
        let symbol_id = self.symbols.len();

        let shadow_id = self.lookup_map.insert(name.clone(), symbol_id);
        let symbol = T::new(symbol_id, name.clone(), shadow_id);
        self.symbols.push(symbol);

        symbol_id
    }

    pub fn lookup_name(&self, name: &str) -> Option<&T> {
        let symbol_id = self.lookup_map.get(name)?;
        self.lookup_id(*symbol_id)
    }

    pub fn lookup_id(&self, id: SymbolId) -> Option<&T> {
        self.symbols.get(id)
    }

    pub fn lookup_id_mut(&mut self, id: SymbolId) -> Option<&mut T> {
        self.symbols.get_mut(id)
    }

    pub(super) fn enter_scope(&mut self) {
        self.scope_stack.push(self.symbols.len());
    }

    pub(super) fn leave_scope(&mut self) {
        let Some(scope_start) = self.scope_stack.pop() else {
            panic!("tried to leave scope when there is no scope");
        };
        
        let indexes = (scope_start..self.symbols.len()).rev();
        for index in indexes {
            let symbol = &self.symbols[index];

            if let Some(shadow_id) = symbol.shadow_id() {
                self.lookup_map.insert(symbol.name().to_owned(), shadow_id);
            } else {
                self.lookup_map.remove(symbol.name());
            }
        }
    }
}
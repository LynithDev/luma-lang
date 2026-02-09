use std::collections::HashMap;

use luma_diagnostic::{CompilerResult, error};

use crate::{SymbolId, TypeKind, stages::analyzer::AnalyzerError};

pub struct TypeCache {
    /// counter for generating type ids
    next_relative_id: RelativeTypeId,
    /// symbol id -> type cache entry
    map: HashMap<SymbolId, TypeCacheEntry>,
    /// resolved relative type id -> concrete type kind
    resolved: HashMap<RelativeTypeId, TypeKind>,
    /// relative type id chaining
    parents: HashMap<RelativeTypeId, RelativeTypeId>,
}

pub type RelativeTypeId = usize;

#[derive(Debug, Clone)]
pub enum TypeCacheEntry {
    Concrete(TypeKind),
    Relative(RelativeTypeId),
}

impl TypeCacheEntry {
    pub fn as_concrete(&self) -> Option<&TypeKind> {
        match self {
            TypeCacheEntry::Concrete(ty) => Some(ty),
            TypeCacheEntry::Relative(_) => None,
        }
    }
}

impl TypeCache {
    pub fn new() -> Self {
        TypeCache {
            map: HashMap::new(),
            next_relative_id: 0,
            resolved: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    pub fn fresh_relative(&mut self) -> RelativeTypeId {
        let id = self.next_relative_id;
        self.next_relative_id += 1;

        // at first, the relative type is its own parent because it isnt yet unified
        self.parents.insert(id, id);
        id
    }

    pub fn insert_concrete(&mut self, symbol: SymbolId, ty: TypeKind) {
        self.map.insert(symbol, TypeCacheEntry::Concrete(ty));
    }

    pub fn insert_relative(&mut self, symbol: SymbolId) -> RelativeTypeId {
        let id = self.fresh_relative();
        self.map.insert(symbol, TypeCacheEntry::Relative(id));
        id
    }

    pub fn find_relative(&mut self, rel: RelativeTypeId) -> Option<RelativeTypeId> {
        let parent = *self.parents.get(&rel)?;

        if parent != rel {
            let root = self.find_relative(parent)?;
            self.parents.insert(rel, root);
            Some(root)
        } else {
            Some(rel)
        }
    }

    pub fn unify(&mut self, a: &TypeCacheEntry, b: &TypeCacheEntry) -> CompilerResult<()> {
        match (a, b) {
            (TypeCacheEntry::Concrete(a_ty), TypeCacheEntry::Concrete(b_ty)) => {
                if a_ty == b_ty {
                    Ok(())
                } else {
                    Err(error!(AnalyzerError::TypeMismatch { 
                        expected: a_ty.clone(), found: b_ty.clone() 
                    }))
                }
            }
            (TypeCacheEntry::Relative(r1), TypeCacheEntry::Relative(r2)) => {
                let root1 = self.find_relative(*r1).ok_or(error!(AnalyzerError::TypeInferenceFailure))?;
                let root2 = self.find_relative(*r2).ok_or(error!(AnalyzerError::TypeInferenceFailure))?;
                if root1 != root2 {
                    self.parents.insert(root1, root2);
                }
                Ok(())
            }
            (TypeCacheEntry::Relative(r), TypeCacheEntry::Concrete(ty))
            | (TypeCacheEntry::Concrete(ty), TypeCacheEntry::Relative(r)) => {
                let root = self.find_relative(*r).ok_or(error!(AnalyzerError::TypeInferenceFailure))?;
                self.resolved.insert(root, ty.clone());
                Ok(())
            }
        }
    }

    pub fn resolve(&mut self, entry: &TypeCacheEntry) -> Option<TypeKind> {
        match entry {
            TypeCacheEntry::Concrete(ty) => Some(ty.clone()),
            TypeCacheEntry::Relative(r) => {
                let root = self.find_relative(*r)?;
                self.resolved.get(&root).cloned()
            }
        }
    }

    pub fn get(&self, symbol: SymbolId) -> Option<&TypeCacheEntry> {
        self.map.get(&symbol)
    }
}


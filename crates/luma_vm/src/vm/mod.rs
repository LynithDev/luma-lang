use luma_core::bytecode::IndexRef;

use crate::{runtime::{CallFrame, ChunkRef, RuntimeContext, RuntimeOptions}, ProgramSource, VmError, VmExitResult, VmResult};

use std::{fmt::Debug, hash::Hash, ops::{Deref, DerefMut}, rc::Rc};

mod exec;
mod alloc;
pub mod runtime;
pub mod value;
pub mod stack;
pub mod heap;
pub mod frames;

#[derive(Clone)]
pub struct VmHandle(Rc<LumaVM>); // todo: arc + rwlock

pub struct LumaVM {
    pub(crate) sources: Vec<ProgramSource>,
    pub(crate) ctx: RuntimeContext,
}

impl LumaVM {
    pub fn try_new(sources: Vec<ProgramSource>) -> VmResult<VmHandle> {
        Self::with_options(sources, RuntimeOptions::default())
    }

    pub fn with_options(sources: Vec<ProgramSource>, options: RuntimeOptions) -> VmResult<VmHandle> {
        if sources.is_empty() {
            return Err(VmError::NoEntrypoint);
        }

        Ok(VmHandle(Rc::new(Self {
            sources,
            ctx: RuntimeContext::new(options),
        })))
    }

    pub fn as_ptr(&self) -> *const LumaVM {
        self as *const LumaVM
    }

    pub fn handle(this: &Rc<Self>) -> VmHandle {
        VmHandle::new(Rc::clone(this))
    }

    pub fn run(&mut self) -> VmExitResult {
        self.init();

        if let Err(err) = self.exec() {
            return VmExitResult::from_error(err);
        }
        
        VmExitResult::from_code(0)
    }

    fn init(&mut self) {
        let call_frame = CallFrame {
            source_index: IndexRef::new(0), // entrypoint is first source
            chunk_ref: ChunkRef::TopLevel,
            instr_pointer: 0,
            base: 0,
        };

        let _ = self.ctx.frames.push(call_frame);
    }

    pub fn entrypoint(&self) -> &ProgramSource {
        &self.sources[0]
    }
}


impl Debug for LumaVM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LumaVM 0x{:x}", self.as_ptr() as usize)
    }
}

impl Hash for LumaVM {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}


impl PartialEq for VmHandle {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for VmHandle {}

impl Hash for VmHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}

impl Deref for VmHandle {
    type Target = LumaVM;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl DerefMut for VmHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl VmHandle {
    pub fn new(vm: Rc<LumaVM>) -> Self {
        Self(vm)
    }

    pub fn as_ptr(&self) -> *const LumaVM {
        self.0.as_ptr()
    }

    pub fn get(&self) -> &LumaVM {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut LumaVM {
        Rc::get_mut(&mut self.0).expect("Multiple handles exist; cannot get mutable reference")
    }
}


use crate::tree::{
    impls::show::{DocTree, Show},
    ID,
};

#[derive(Debug)]
pub enum DMacroKind {
    Declarative,
    ProcFunction,
    ProcAttribute,
    ProcDerive,
}
#[derive(Debug)]
pub struct DMacro {
    pub id: ID,
    pub kind: DMacroKind,
}
impl DMacro {
    pub fn new(id: ID, kind: DMacroKind) -> Self {
        Self { id, kind }
    }
}

impl Show for DMacro {
    fn show(&self) -> DocTree {
        let id = &self.id;
        match self.kind {
            DMacroKind::Declarative => format!("[decl] {id}"),
            DMacroKind::ProcFunction => format!("[func] {id}"),
            DMacroKind::ProcAttribute => format!("[attr] {id}"),
            DMacroKind::ProcDerive => format!("[derv] {id}"),
        }
        .show()
    }

    fn show_prettier(&self) -> DocTree {
        self.show()
    }
}

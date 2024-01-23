use super::ID;

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

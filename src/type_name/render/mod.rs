use rustdoc_types::Id;

use super::style::{StyledType, Tag};
use crate::{
    tree::IDMap,
    type_name::style::{Punctuation, Symbol},
    util::XString,
};
use std::{fmt, mem};

#[derive(Clone, Default)]
pub struct DeclarationLines {
    lines: Vec<DeclarationLine>,
}

impl fmt::Debug for DeclarationLines {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            _ = writeln!(f, "{line:?}");
        }
        Ok(())
    }
}

impl std::ops::Deref for DeclarationLines {
    type Target = [DeclarationLine];

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl From<&StyledType> for DeclarationLines {
    fn from(value: &StyledType) -> Self {
        Self::new_(value)
    }
}

impl DeclarationLines {
    pub fn new(id: &Id, map: &IDMap) -> Self {
        Self::new_(&StyledType::new(id, map))
    }

    fn new_(styled_type: &StyledType) -> Self {
        let tags = styled_type.tags();
        let mut lines = Vec::with_capacity(8);
        let mut line = Vec::with_capacity(8);
        let mut iter = tags.iter();
        let mut text_tag = TextTag::default();
        while let Some(tag) = iter.next() {
            match tag {
                Tag::Name(s) => text_tag.text.push_str(s),
                Tag::Decl(s) => text_tag.text.push_str(s.to_str()),
                Tag::Path(id) | Tag::PubScope(id) => {
                    line.push(text_tag.take());
                    text_tag.id = Some(*id);
                    let next = iter.next();
                    if let Some(Tag::Name(name)) = next {
                        text_tag.text = name.clone();
                    } else {
                        error!(?id, ?next, "name doesn't follow id");
                    }
                    line.push(text_tag.take());
                }
                Tag::Symbol(Symbol::Punctuation(Punctuation::NewLine)) => {
                    line.push(text_tag.take());
                    lines.push(DeclarationLine {
                        line: mem::take(&mut line),
                    });
                }
                Tag::Symbol(s) => text_tag.text.push_str(s.to_str()),
                Tag::UnusualAbi(s) => text_tag.text.push_str(s),
                _ => (),
            }
        }
        if !text_tag.text.is_empty() {
            line.push(text_tag);
        }
        // If the last text_tag carries id, we'll meet an empty text_tag
        // with the last line not pushed, thus check the last line and push it!
        if !line.is_empty() {
            lines.push(DeclarationLine { line });
        }
        let mut decl_lines = Self { lines };
        decl_lines.shrink_to_fit();
        decl_lines
    }

    fn shrink_to_fit(&mut self) {
        for line in &mut self.lines {
            line.line.shrink_to_fit();
        }
        self.lines.shrink_to_fit();
    }
}

#[derive(Clone, Default)]
pub struct DeclarationLine {
    line: Vec<TextTag>,
}

impl fmt::Debug for DeclarationLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for tt in &self.line {
            _ = write!(f, "{tt:?} ");
        }
        Ok(())
    }
}

impl std::ops::Deref for DeclarationLine {
    type Target = [TextTag];

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

#[derive(Clone, Default)]
pub struct TextTag {
    pub text: XString,
    pub id: Option<Id>,
}

impl fmt::Debug for TextTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TextTag { text, id } = self;
        if let Some(id) = id {
            _ = write!(f, "{text}#{id:?}#☺️");
        } else {
            _ = write!(f, "{text}☺️");
        }
        Ok(())
    }
}

impl TextTag {
    fn take(&mut self) -> Self {
        mem::take(self)
    }
}

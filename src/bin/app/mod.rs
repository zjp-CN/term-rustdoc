use crate::Result;
use ratatui::{
    style::{Color, Style},
    text::Line,
    widgets::{Block, Paragraph},
};
use rustdoc_types::Crate;
use term_rustdoc::tree::{DModule, IDMap, Show, TreeLines};

#[derive(Default)]
pub struct App {
    doc: Option<RustDoc>,
    pub should_quit: bool,
}

struct RustDoc {
    doc: Crate,
}

impl RustDoc {
    fn dmodule_idmap(&self) -> (DModule, IDMap<'_>) {
        (DModule::new(&self.doc), IDMap::from_crate(&self.doc))
    }
}

impl App {
    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_doc(&mut self) -> Result<TreeLines> {
        let doc =
            serde_json::from_reader(std::fs::File::open("target/deps/doc/integration.json")?)?;
        let doc = RustDoc { doc };
        let (dmod, map) = doc.dmodule_idmap();
        let outline = dmod.show_prettier(&map).into_treelines();
        self.doc = Some(doc);
        Ok(outline)
    }
}

pub fn cache_outline(doc: &str) -> (u16, Paragraph<'_>) {
    let lines = doc.lines().map(Line::from).collect::<Vec<_>>();
    (
        (lines.len() - 5) as u16,
        Paragraph::new(lines)
            .block(
                Block::default()
                    .title("DModule Tree for tokio crate")
                    .style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::Reset)),
    )
}

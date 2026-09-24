use std::path::PathBuf;

use texas_core::line_ending::LineEnding;

use crate::{
    command::{TexasCommand, TexasWorkbenchCommand},
    workspace::TexasWorkspace,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PaletteItem {
    pub content: PaletteItemContent,
    pub filter_text: String,
    pub score: u32,
    pub indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaletteItemContent {
    PaletteHelp {
        cmd: TexasWorkbenchCommand,
    },
    File {
        path: PathBuf,
        full_path: PathBuf,
    },
    Line {
        line: usize,
        content: String,
    },
    Command {
        cmd: TexasCommand,
    },
    Workspace {
        workspace: TexasWorkspace,
    },
    ColorTheme {
        name: String,
    },
    IconTheme {
        name: String,
    },
    Language {
        name: String,
    },
    LineEnding {
        kind: LineEnding,
    },
    SCMReference {
        name: String,
    },
    TerminalProfile {
        name: String,
        profile: texas_rpc::terminal::TerminalProfile,
    },
}

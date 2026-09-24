use strum_macros::EnumIter;

use crate::command::TexasWorkbenchCommand;

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum PaletteKind {
    PaletteHelp,
    File,
    Line,
    Command,
    Workspace,
    ColorTheme,
    IconTheme,
    Language,
    LineEnding,
    SCMReferences,
    TerminalProfile,
    DiffFiles,
    HelpAndFile,
}

impl PaletteKind {
    /// The symbol/prefix that is used to signify the behavior of the palette.
    pub fn symbol(&self) -> &'static str {
        match &self {
            PaletteKind::PaletteHelp => "?",
            PaletteKind::Line => "/",
            PaletteKind::Workspace => ">",
            PaletteKind::Command => ":",
            PaletteKind::TerminalProfile => "<",
            PaletteKind::File
            | PaletteKind::ColorTheme
            | PaletteKind::IconTheme
            | PaletteKind::Language
            | PaletteKind::LineEnding
            | PaletteKind::SCMReferences
            | PaletteKind::HelpAndFile
            | PaletteKind::DiffFiles => "",
        }
    }

    /// Extract the palette kind from the input string. This is most often a prefix.
    pub fn from_input(input: &str) -> PaletteKind {
        match input {
            _ if input.starts_with('?') => PaletteKind::PaletteHelp,
            _ if input.starts_with('/') => PaletteKind::Line,
            _ if input.starts_with('>') => PaletteKind::Workspace,
            _ if input.starts_with(':') => PaletteKind::Command,
            _ if input.starts_with('<') => PaletteKind::TerminalProfile,
            _ => PaletteKind::File,
        }
    }

    /// Get the [`TexasWorkbenchCommand`] that opens this palette kind, if one exists.
    pub fn command(self) -> Option<TexasWorkbenchCommand> {
        match self {
            PaletteKind::PaletteHelp => Some(TexasWorkbenchCommand::PaletteHelp),
            PaletteKind::Line => Some(TexasWorkbenchCommand::PaletteLine),
            PaletteKind::Workspace => Some(TexasWorkbenchCommand::PaletteWorkspace),
            PaletteKind::Command => Some(TexasWorkbenchCommand::PaletteCommand),
            PaletteKind::File => Some(TexasWorkbenchCommand::Palette),
            PaletteKind::HelpAndFile => {
                Some(TexasWorkbenchCommand::PaletteHelpAndFile)
            }
            PaletteKind::ColorTheme => Some(TexasWorkbenchCommand::ChangeColorTheme),
            PaletteKind::IconTheme => Some(TexasWorkbenchCommand::ChangeIconTheme),
            PaletteKind::Language => Some(TexasWorkbenchCommand::ChangeFileLanguage),
            PaletteKind::LineEnding => {
                Some(TexasWorkbenchCommand::ChangeFileLineEnding)
            }
            PaletteKind::SCMReferences => {
                Some(TexasWorkbenchCommand::PaletteSCMReferences)
            }
            PaletteKind::TerminalProfile => None, // InternalCommand::NewTerminal
            PaletteKind::DiffFiles => Some(TexasWorkbenchCommand::DiffFiles),
        }
    }

    pub fn get_input<'a>(&self, input: &'a str) -> &'a str {
        match self {
            PaletteKind::File
            | PaletteKind::ColorTheme
            | PaletteKind::IconTheme
            | PaletteKind::Language
            | PaletteKind::LineEnding
            | PaletteKind::SCMReferences
            | PaletteKind::HelpAndFile
            | PaletteKind::DiffFiles => input,
            PaletteKind::PaletteHelp
            | PaletteKind::Command
            | PaletteKind::Workspace
            | PaletteKind::Line
            | PaletteKind::TerminalProfile => input.get(1..).unwrap_or(""),
        }
    }

    /// Get the palette kind that it should be considered as based on the current
    /// [`PaletteKind`] and the current input.
    pub fn get_palette_kind(&self, input: &str) -> PaletteKind {
        if self == &PaletteKind::HelpAndFile && input.is_empty() {
            return *self;
        }

        if self != &PaletteKind::File
            && self != &PaletteKind::HelpAndFile
            && self.symbol() == ""
        {
            return *self;
        }

        PaletteKind::from_input(input)
    }
}

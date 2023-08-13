use owo_colors::Style;

pub struct Styling {
    pub location: Style,

    pub header_error: Style,
    pub header_warning: Style,
    pub header_note: Style,

    // Styles used in the error message:
    pub type_: Style,
    pub wrong_type: Style,
    pub trait_: Style,

    // Style used in the line display:
    pub line_wrong_part: Style,
    pub underline: bool,
}

impl Styling {
    pub fn no_color() -> Self {
        Styling {
            location: Style::new(),
            header_error: Style::new(),
            header_warning: Style::new(),
            header_note: Style::new(),
            type_: Style::new(),
            wrong_type: Style::new(),
            trait_: Style::new(),
            line_wrong_part: Style::new(),
            underline: true,
        }
    }

    pub fn colored() -> Self {
        Styling {
            location: Style::new().italic(),
            header_error: Style::new().red().bold(),
            header_warning: Style::new().yellow().bold(),
            header_note: Style::new().purple().bold(),
            type_: Style::new().green(),
            wrong_type: Style::new().red(),
            trait_: Style::new().blue(),
            line_wrong_part: Style::new().red(),
            underline: false,
        }
    }
}

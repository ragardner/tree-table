#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellAlign {
    #[default]
    General, // left for text, right for numbers

    // Top row
    TopLeft,
    TopCenter,
    TopRight,

    // Middle row
    MiddleLeft,
    MiddleCenter,
    MiddleRight,

    // Bottom row
    BottomLeft,
    BottomCenter,
    BottomRight,

    // Special
    Justify,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalAlign {
    General,
    Left,
    Center,
    Right,
    Justify,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
    Justify,
}

impl CellAlign {
    pub fn horizontal(&self) -> HorizontalAlign {
        match self {
            CellAlign::General
            | CellAlign::TopLeft
            | CellAlign::MiddleLeft
            | CellAlign::BottomLeft => HorizontalAlign::Left,
            CellAlign::TopCenter | CellAlign::MiddleCenter | CellAlign::BottomCenter => {
                HorizontalAlign::Center
            }
            CellAlign::TopRight | CellAlign::MiddleRight | CellAlign::BottomRight => {
                HorizontalAlign::Right
            }
            CellAlign::Justify => HorizontalAlign::Justify,
            CellAlign::Fill => HorizontalAlign::Fill,
        }
    }

    pub fn vertical(&self) -> VerticalAlign {
        match self {
            CellAlign::TopLeft | CellAlign::TopCenter | CellAlign::TopRight => VerticalAlign::Top,
            CellAlign::MiddleLeft | CellAlign::MiddleCenter | CellAlign::MiddleRight => {
                VerticalAlign::Middle
            }
            CellAlign::BottomLeft | CellAlign::BottomCenter | CellAlign::BottomRight => {
                VerticalAlign::Bottom
            }
            _ => VerticalAlign::Top,
        }
    }
}

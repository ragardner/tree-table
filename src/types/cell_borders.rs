use alloc::string::String;
use alloc::string::ToString;

use crate::BorderArgs;
use crate::Color3;
use crate::try_hex_to_color3;
use core::convert::TryFrom;

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    size: Option<u8>,
    bg: Option<Color3>,
}

#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(from_wasm_abi))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct CellBorders {
    pub top: Option<Border>,
    pub right: Option<Border>,
    pub bottom: Option<Border>,
    pub left: Option<Border>,
}

impl TryFrom<BorderArgs> for Border {
    type Error = String;

    fn try_from(args: BorderArgs) -> Result<Self, Self::Error> {
        Ok(Self {
            size: args.size,
            bg: try_hex_to_color3(args.bg)?,
        })
    }
}

impl From<Border> for BorderArgs {
    fn from(border: Border) -> Self {
        Self {
            size: border.size,
            bg: border.bg.map(|color| color.to_string()),
        }
    }
}

impl Border {
    /// Convert `Option<BorderArgs>` → `Option<Border>`
    pub fn try_from_opt(opt: Option<BorderArgs>) -> Result<Option<Self>, String> {
        match opt {
            Some(args) => Ok(Some(Self::try_from(args)?)),
            None => Ok(None),
        }
    }
}

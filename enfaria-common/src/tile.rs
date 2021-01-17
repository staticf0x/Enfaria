use crate::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromVariant, ToVariant)]
pub struct Tile {
    pub name: String,
    #[serde(default)]
    pub contains: Vec<Object>,
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> FmtResult {
        write!(fmt, "{}", self.name)
    }
}

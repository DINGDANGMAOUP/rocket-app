use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
#[derive(Clone)]
pub enum MenuType {
    Menu,
    Button,
}

impl Default for MenuType {
    fn default() -> Self {
        0.into()
    }
}

impl From<MenuType> for i32 {
    fn from(arg: MenuType) -> Self {
        match arg {
            MenuType::Menu => MenuType::Menu as i32,
            MenuType::Button => MenuType::Button as i32,
        }
    }
}

impl From<i32> for MenuType {
    fn from(arg: i32) -> Self {
        match arg {
            0 => MenuType::Menu,
            1 => MenuType::Button,
            _ => MenuType::Menu,
        }
    }
}

impl Debug for MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Menu => write!(f, "0"),
            Self::Button => write!(f, "1"),
        }
    }
}

impl Display for MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Menu => "0",
            Self::Button => "1",
        })
    }
}

impl Serialize for MenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        i32::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MenuType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = i32::deserialize(deserializer).map(|x| MenuType::from(x))?;
        Ok(s)
    }
}

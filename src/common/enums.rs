use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
#[derive(Clone)]
pub enum MenuType {
    Menu,
    Button,
}

impl Default for MenuType {
    fn default() -> Self {MenuType::Menu }
}

 
impl From<MenuType> for &str {
    fn from(arg: MenuType) -> Self {
        match arg {
            MenuType::Button => "Button",
            MenuType::Menu => "Menu",
        }
    }
}

impl From<&str> for MenuType {
    fn from(arg: &str) -> Self {
        match arg {
            "" => MenuType::Button,
            "Button" => MenuType::Button,
            "Menu" => MenuType::Menu,
            _ => MenuType::Button,
        }
    }
}

impl Debug for MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Menu => write!(f, "Menu"),
            Self::Button => write!(f, "Button"),
        }
    }
}

impl Display for  MenuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Menu => "Menu",
            Self::Button => "Button",
        })
        
    }
}

impl Serialize for MenuType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
            self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MenuType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let s = String::deserialize(deserializer)?;
            Ok(MenuType::from(s.as_str()))
    }
}
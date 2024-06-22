use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Name(pub String);

impl Name {
    pub fn from<T: ToString>(name: T) -> Self {
        Self(name.to_string())
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

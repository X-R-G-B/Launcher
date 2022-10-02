#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod people;

use crate::people::people::*;

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct FirstName(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

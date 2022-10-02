#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::prelude::*;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct FirstName(String);

pub fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()))
        .insert(FirstName("Proctor".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Hume".to_string()))
        .insert(FirstName("Hume".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(FirstName("Nieve".to_string()))
        .insert(Name("Zayna Nieves".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(FirstName("ABC".to_string()));
}

pub fn greet_people(query: Query<&FirstName, With<Name>>) {
    for firstname in query.iter() {
        println!("hello {}!", firstname.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people).add_system(greet_people);
    }
}

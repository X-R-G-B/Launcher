#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod people;

use futures_lite::future;

// use std::string;

// use crate::people::people::*;

use bevy::{prelude::*, winit::WinitSettings, tasks::{AsyncComputeTaskPool, Task},};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct ButtonDownload;

#[derive(Component)]
struct FirstName(String);

impl Plugin for ButtonDownload {
    fn build(&self, _app: &mut App) {

    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
use octocrab::{self, GitHubError};
use octocrab::models::repos::Release;
use octocrab::Error;
use bevy::tasks::TaskPool;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
// const GITHUB_PATH: &str = "https://github.com/X-R-G-B/Artena";

#[derive(Component)]
pub struct GithubReleaseDownloader;

#[derive(Component)]
pub struct GithubReleaseResult(Task<Result<Release, Error>>);

async fn get_latest() -> Result<Release, Error> {
    println!("Request API");
    let octo_inst = octocrab::instance();
    println!("Crash into call instance octocrab");
        // .await?;
        // .releases()
        // .get_latest()
    let repos = octo_inst.repos("X-R-G-B", "Artena");
    println!("Crash into call repos");
    let releases = repos.releases();
    println!("Crash into call releases");
    let latest = releases.get_latest();
    println!("Crash into call latest");
    let await_var = latest.await?;
    println!("Crash into call await");
    println!("Pas de crash OK!");
    return Ok(await_var);
}

fn button_system(
    mut interaction_query: Query<'_, '_, 
    (&Interaction, &mut UiColor, &Children),
    (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<'_, '_, &mut Text>,
    mut commands: Commands,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                commands.spawn().insert(GithubReleaseDownloader);
                println!("salut");
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn downloader_system_spawn(mut commands: Commands, query: Query<&GithubReleaseDownloader>) {
    println!("Entering download");
    let thread_pool = TaskPool::new();
    println!("Panick at thread pool : NO");
    if query.into_iter().len() >= 1 {
        let task = thread_pool.spawn(async {
            get_latest().await
        });
        println!("Panick at task : NO");
        commands.spawn().insert(GithubReleaseResult(task));
        println!("Panick at commands : NO");
    }
    println!("Everything OK !");
}

fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut GithubReleaseResult)>,
    ) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(release) = future::block_on(future::poll_once(&mut task.0)) {
            if release.is_ok() {
                println!("OKKKK");
            }
            commands.entity(entity).remove::<GithubReleaseResult>();
        }
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
            ));
        });
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////



// #[tokio::main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_startup_system(setup)
        .add_system(downloader_system_spawn)
        .add_system(button_system)
        .add_system(handle_tasks)
        .run();
}

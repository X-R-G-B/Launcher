pub mod const_file;

use const_file::*;

use bevy::{prelude::*, tasks::Task, winit::WinitSettings};

use octocrab::{models::repos::Release, Error};

struct ButtonDownload;

#[derive(Component, Debug)]
pub struct GithubReleaseDownloader;

#[derive(Component, Debug)]
pub struct GithubReleaseResult(Task<Result<Release, Error>>);

impl Plugin for ButtonDownload {
    fn build(&self, _app: &mut App) {}
}

async fn get_latest() -> Result<Release, Error> {
    dbg!("Request API : ");
    let octo_inst = octocrab::instance();
    let releases = octo_inst
        .repos("X-R-G-B", "Artena")
        .releases()
        .get_latest()
        .await?;
    dbg!(&releases);
    Ok(releases)
}

fn button_system(
    mut interaction_query: Query<
        '_,
        '_,
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<'_, '_, &mut Text>,
    handle: ResMut<tokio::runtime::Handle>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = match text_query.get_mut(children[0]) {
            Ok(text) => {
                text
            },
            Err(_) => {
                return;
            },
        };
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = const_file::PRESSED_BUTTON.into();
                handle.spawn(async move { get_latest().await });
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
                    font: asset_server.load(TEXT_BUTTON_FONT),
                    font_size: 40.0,
                    color: TEXT_BUTTON_COLOR,
                },
            ));
        });
}

#[tokio::main]
async fn main() {
    let handle = tokio::runtime::Handle::current();
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(handle)
        .add_startup_system(setup)
        .add_system(button_system)
        .run();
}

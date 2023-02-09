use bevy::prelude::*;
use tower_defence::resources::grid::{Grid, GridSize};
//use tower_defence::gui::main_menu::setup_main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_startup_system(setup_main_menu)
        .add_startup_system(setup)
        .insert_resource(Grid::new(GridSize {
            width: 27,
            height: 15,
        }))
        .add_startup_system(debug_grid)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera so we can look at things, even GUI????
    commands.spawn(Camera2dBundle::default());
}

fn debug_grid(mut commands: Commands, grid: Res<Grid>) {
    let width = grid.get_width();
    let height = grid.get_height();

    let tile_width = 100.0 / width as f32;
    let tile_height = 100.0 / height as f32;

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                },
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|parent| {
            (0..width).into_iter().for_each(|_| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            size: Size {
                                width: Val::Percent(tile_width - 0.2),
                                height: Val::Percent(100.0),
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        (0..height).into_iter().for_each(|_| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(tile_height - 0.4),
                                    },
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            });
                        })
                    });
            });
        });
}

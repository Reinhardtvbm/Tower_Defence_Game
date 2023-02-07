use bevy::prelude::*;

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                },
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(40.0),
                            height: Val::Percent(80.0),
                        },
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(80.0),
                                height: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    });

                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(80.0),
                                height: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: Color::RED.into(),
                        ..default()
                    });

                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(80.0),
                                height: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: Color::BLUE.into(),
                        ..default()
                    });

                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size {
                                width: Val::Percent(80.0),
                                height: Val::Percent(10.0),
                            },
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    });
                });
        });
}

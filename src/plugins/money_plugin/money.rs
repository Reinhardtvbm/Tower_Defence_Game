use bevy::{prelude::*, render::view::window};

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Money(100.0)).add_startup_system(display_money);
    }
}

#[derive (Resource)]
pub struct Money(f32);

fn display_money(mut commands: Commands, ass_serve: Res<AssetServer>, money: Res<Money>, windows: Res<Windows>) {
    let font = ass_serve.load("font/OpenSans-VariableFont_wdth,wght.ttf");
    let text_style = TextStyle {
        font,
        font_size: 60.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment::TOP_LEFT;

    let window = windows.get_primary().unwrap();
    let x_pos = 10.0 - (window.width() / 2.0);
    let y_pos = -10.0 + (window.height() / 2.0);

    commands.spawn(
        Text2dBundle {
            text: Text::from_section(format!("{}",money.0), text_style.clone())
                .with_alignment(text_alignment),
                transform: Transform {
                    translation: Vec3 {
                        x: x_pos,
                        y: y_pos,
                        ..default()
                    },
                    ..default()
                },
            ..default()
        },
    );
}





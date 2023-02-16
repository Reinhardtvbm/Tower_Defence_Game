use bevy::prelude::*;


pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Money::new(70.0)).add_system(display_money);
    }
}

#[derive (Resource)]
pub struct Money {
    amount: f32,
    entity: Option<Entity>
}

impl Money {
    pub fn new(amount: f32) -> Self {
        Money { amount , entity: None}
    }

    pub fn spend_money(&mut self, amount: f32) {
        self.amount -= amount
    }

    pub fn entity(&self) -> Option<Entity> {
        self.entity
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn set_entity_none(&mut self) {
        self.entity = None;
    }

    pub fn amount(&self) -> f32{
        self.amount
    }
}

fn display_money(mut commands: Commands, ass_serve: Res<AssetServer>, mut money: ResMut<Money>, windows: Res<Windows>) {
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

    if let Some(entity) = money.entity() {
        commands.entity(entity).despawn();
        money.set_entity_none();
    }

    let entity = commands.spawn(
        Text2dBundle {
            text: Text::from_section(format!("{}",money.amount), text_style.clone())
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
    ).id();

    money.set_entity(entity)
}






use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update_dot_position)
        .run();
}

// Define a component to mark entities as dots
#[derive(Component)]
struct Dot;

// Set up the initial scene
fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a white dot
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            ..default()
        },
        Dot, // Add the Dot component to mark this entity as a dot
    ));
}

// Update the position of the dot to make it move in a clockwise circle
fn update_dot_position(time: Res<Time>, mut query: Query<&mut Transform, With<Dot>>) {
    for mut transform in &mut query {
        // Calculate the angle based on elapsed time
        let angle = time.elapsed_seconds() * 2.0 * PI;
        let radius = 100.0;

        // Update the dot's position
        // Use negative sine for y to make it move clockwise
        transform.translation.x = radius * angle.cos();
        transform.translation.y = -radius * angle.sin(); // Negative sign here for clockwise motion
    }
}

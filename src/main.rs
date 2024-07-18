use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const RED_SRGB: bevy::prelude::Color = Color::srgb(1., 0., 0.);
const BLUE_SRGB: bevy::prelude::Color = Color::srgb(0., 0., 1.);
const GREEN_SRGB: bevy::prelude::Color = Color::srgb(0., 1., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_objects, check_collision))
        .run();
}

#[derive(Component)]
struct MovingObject {
    center: Vec2,
    radius: f32,
    angle: f32,
    speed: f32,
}

#[derive(Component)]
struct Collider {
    radius: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Spawn two circular objects
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(20.0)).into(),
            material: materials.add(ColorMaterial::from(RED_SRGB)),
            transform: Transform::from_translation(Vec3::new(-100.0, 0.0, 0.0)),
            ..default()
        },
        MovingObject {
            center: Vec2::new(-100.0, 0.0),
            radius: 100.0,
            angle: 0.0,
            speed: 1.0,
        },
        Collider { radius: 20.0 },
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(20.0)).into(),
            material: materials.add(ColorMaterial::from(BLUE_SRGB)),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
            ..default()
        },
        MovingObject {
            center: Vec2::new(100.0, 0.0),
            radius: 100.0,
            angle: std::f32::consts::PI,
            speed: 1.5,
        },
        Collider { radius: 20.0 },
    ));
}

fn move_objects(mut query: Query<(&mut Transform, &mut MovingObject)>, time: Res<Time>) {
    for (mut transform, mut moving_object) in query.iter_mut() {
        moving_object.angle += moving_object.speed * time.delta_seconds();
        let new_position = Vec2::new(
            moving_object.center.x + moving_object.radius * moving_object.angle.cos(),
            moving_object.center.y + moving_object.radius * moving_object.angle.sin(),
        );
        transform.translation = new_position.extend(0.0);
    }
}

fn check_collision(
    mut query: Query<(&Transform, &Collider, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut iter = query.iter_mut();
    if let Some((transform1, collider1, material1)) = iter.next() {
        if let Some((transform2, collider2, material2)) = iter.next() {
            let distance = transform1.translation.distance(transform2.translation);
            let collision = distance < (collider1.radius + collider2.radius);

            let (color1, color2) = if collision {
                (GREEN_SRGB, GREEN_SRGB)
            } else {
                (RED_SRGB, BLUE_SRGB)
            };

            *materials.get_mut(material1.id()).unwrap() = ColorMaterial::from(color1);
            *materials.get_mut(material2.id()).unwrap() = ColorMaterial::from(color2);
        }
    }
}

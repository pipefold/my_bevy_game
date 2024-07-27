use bevy::prelude::*;

const RED_SRGB: Color = Color::srgb(1.0, 0.0, 0.0);
const BLUE_SRGB: Color = Color::srgb(0.0, 0.0, 1.0);
const GREEN_SRGB: Color = Color::srgb(0.0, 1.0, 0.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_objects, check_collision))
        .run();
}

#[derive(Component)]
struct MovingObject {
    center: Vec3,
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
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Spawn two spherical objects
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.5 })),
            material: materials.add(StandardMaterial::from(RED_SRGB)),
            transform: Transform::from_translation(Vec3::new(-2.0, 0.0, 0.0)),
            ..default()
        },
        MovingObject {
            center: Vec3::new(-2.0, 0.0, 0.0),
            radius: 2.0,
            angle: 0.0,
            speed: 1.0,
        },
        Collider { radius: 0.5 },
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: 0.5 })),
            material: materials.add(StandardMaterial::from(BLUE_SRGB)),
            transform: Transform::from_translation(Vec3::new(2.0, 0.0, 0.0)),
            ..default()
        },
        MovingObject {
            center: Vec3::new(2.0, 0.0, 0.0),
            radius: 2.0,
            angle: std::f32::consts::PI,
            speed: 1.5,
        },
        Collider { radius: 0.5 },
    ));
}

fn move_objects(mut query: Query<(&mut Transform, &mut MovingObject)>, time: Res<Time>) {
    for (mut transform, mut moving_object) in query.iter_mut() {
        moving_object.angle += moving_object.speed * time.delta_seconds();
        let new_position = Vec3::new(
            moving_object.center.x + moving_object.radius * moving_object.angle.cos(),
            moving_object.center.y,
            moving_object.center.z + moving_object.radius * moving_object.angle.sin(),
        );
        transform.translation = new_position;
    }
}

fn check_collision(
    mut query: Query<(&Transform, &Collider, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

            *materials.get_mut(material1.id()).unwrap() = StandardMaterial::from(color1);
            *materials.get_mut(material2.id()).unwrap() = StandardMaterial::from(color2);
        }
    }
}

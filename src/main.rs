use bevy::prelude::*;
use rand::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_cubes)
        .add_system(gravity)
        .run();
}

#[derive(Component)]
struct Velocity(Vec3);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::from([1.0, 1.0, 1.0]);
    ambient_light.brightness = 1.0;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(300.0, 300.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    for _i in 0..4000 {
        let mut rng = thread_rng();
        let x: f32 = rng.gen_range(-100.0..100.0);
        let mut y: f32 = rng.gen_range(-100.0..100.0);
        let z: f32 = rng.gen_range(-100.0..100.0);
        if x*x + y*y + z*z > 10000.0 {
            continue;
        }
        y = y / 20.0;

        let pos = Vec3::new(x, y, z);

        let mut vel = Vec3::new(0.0, 0.0, 0.0);
        vel.x = -pos.z / 1.1;
        vel.z = pos.x / 1.1;

        vel = vel.normalize() * vel.length() / 15.0;

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { ..Default::default() })),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(pos),
                ..Default::default()
            })
            .insert(Velocity(vel));
    }
}

fn gravity(mut query: Query<(&mut Velocity, &Transform)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(mut velocity1, transform1), (mut velocity2, transform2)]) = iter.fetch_next() {
        let delta = transform1.translation - transform2.translation;
        if delta.length() < 2.0 {
            continue;
        }
        let normalised_delta = delta.normalize();
        let force = 1.0 / (delta.length() * delta.length());
        velocity1.0 -= normalised_delta * force * 1.0;
        velocity2.0 += normalised_delta * force * 1.0;
    }
}

fn update_cubes(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}



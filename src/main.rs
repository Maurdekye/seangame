use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
    transform,
};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const PLAYER_ACCELERATION: f32 = 0.2;
const PLAYER_PEAK_SPEED: f32 = 40000.0;
const PLAYER_DRAG: f32 = 0.99;
const GRAVITY_CONSTANT: f32 = 2000.0;
const JUMP_POWER: f32 = 1000.0;

const LEFT_WALL: f32 = -550.0;
const RIGHT_WALL: f32 = 550.0;
const FLOOR: f32 = -300.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Gravity;

#[derive(Component)]
struct GroundDetection {
    on_ground: bool,
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
        Velocity(Vec2::new(0.0, 0.0)),
        Gravity,
        GroundDetection { on_ground: false },
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &mut GroundDetection), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, mut on_ground) = query.single_mut();
    let direction = if keyboard_input.pressed(KeyCode::Left) {
        -1.0
    } else if keyboard_input.pressed(KeyCode::Right) {
        1.0
    } else {
        0.0
    };

    let delta_speed = direction * time.delta_seconds() * PLAYER_PEAK_SPEED - velocity.x;
    velocity.x += delta_speed * PLAYER_ACCELERATION;
    velocity.x *= PLAYER_DRAG;

    println!("{:?}", velocity.x / time.delta_seconds());

    if keyboard_input.pressed(KeyCode::Up) && on_ground.on_ground {
        velocity.y += JUMP_POWER;
        on_ground.on_ground = false;
    }
}

fn apply_gravity(mut query: Query<(&Transform, &mut Velocity, &Gravity)>, time: Res<Time>) {
    for (_, mut velocity, _) in &mut query {
        velocity.y -= time.delta_seconds() * GRAVITY_CONSTANT;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn apply_border_collision(
    mut query: Query<(&mut Transform, &mut Velocity, &mut GroundDetection)>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity, mut on_ground) in &mut query {
        if transform.translation.y - transform.scale.y / 2.0 < FLOOR {
            velocity.y = velocity.y.max(0.0);
            transform.translation.y = FLOOR + transform.scale.y / 2.0;
            on_ground.on_ground = true;
        } else {
            on_ground.on_ground = false;
        }
        if transform.translation.x - transform.scale.x / 2.0 < LEFT_WALL {
            velocity.x = velocity.x.min(0.0);
            transform.translation.x = LEFT_WALL + transform.scale.x / 2.0;
        }
        if transform.translation.x + transform.scale.x / 2.0 > RIGHT_WALL {
            velocity.x = velocity.x.max(0.0);
            transform.translation.x = RIGHT_WALL - transform.scale.x / 2.0;
        }
    }
}

fn print_player_pos(query: Query<&Transform, &Player>) {
    println!("{:?}", query.single().translation);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                apply_gravity,
                apply_velocity,
                apply_border_collision,
                // print_player_pos,
            )
                .chain(),
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

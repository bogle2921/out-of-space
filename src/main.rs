use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 70.;
pub const ENEMY_SPEED: f32 = 70.;
pub const SPRITE_SIZE: f32 = 64.0;
pub const ENEMY_SIZE: f32 = 108.;
pub const NUM_ENEMIES: usize = 4;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;
pub const METEOR_SPAWN_TIME: f32 = 10.0;
pub const METEOR_OFFSET: f32 = 20.0;
pub const METEOR_SPEED: f32 = 200.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<EnemySpawnTimer>()
    .init_resource::<MeteorSpawnTimer>()
    .add_systems(Startup, (setup_bg, spawn_camera, spawn_player, setup_enemies))
    .add_systems(Update, (player_movement, confine_movement, enemy_movement, update_enemy_direction))
    .add_systems(Update, (enemy_hit_player, meteor_collision))
    .add_systems(Update, (timer_countdown, spawn_enemies))
    .add_systems(Update, (meteor_countdown, spawn_meteor, meteor_movement))
    .run();
}

pub fn setup_bg(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<AssetServer>,
){
    let window = query_window.get_single().unwrap();
    commands.spawn(
        SpriteBundle {
            texture: assets.load("sprites/space-background.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -1.0),
            ..default()
            }
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = query_window.get_single().unwrap();
    
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

#[derive(Component)]
pub struct Player {
    velocity: Vec3
}

#[derive(Component)]
pub struct Enemy{
    pub direction: Vec2
}

#[derive(Component)]
pub struct Meteor{
    pub direction: Vec2
}

#[derive(Resource)]
pub struct MeteorSpawnTimer {
    pub timer: Timer
}

impl Default for MeteorSpawnTimer {
    fn default() -> MeteorSpawnTimer {
        MeteorSpawnTimer {timer: Timer::from_seconds(METEOR_SPAWN_TIME, TimerMode::Repeating)}
    }
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating)}
    }
}

pub fn spawn_player(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = query_window.get_single().unwrap();

    commands.spawn(
        (SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/player.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            ..default()
        },
        Player {
            velocity: Vec3::ZERO,
        },
    ));
}

pub fn setup_enemies(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = query_window.get_single().unwrap();

    for _ in 0..NUM_ENEMIES {
        let rng_x = random::<f32>() * window.width();
        let rng_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rng_x, rng_y, 0.0),
                texture: asset_server.load("sprites/midLevel_enemy.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
            },
        ));
    }
}

pub fn player_movement(
    key_input: Res<Input<KeyCode>>,
    mut query_player: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
){
    if let Ok((mut player, mut transform)) = query_player.get_single_mut() {
        let mut current = player.velocity;

        if key_input.just_pressed(KeyCode::Left) || key_input.just_pressed(KeyCode::A) {
            current.x += -1.0;
        } else {
            current.x += 0.0;
        }

        if key_input.just_pressed(KeyCode::Right) || key_input.just_pressed(KeyCode::D) {
            current.x += 1.0;
        } else {
            current.x += 0.0;
        }

        if key_input.just_pressed(KeyCode::Up) || key_input.just_pressed(KeyCode::W) {
            current.y += 1.0;
        } else {
            current.y += 0.0;
        }

        if key_input.just_pressed(KeyCode::Down) || key_input.just_pressed(KeyCode::S) {
            current.y += -1.0;
        } else {
            current.y += 0.0;
        }

        if current.x >= PLAYER_SPEED * time.delta_seconds() {
            current.x = PLAYER_SPEED * time.delta_seconds();
        }

        if current.y >= PLAYER_SPEED * time.delta_seconds() {
            current.y = PLAYER_SPEED * time.delta_seconds();
        }

        if current.length() > 0.0 {
            current = current.normalize();
        }

        player.velocity = current * PLAYER_SPEED * time.delta_seconds();
        transform.translation += player.velocity;
    }
}

pub fn confine_movement(
    mut query_player: Query<&mut Transform, With<Player>>,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = query_player.get_single_mut() {
        let window = query_window.get_single().unwrap();

        let min_x = 0.0 + SPRITE_SIZE / 2.0;
        let max_x = window.width() - SPRITE_SIZE / 2.0;
        let min_y = 0.0 + SPRITE_SIZE / 2.0;
        let max_y = window.height() - SPRITE_SIZE / 2.0;

        let mut translation = transform.translation;
        if translation.x < min_x {
            translation.x = min_x;
        } else if translation.x > max_x {
            translation.x = max_x;
        }

        if translation.y < min_y {
            translation.y = min_y;
        } else if translation.y > max_y {
            translation.y = max_y;
        }

        transform.translation = translation;
    }
}

pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let dir = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += dir * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    query_window: Query<&Window, With<PrimaryWindow>>
) {
    let window = query_window.get_single().unwrap();

    let min_x = 0.0 + ENEMY_SIZE / 2.0;
    let max_x = window.width() - ENEMY_SIZE / 2.0;
    let min_y = 0.0 + ENEMY_SIZE / 2.0;
    let max_y = window.height() - ENEMY_SIZE / 2.0;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < min_x || translation.x > max_x {
            enemy.direction.x *= -1.0;
        }

        if translation.y < min_y || translation.y > max_y {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut(){
        for enemy_transform in enemy_query.iter(){
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < SPRITE_SIZE / 2.0 + ENEMY_SIZE / 2.0 {
                println!("Game Over!");
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<EnemySpawnTimer>,
) {
    if spawn_timer.timer.finished() {
        let window = query_window.get_single().unwrap();

        let rng_x = random::<f32>() * window.width();
        let rng_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rng_x, rng_y, 0.0),
                texture: asset_server.load("sprites/midLevel_enemy.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()
            },
        ));
    }
}

pub fn timer_countdown(
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
}

pub fn spawn_meteor(
    spawn_timer: Res<MeteorSpawnTimer>,
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if spawn_timer.timer.finished() {
        let window = query_window.get_single().unwrap();

        let rng_y = random::<f32>() * METEOR_OFFSET + window.height();
        let rng_x = random::<f32>() * window.width();

        commands.spawn(
            (SpriteBundle {
                transform: Transform::from_xyz(rng_x, rng_y, 0.0),
                texture: asset_server.load("sprites/meteor.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                    ..default()
                },
                ..default()
            },
            Meteor {
                direction: Vec2::NEG_Y,
            },
        ));
    }
}

pub fn meteor_movement(
    mut meter_query: Query<(&mut Transform, &Meteor)>,
    time: Res<Time>
) {
    for (mut transform, meteor) in meter_query.iter_mut() {
        let dir = Vec3::new(0.0, meteor.direction.y, 0.0);
        transform.translation += dir * METEOR_SPEED * time.delta_seconds();
    }
}

pub fn meteor_countdown(
    mut spawn_timer: ResMut<MeteorSpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer.timer.tick(time.delta());
}

pub fn meteor_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    meteor_query: Query<&Transform, With<Meteor>>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut(){
        if let Ok(meteor) = meteor_query.get_single() {
            let distance = player_transform.translation.distance(meteor.translation);
            if distance < SPRITE_SIZE {
                println!("Game Over!");
                commands.entity(player_entity).despawn();
            }
            for (enemy_entity, enemy_transform) in enemy_query.iter_mut(){
                let distance = enemy_transform.translation.distance(meteor.translation);
                if distance < SPRITE_SIZE / 2.0 + ENEMY_SIZE / 2.0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }
}


#![allow(unused)]

mod chunk;
mod circuit_states;
mod material;

use crate::{chunk::ComponentChunkBuilder, circuit_states::CircuitStates};
use bevy::prelude::*;
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_flycam::{FlyCam, PlayerPlugin};
use material::ChunkMaterial;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Immediate,
            width: 640.0,
            height: 480.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(OverlayPlugin {
            font_size: 16.0,
            ..default()
        })
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(material::ChunkMaterialPlugin)
        .add_startup_system(setup)
        .add_system(move_cube)
        .add_system(print_position)
        .run();
}

#[derive(Component)]
struct MovingCube {
    data: f32,
}

fn move_cube(mut query: Query<(Entity, &mut Transform, &mut MovingCube)>) {

    // for (mut transform, mut cube) in query.iter_mut() {
    //     *transform = transform.with_rotation(Quat::from_rotation_x(cube.data.sin()));
    //     cube.data = cube.data + 0.1;
    // }
}

fn print_position(mut query: Query<&Transform, With<FlyCam>>) {
    screen_print!("{:?}", query.single().translation);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //         ..default()
    //     })
    //     .insert(MovingCube { data: 0.0 });
    commands.spawn_bundle(UiCameraBundle::default());
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_scaled_axis(Vec3::new(
                -std::f32::consts::FRAC_PI_2,
                -std::f32::consts::FRAC_PI_4,
                0.0,
            )),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });
    let mut chunk_builder = ComponentChunkBuilder::new(CircuitStates { vec: vec![1] });
    chunk_builder.rectangle(
        [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ],
        Color::WHITE,
        [Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)],
        Vec3::new(0.0, 1.0, 0.0),
    );

    commands.spawn_bundle(chunk_builder.build(
        &mut meshes,
        &mut materials,
        &mut images,
        Vec3::new(0.0, 0.0, 0.0),
    ));
    // commands.spawn_bundle(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         // Configure the projection to better fit the scene
    //         shadow_projection: OrthographicProjection {
    //             left: -HALF_SIZE,
    //             right: HALF_SIZE,
    //             bottom: -HALF_SIZE,
    //             top: HALF_SIZE,
    //             near: -10.0 * HALF_SIZE,
    //             far: 10.0 * HALF_SIZE,
    //             ..default()
    //         },
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_scaled_axis(Vec3::new(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4, 0.0)),
    //         ..default()
    //     },
    //     ..default()
    // });
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

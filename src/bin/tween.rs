use bevy::DefaultPlugins;
use bevy::app::App;
use bevy::asset::{Assets, Handle};

use bevy::math::{Quat, Vec2};
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tween::DefaultTweenPlugins;
use bevy_tween::prelude::*;
use bevy_tween::tween::AnimationTarget;
use card_test::camera_controller::{CameraController, CameraControllerPlugin};
use card_test::cards::EventWithData::BackTo;
use card_test::cards::{Card, gen_put_card};
use card_test::cases::{CaseImages, CasePlane, render_case};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            // CameraControllerPlugin,
            // 动画相关
            DefaultTweenPlugins,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, deal_tween_event)
        .run();
}

#[derive(Component)]
pub struct CardPlane;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all ^= true;
    // 自由相机来测试Ω
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 25.).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController::default(),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));

    // 场地相关的 素材
    let case_images: CaseImages = CaseImages {
        stone1: asset_server.load("stone_1.png"),
        stone2: asset_server.load("stone_2.png"),
        safe: asset_server.load("safe.png"),
        lx: asset_server.load("lx.png"),
        jq: asset_server.load("jq.png"),
    };

    render_case(&mut commands, &mut meshes, &mut materials, case_images);

    // 设置两个用来触发的 平面 用来计算当前鼠标的位置
    let card_plane =
        Transform::from_xyz(0.0, 0.0, 18.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));
    let case_plane =
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.0));

    commands.spawn((CardPlane, card_plane));
    commands.spawn((CasePlane, case_plane));

    // 卡片放置器 放置在查看面上
    let mut card_fn = gen_put_card::<CardPlane>(
        &mut commands,
        &mut materials,
        &mut meshes,
        3. / 1.4,
        3.,
        0.05,
        0.01,
    );
    let yellow = asset_server.load("NAAI-A-001.png");
    card_fn(
        yellow.clone(),
        Transform::from_xyz(0., -4., card_plane.translation.z),
    );
}

// 测试移动效果
pub fn change_trans(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    card: Query<(Entity, &Transform), With<Card>>,
) {
    let end = Vec3::new(7., 7., 0.1);
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        card.iter().for_each(|(entity, transform)| {
            let card = AnimationTarget.into_target();
            let mut start = card.transform_state(transform.clone());
            commands.entity(entity).animation().insert_tween_here(
                Duration::from_secs_f32(2.),
                EaseKind::ExponentialOut,
                start.translation_to(end),
            );
        })
    }
}

pub fn deal_tween_event(mut commands: Commands, mut event: EventReader<TweenEvent<&'static str>>) {
    event.read().for_each(|event| match event.data {
        "back" => {
            info!("{:?}", event.entity);
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(Transform::from_translation(Vec3::ZERO));

            commands.entity(event.entity).animation().insert_tween_here(
                Duration::from_secs_f32(2.),
                EaseKind::ExponentialOut,
                start.translation_to(Vec3::ZERO),
            );
        }
        _ => {}
    });
}

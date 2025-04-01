use crate::cards::deal_on_drop;
use bevy::prelude::*;
use std::f32::consts::PI;

// 场地图片配置
#[derive(Resource)]
pub struct CaseImages {
    pub stone1: Handle<Image>,
    pub stone2: Handle<Image>,
    pub safe: Handle<Image>,
    pub lx: Handle<Image>,
    pub jq: Handle<Image>,
}

#[derive(Component)]
pub struct CasePlane;
#[derive(Component)]
pub struct CaseBase;

#[derive(Debug, Clone, Copy)]
pub enum CaseZoneType {
    Nothing,
    Battle,
    Prepare,
    Safe,
    Lx,
    JQ,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CaseZone {
    zone_type: CaseZoneType,
    transform: Transform,
    num: u32,
}

pub fn render_case(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    case_images: CaseImages,
) {
    // 生成一组场地的mesh
    let mesh_to_render = case_mesh(materials, case_images, 4.0, 1.2);
    // 一个需要的墙 卡片都在这个墙上
    commands
        .spawn((
            CaseBase,
            Transform::from_xyz(0., 0., 0.), // .with_rotation(Quat::from_axis_angle(Vec3::X, PI / 2.))
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (mesh, tr, mal, c) in mesh_to_render {
                parent
                    .spawn((
                        c,
                        Mesh3d(meshes.add(mesh)),
                        tr.clone(),
                        MeshMaterial3d(mal.clone()),
                    ))
                    .observe(deal_on_drop);
            }
        });
}

pub fn case_mesh(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    case_images: CaseImages,
    a: f32,
    mid: f32,
) -> [(
    Rectangle,
    Transform,
    Handle<StandardMaterial>,
    impl Component,
); 21] {
    // 战场
    let half_a = a * 0.5;
    let half_mid = mid * 0.5;
    let mid_point = Transform::default();

    let res_mesh = [
        // 中心方块
        (
            Rectangle::from_size(Vec2::new(a * 6.0, mid)),
            mid_point.clone(),
            materials.add(Color::BLACK),
            CaseZone {
                zone_type: CaseZoneType::Nothing,
                transform: mid_point.clone(),
                num: 0,
            },
        ),
        // 上方预备区
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-3.0 * half_a, half_mid + half_a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone2.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Prepare,
                transform: Transform::from_xyz(-3.0 * half_a, half_mid + half_a, 0.0),
                num: 0,
            },
        ),
        // 上方战场 x3
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-1.0 * half_a, half_mid + half_a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(-1.0 * half_a, half_mid + half_a, 0.0),
                num: 1,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(half_a, half_mid + half_a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(half_a, half_mid + half_a, 0.0),
                num: 2,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(3.0 * half_a, half_mid + half_a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(3.0 * half_a, half_mid + half_a, 0.0),
                num: 3,
            },
        ),
        // 上方安全屋
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-3.0 * half_a, half_mid + half_a + a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(-3.0 * half_a, half_mid + half_a + a, 0.0),
                num: 1,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-1.0 * half_a, half_mid + half_a + a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(-1.0 * half_a, half_mid + half_a + a, 0.0),
                num: 2,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(half_a, half_mid + half_a + a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(half_a, half_mid + half_a + a, 0.0),
                num: 3,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(3.0 * half_a, half_mid + half_a + a, 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(3.0 * half_a, half_mid + half_a + a, 0.0),
                num: 4,
            },
        ),
        // 上方理性区
        (
            Rectangle::from_size(Vec2::new(a, a * 2.0)),
            Transform::from_xyz(-5.0 * half_a, half_mid + a, 0.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.lx.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Lx,
                transform: Transform::from_xyz(-5.0 * half_a, half_mid + a, 0.0),
                num: 1,
            },
        ),
        // 上方激情区
        (
            Rectangle::from_size(Vec2::new(a, a * 2.0)),
            Transform::from_xyz(5.0 * half_a, half_mid + a, 0.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Z, PI)),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.jq.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::JQ,
                transform: Transform::from_xyz(5.0 * half_a, half_mid + a, 0.0),
                num: 1,
            },
        ),
        //=============
        // 下方预备区
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(3.0 * half_a, -(half_mid + half_a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone2.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Prepare,
                transform: Transform::from_xyz(3.0 * half_a, -(half_mid + half_a), 0.0),
                num: 1,
            },
        ),
        // 下方战场 x3
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a), 0.0),
                num: 1,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(half_a, -(half_mid + half_a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(half_a, -(half_mid + half_a), 0.0),
                num: 2,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.stone1.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Battle,
                transform: Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a), 0.0),
                num: 3,
            },
        ),
        // 下方安全屋
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(-3.0 * half_a, -(half_mid + half_a + a), 0.0),
                num: 1,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(-1.0 * half_a, -(half_mid + half_a + a), 0.0),
                num: 2,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(half_a, -(half_mid + half_a + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(half_a, -(half_mid + half_a + a), 0.0),
                num: 3,
            },
        ),
        (
            Rectangle::from_size(Vec2::new(a, a)),
            Transform::from_xyz(3.0 * half_a, -(half_mid + half_a + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.safe.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Safe,
                transform: Transform::from_xyz(3.0 * half_a, -(half_mid + half_a + a), 0.0),
                num: 3,
            },
        ),
        // 下方理性区
        (
            Rectangle::from_size(Vec2::new(a, a * 2.0)),
            Transform::from_xyz(5.0 * half_a, -(half_mid + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.lx.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::Lx,
                transform: Transform::from_xyz(5.0 * half_a, -(half_mid + a), 0.0),
                num: 1,
            },
        ),
        // 下方激情区
        (
            Rectangle::from_size(Vec2::new(a, a * 2.0)),
            Transform::from_xyz(-5.0 * half_a, -(half_mid + a), 0.0),
            materials.add(StandardMaterial {
                base_color_texture: Some(case_images.jq.clone()),
                unlit: true,
                ..default()
            }),
            CaseZone {
                zone_type: CaseZoneType::JQ,
                transform: Transform::from_xyz(-5.0 * half_a, -(half_mid + a), 0.0),
                num: 1,
            },
        ),
    ];

    res_mesh
}

use crate::cards::EventWithData::BackTo;
use crate::cases::CaseZone;
use bevy::ecs::observer::TriggerTargets;
use bevy::prelude::*;
use bevy_tween::combinator::{TransformTargetStateExt, event, parallel};
use bevy_tween::interpolation::EaseKind;
use bevy_tween::prelude::{AnimationBuilderExt, IntoTarget};
use bevy_tween::tween::AnimationTarget;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Default, Component)]
pub struct Card {
    pub trans: Transform,
}

#[derive(Component, Debug)]
pub struct CardInfo {}
// 生成闭包的模板

pub fn gen_put_card<C>(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
    radius: f32,
    thick: f32,
) -> impl FnMut(Handle<Image>, Transform) -> Entity
where
    C: Component,
{
    move |images: Handle<Image>, transform: Transform| {
        let mesh_list = gen_card_mesh_list(meshes, width, height, radius, thick);

        commands
            .spawn((
                Card {
                    trans: transform.clone(),
                },
                Visibility::Inherited,
                transform,
                AnimationTarget,
            ))
            .with_children(|parent| {
                // 加载黑色边框
                for (mesh_handle, trans) in mesh_list.0 {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(Color::BLACK)),
                    ));
                }
                // 加载内容
                for (mesh_handle, trans) in mesh_list.1 {
                    parent.spawn((
                        CardInfo {
                            // todo 这里是卡片的信息内容
                        },
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: Some(images.clone()),
                            alpha_mode: AlphaMode::Blend,
                            unlit: true,
                            ..Default::default()
                        })),
                    ));
                }
                // 背面
                for (mesh_handle, trans) in mesh_list.2 {
                    parent.spawn((
                        Mesh3d(mesh_handle.clone()),
                        trans.clone(),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            base_color_texture: Some(images.clone()),
                            alpha_mode: AlphaMode::Blend,
                            ..Default::default()
                        })),
                    ));
                }
            })
            .observe(move_on_drag::<C>())
            .observe(drag_start)
            .observe(drag_end)
            .observe(over_card)
            .observe(out_card)
            .id()
    }
}

// 处理拖拽到的代码
pub fn deal_on_drop(
    drag_drop: Trigger<Pointer<DragDrop>>,
    mut query: Query<&mut CaseZone>,
    mut card_query: Query<&mut CardInfo>,
    mut commands: Commands,
) {
    // 场地的值？ TODO 这处理
    info!("{:?}", drag_drop);

    let x = query.get_mut(drag_drop.target).unwrap();
    info!("{:?}", x);
    let y = card_query.get(drag_drop.dropped).unwrap();
    //todo 处理内部的场地和卡片的关系
    info!("{:?}", y);
}

pub fn drag_start(
    drag_start: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    query: Query<(), With<CardInfo>>,
) {
    if query.get(drag_start.target).is_ok() {
        commands
            .entity(drag_start.target)
            .insert(PickingBehavior::IGNORE);
    }
}

pub enum EventWithData {
    BackTo((Vec3, Vec3)),
}

pub fn over_card(
    out: Trigger<Pointer<Over>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card)>,
) {
    if let Ok(parent) = query.get(out.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());
            let mut end = tr.clone().translation;
            end.y = -2.0;
            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(end),
            );
        }
    }
}

pub fn out_card(
    out: Trigger<Pointer<Out>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card)>,
) {
    if let Ok(parent) = query.get(out.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());

            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(card.trans.translation),
            );
        }
    }
}

pub fn drag_end(
    drag_start: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    query: Query<&Parent>,
    query_transform: Query<(&Transform, &Card)>,
) {
    info!("{:?}", drag_start.target);
    // 发送回到原来位置的命令
    if let Ok(parent) = query.get(drag_start.target) {
        if let Ok((tr, card)) = query_transform.get(parent.get()) {
            // commands
            //     .entity(parent.get())
            //     .animation()
            //     .insert(parallel(event("back")));
            let target = AnimationTarget.into_target();
            let mut start = target.transform_state(tr.clone());

            commands.entity(parent.get()).animation().insert_tween_here(
                Duration::from_secs_f32(1.1),
                EaseKind::ExponentialOut,
                start.translation_to(card.trans.translation),
            );
        }
    }
    commands
        .entity(drag_start.target)
        .remove::<PickingBehavior>();
}

// 在这里个方法里 还可以做其他的事情 比如通知全局现在要选择
pub fn move_on_drag<C>() -> impl Fn(
    Trigger<Pointer<Drag>>,
    Query<&mut Transform>,
    Single<(&Camera, &GlobalTransform)>,
    Single<&Window>,
    Single<&GlobalTransform, With<C>>,
)
where
    C: Component,
{
    move |drag, mut transforms, camera_query, windows, ground| {
        // 这个是需要修改的值
        let mut transform = transforms.get_mut(drag.entity()).unwrap();

        let (camera, camera_transform) = *camera_query;

        let Some(cursor_position) = windows.cursor_position() else {
            info!("a");
            return;
        };

        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            info!("b");
            return;
        };

        // Calculate if and where the ray is hitting the ground plane.
        let Some(distance) =
            ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
        else {
            info!("c");
            return;
        };
        let point = ray.get_point(distance);
        // info!("{:?}", point);
        transform.translation.x = point.x;
        transform.translation.y = point.y;
    }
}

fn gen_card_mesh_list(
    meshes: &mut ResMut<Assets<Mesh>>,
    width: f32,
    height: f32,
    radius: f32,
    thick: f32,
) -> (
    [(Handle<Mesh>, Transform); 8],
    [(Handle<Mesh>, Transform); 1],
    [(Handle<Mesh>, Transform); 1],
) {
    // 四个 扇形 四个长方形  一个中央的部分
    let a: f32 = width - 2.0 * radius;
    let b: f32 = height - 2.0 * radius;

    // 四个角的坐标
    let right_top = Transform::from_xyz(a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4.));
    let right_bottom = Transform::from_xyz(a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 4. - PI / 2.));

    let left_top = Transform::from_xyz(-a / 2.0, b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4.));

    let left_bottom = Transform::from_xyz(-a / 2.0, -b / 2.0, 0.0)
        .with_rotation(Quat::from_axis_angle(Vec3::Z, PI / 4. + PI / 2.));

    // 四个边框的坐标
    let right = Transform::from_xyz((a + radius) / 2.0, 0.0, 0.0);
    let left = Transform::from_xyz(-(a + radius) / 2.0, 0.0, 0.0);
    let top = Transform::from_xyz(0.0, (b + radius) / 2.0, 0.0);
    let bottom = Transform::from_xyz(0.0, -(b + radius) / 2.0, 0.0);
    // 中心的坐标
    let center = Transform::from_xyz(0.0, 0.0, thick / 2.);
    let back = Transform::from_xyz(0.0, 0.0, -thick / 2.)
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI));
    // 加载一组的shape

    let frames = [
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            right_bottom,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_top,
        ),
        (
            meshes.add(Extrusion::new(CircularSector::new(radius, PI / 4.0), thick)),
            left_bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            top,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(a, radius)),
                thick,
            )),
            bottom,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            left,
        ),
        (
            meshes.add(Extrusion::new(
                Rectangle::from_size(Vec2::new(radius, b)),
                thick,
            )),
            right,
        ),
    ];

    // 正面主要
    let content = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), center)];
    let back_side = [(meshes.add(Rectangle::from_size(Vec2::new(a, b))), back)];

    (frames, content, back_side)
}

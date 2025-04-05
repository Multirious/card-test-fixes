use crate::cards::Card;
use bevy::app::App;
use bevy::color::palettes::css::{WHITE, YELLOW};
use bevy::picking::focus::update_interactions;
use bevy::prelude::*;
use bevy_tween::combinator::{backward, forward, sequence, tween};
use bevy_tween::interpolate::{scale, sprite_color, translation_to};
use bevy_tween::prelude::*;
use bevy_tween::tween::AnimationTarget;
use rand::prelude::*;
use std::f32::consts::PI;

pub mod camera_controller;
pub mod cards;
pub mod cases;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, effect_system);
    }
}

#[derive(Component)]
pub struct OnConfirm;

#[derive(Component)]
pub struct OnCancel;

fn spawn_ui_popup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    title: &'static str,
    mut on_confirm: impl FnMut(&mut Commands, &mut Query<&Children, With<Card>>) + Send + Sync + 'static,
    mut on_cancel: impl FnMut(&mut Commands) + Send + Sync + 'static,
) {
    let all = commands
        .spawn(
            (Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }),
        )
        .id();

    commands.entity(all).with_children(|plane| {
        plane
            .spawn((
                Node {
                    width: Val::Px(500.),
                    height: Val::Px(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
                BorderColor(Color::BLACK),
                BorderRadius::all(Val::Px(10.0)),
            ))
            .with_children(|parent| {
                parent
                    .spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },))
                    .with_children(|title_bar| {
                        title_bar.spawn((
                            Text::new(title),
                            TextFont {
                                font: asset_server.load("fonts/wqy-microhei.ttc"),
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                        ));
                    });

                // 按钮区域
                parent
                    .spawn(
                        (Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..default()
                        }),
                    )
                    .with_children(|b_zone| {
                        b_zone
                            .spawn((
                                Button,
                                OnConfirm,
                                Node {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.0, 0.1, 0.1)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("确认"),
                                    TextFont {
                                        font: asset_server.load("fonts/wqy-microhei.ttc"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.0)),
                                ));
                            })
                            .observe(
                                move |click: Trigger<Pointer<Click>>, mut commands: Commands, mut children_query: Query<&Children,With<Card>>, | {
                                    on_confirm(&mut commands, &mut children_query);
                                    commands.entity(all).despawn_recursive();
                                },
                            );

                        b_zone
                            .spawn((
                                Button,
                                OnCancel,
                                Node {
                                    width: Val::Px(80.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("取消"),
                                    TextFont {
                                        font: asset_server.load("fonts/wqy-microhei.ttc"),
                                        font_size: 33.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                ));
                            })
                            .observe(
                                move |click: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    on_cancel(&mut commands);
                                    commands.entity(all).despawn_recursive();
                                },
                            );
                    });
            });
    });
}

#[derive(Component)]
pub struct MainCamera;

fn effect_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event: EventReader<TweenEvent<&'static str>>,
    query: Query<(Entity, &Transform), With<MainCamera>>,
) {
    event.read().for_each(|event| match event.data {
        "boom" => {
            info!("Boom!");
            let entity = AnimationTarget.into_target();
            commands
                .spawn((
                    Sprite {
                        image: asset_server.load("circle.png"),
                        ..default()
                    },
                    // todo 这里的值要变
                    Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    // .with_rotation(Quat::from_axis_angle(Vec3::Y, -PI / 2.0)),
                    AnimationTarget,
                ))
                .animation()
                .insert_tween_here(
                    Duration::from_secs_f32(10.0),
                    EaseKind::QuadraticOut,
                    (
                        entity.with(scale(Vec3::new(1., 1., 0.), Vec3::new(15., 15., 0.))),
                        entity.with(sprite_color(
                            into_color(WHITE.with_alpha(1.)),
                            into_color(YELLOW.with_alpha(1.)),
                        )),
                    ),
                );
        }
        "shark2" => {
            // todo 镜头动
            let mut rng = rand::thread_rng();
            let dx: f32 = rng.gen_range(-5.0..=5.0);
            let dy: f32 = rng.gen_range(-5.0..=5.0);

            if let Ok((entity, trans)) = query.get_single() {
                let entity_a = AnimationTarget.into_target();
                let mut target_state = entity_a.state(trans.translation.clone());
                commands
                    .entity(entity)
                    .insert(AnimationTarget)
                    .animation()
                    .repeat(Repeat::Times {
                        times: 7,
                        times_repeated: 1,
                    })
                    .insert(sequence((tween(
                        Duration::from_secs_f32(0.1),
                        EaseKind::ExponentialOut,
                        target_state.with(translation_to(Vec3::new(dx, dy, trans.translation.z))),
                    ),)));
            }
        }
        _ => {}
    });
}

fn into_color<T: Into<bevy::color::Srgba>>(color: T) -> Color {
    Color::Srgba(color.into())
}

fn big_x_do_effect(
    mut q_big_x: Query<&mut Transform, With<MainCamera>>,
    // mut q_rotation_animator: Query<&mut TimeRunner, With<RotatationAnimator>>,
) {
    let mut rng = rand::thread_rng();
    let dx: f32 = rng.gen_range(-5.0..=5.0);
    let dy: f32 = rng.gen_range(-5.0..=5.0);
    let mut new_vec = q_big_x.single_mut().translation;
    new_vec.x += dx;
    new_vec.y += dy;
    q_big_x.single_mut().translation = new_vec;
}

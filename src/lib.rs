use bevy::app::App;
use bevy::picking::focus::update_interactions;
use bevy::prelude::*;
use bevy::utils::info;

pub mod camera_controller;
pub mod cards;
pub mod cases;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_interactions);
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
    mut on_confirm: impl FnMut(&mut Commands) + Send + Sync + 'static,
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
                                move |click: Trigger<Pointer<Click>>, mut commands: Commands| {
                                    on_confirm(&mut commands);
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

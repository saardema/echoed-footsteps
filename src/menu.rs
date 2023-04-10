use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            // .add_system(skip_menu.in_set(OnUpdate(GameState::Menu)));
            .insert_resource(NextLevelState {
                delay_timer: Timer::from_seconds(2.0, TimerMode::Once),
                flash_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                flash_on: true,
            })
            .add_system(animate_level_complete_screen.in_set(OnUpdate(GameState::LevelComplete)))
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::LevelComplete)))
            .add_system(init_level_complete_screen.in_schedule(OnEnter(GameState::LevelComplete)));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    // commands.spawn(TextBundle::from_section(
    //     "Echoed Footsteps",
    //     TextStyle {
    //         font: font_assets.pixeboy.clone(),
    //         font_size: 60.0,
    //         color: Color::rgb(0.9, 0.9, 0.9),
    //     },
    // ));

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: font_assets.pixeboy.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn skip_menu(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Playing);
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands,
    button: Query<Entity, With<Button>>,
    text: Query<Entity, With<Text>>,
) {
    for entity in button.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in text.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn init_level_complete_screen(mut commands: Commands, font_assets: Res<FontAssets>) {
    // commands.spawn(TextBundle::from_section(
    //     "Level Complete!",
    //     TextStyle {
    //         font: font_assets.pixeboy.clone(),
    //         font_size: 60.0,
    //         color: Color::rgb(0.9, 0.9, 0.9),
    //     },
    // ));

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Level Complete!",
                TextStyle {
                    font: font_assets.pixeboy.clone(),
                    font_size: 60.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        })
        .insert(Transform::from_xyz(
            WINDOW_WIDTH / 2.,
            WINDOW_HEIGHT / 2.,
            1.,
        ));
}

fn animate_level_complete_screen(
    mut text_query: Query<&mut Visibility, With<Text>>,
    mut next_level_state: ResMut<NextLevelState>,
    mut state: ResMut<NextState<GameState>>,
    mut level_selection: ResMut<LevelSelection>,
    time: Res<Time>,
) {
    next_level_state.delay_timer.tick(time.delta());
    next_level_state.flash_timer.tick(time.delta());

    if next_level_state.flash_timer.just_finished() {
        next_level_state.flash_on = !next_level_state.flash_on;
    }

    for mut visibility in text_query.iter_mut() {
        *visibility = match next_level_state.flash_on {
            false => Visibility::Hidden,
            true => Visibility::Visible,
        }
    }

    if next_level_state.delay_timer.just_finished() {
        match *level_selection {
            LevelSelection::Index(i) => *level_selection = LevelSelection::Index(i + 1),
            _ => {}
        }
        state.set(GameState::Playing);
    }
}

#[derive(Resource)]
struct NextLevelState {
    delay_timer: Timer,
    flash_timer: Timer,
    flash_on: bool,
}

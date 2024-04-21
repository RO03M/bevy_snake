use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

// Size of the arena in grid units
const ARENA_WIDTH: i32 = 15;
const ARENA_HEIGHT: i32 = 15;

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct SnakeHead {
    direction: Direction
}

#[derive(Component, Clone)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_snake, spawn_segment, spawn_segment, spawn_segment, spawn_segment))
            .add_systems(Update, (
                handle_keyboard_direction,
                snake_movement.run_if(on_timer(Duration::from_millis(250))),
                spawn_food.run_if(on_timer(Duration::from_millis(100)))
            ))
            .add_systems(PostUpdate, (size_scaling.pipe(position_translation)));
    }
}

fn spawn_snake(mut commands: Commands) {
    const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        },
        Size::square(1.0),
        Position {
            x: 0,
            y: 0
        },
        SnakeHead {
            direction: Direction::Right
        }
    ));
}

fn spawn_segment(mut commands: Commands) {
    const SNAKE_HEAD_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            ..default()
        },
        Size::square(1.0),
        Position {
            x: 0,
            y: 0
        },
        SnakeSegment
    ));   
}

fn spawn_food(mut commands: Commands) {
    const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
    let mut rng = rand::thread_rng();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        },
        Size::square(1.0),
        Position {
            x: rng.gen_range((-ARENA_WIDTH / 2)..(ARENA_WIDTH / 2)),
            y: rng.gen_range((-ARENA_HEIGHT / 2)..(ARENA_HEIGHT / 2))
        },
        Food
    ));
}

fn handle_keyboard_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut snake_head: Query<&mut SnakeHead>
) {
    let mut snake_head = snake_head.single_mut();
    
    let direction = if input.pressed(KeyCode::ArrowLeft) {
        Direction::Left
    } else if input.pressed(KeyCode::ArrowRight) {
        Direction::Right
    } else if input.pressed(KeyCode::ArrowDown) {
        Direction::Down
    } else if input.pressed(KeyCode::ArrowUp) {
        Direction::Up
    } else {
        snake_head.direction
    };

    snake_head.direction = direction;
}

fn snake_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut head_query: Query<(&SnakeHead, &mut Position), Without<SnakeSegment>>,
    mut segments_query: Query<(&mut Position), (Without<SnakeHead>, Without<Food>)>
) {
    const SNAKE_SPEED: i32 = 1;
    
    let (snake_head, mut snake_head_position) = head_query.single_mut();
    let mut previous_position = snake_head_position.clone();

    match snake_head.direction {
        Direction::Up => {
            snake_head_position.y += SNAKE_SPEED
        },
        Direction::Left => {
            snake_head_position.x -= SNAKE_SPEED;
        },
        Direction::Right => {
            snake_head_position.x += SNAKE_SPEED;
        },
        Direction::Down => {
            snake_head_position.y -= SNAKE_SPEED;
        },
    };

    for mut segment in segments_query.iter_mut() {
        let current_segment_position = segment.clone();

        segment.x = previous_position.x;
        segment.y = previous_position.y;

        previous_position = current_segment_position;
    }
}

fn size_scaling(
    windows: Query<&Window>,
    mut query: Query<(&Size, &mut Transform)>
) {
    let window = windows.single();

    for (size, mut transform) in query.iter_mut() {
        let grid_width = size.width / ARENA_WIDTH as f32 * window.width() as f32 ;
        let grid_height = size.height / ARENA_HEIGHT as f32 * window.height() as f32;
        transform.scale = Vec3::new(
            grid_width,
            grid_height,
            1.0
        );
    }
}

fn position_translation(
    mut query: Query<(&Position, &mut Transform)>
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            position.x as f32 * transform.scale.x,
            position.y as f32 * transform.scale.y,
            0.0
        );
    }
}
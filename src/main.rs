mod globals;

use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use globals::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            height: WINDOW_HEIGHT,
            resizable: false,
            title: "Click Me If You Can".to_string(),
            width: WINDOW_WIDTH,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(game_setup.system())
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // Defining the Wall
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.;

    commands
        // Cameras
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        
        // Left Wall
        .spawn(SpriteComponents {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-400., 0., 0.)),
            sprite: Sprite::new(Vec2::new(wall_thickness, 200. + wall_thickness)),
            ..Default::default()
        })
        ;
}
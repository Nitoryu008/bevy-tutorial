use bevy::color::palettes::css::*;
use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let mesh = Mesh2d(meshes.add(Circle::new(100.)));
    let material = MeshMaterial2d(materials.add(Color::from(SKY_BLUE)));
    let transform = Transform::from_xyz(0., 0., 0.);
    commands.spawn((mesh, material, transform));
    let mesh = Mesh2d(meshes.add(Rectangle::new(10., 50.)));
    let material = MeshMaterial2d(materials.add(Color::from(RED_900)));
    let transform = Transform::from_xyz(200., 10., 0.);
    commands.spawn((mesh, material, transform));
}

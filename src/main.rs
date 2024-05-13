use bevy::prelude::*;
use bevy_lunex::prelude::*;
use bevy_mod_picking::prelude::*;

mod boilerplate;
use boilerplate::*;


fn main() {
    App::new()
        .add_plugins((default_plugins(), DefaultPickingPlugins, UiGeneralPlugin, UiPlugin::<MainUi>::new()))
        //.add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())

        .add_systems(Startup, setup)

        .add_systems(Update, rotate_playercam)
        .add_systems(Update, zoom_playercam)
        .run();
}

fn setup(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // Spawn camera
    commands.spawn(
        PbrBundle { transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(1.0, 1.0, 1.0)), ..default() }
    ).with_children(|parent| {
        parent.spawn((
            Camera3dBundle::default(),
            PlayerCam {
                orbit: Vec3::new(0.0, 0.0, 0.0),
                distance: 800.0,
                sensitivity: Vec2::splat(0.1),
            }
        ));
    });


    for x in -1..2 {

        // Spawn the master ui
        commands.spawn((
            UiTreeBundle::<MainUi> {
                transform: Transform::from_xyz(-400.0, 300.0, 0.0 + (300.0 * x as f32)),
                tree: UiTree::new("MyWidget"),
                ..default()
            },
            mesh.add(Mesh::from(Cuboid { half_size: Vec3::splat(10.0) })),
            material.add(Color::rgb(1.0, 0.0, 1.0)),
        )).with_children(|ui| {
    
            let root = UiLink::<MainUi>::path("Root");
            ui.spawn((
                root.clone(),
                UiLayout::window_full().size((818.0, 965.0)).pack(),
                //UiMaterial3dBundle::from_image(&mut material, asset_server.load("bevycom.png")),
                UiMaterial3dBundle::from_transparent_image(&mut material, asset_server.load("bevycom.png")),
            ));

            ui.spawn((
                root.add("Head"),
                UiLayout::window_full().height(Rl(35.0)).pack(),
                //UiMaterial3dBundle::from_image(&mut material, asset_server.load("bevycom.png")),
                //UiMaterial3dBundle::from_transparent_image(&mut material, asset_server.load("bevycom_base_head.png")),
            ));
    
        }); 
    }

}

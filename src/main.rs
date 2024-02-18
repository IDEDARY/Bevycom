use bevy::prelude::*;
use bevy_lunex::prelude::*;

mod boilerplate;
use boilerplate::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiPlugin::<NoData, NoData, MyWidget>::new()))
        .add_plugins(UiDebugPlugin::<NoData, NoData, MyWidget>::new())

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
            UiTreeBundle::<NoData, NoData, MyWidget> {
                transform: Transform::from_xyz(-400.0, 300.0, 0.0 + (350.0 * x as f32)),
                tree: UiTree::new("MyWidget"),
                ..default()
            },
            mesh.add(Mesh::from(Cuboid { half_size: Vec3::splat(10.0) })),
            material.add(Color::rgb(1.0, 0.0, 1.0)),
            Visibility::default(),
            ViewVisibility::default(),
        )).with_children(|ui| {
    
            let root = UiLink::path("Root");
            ui.spawn((
                MyWidget,
                root.clone(),
                UiLayout::Window::FULL.size((818.0, 965.0)).pack(),
                UiMaterial3dBundle::from_image(&mut material, asset_server.load("bevycom.png")),
                //UiMaterial3dBundle::from_transparent_image(&mut material, asset_server.load("bevycom.png")),
            ));
    
            let head = root.add("Head");
            ui.spawn((
                MyWidget,
                head.clone(),
                UiLayout::Div::new().pad(20.0).pack(),
                UiStack::new().direction(FlexDirection::Vertical),
                UiMaterial3dBundle::from_transparent_image(&mut material, asset_server.load("bevycom_base_head.png")),
            ));
    
            ui.spawn((
                MyWidget,
                head.add("Icon"),
                UiLayout::Div::new().margin_r(20.0).br().pack(),
                UiContent::new((115.0, 155.0)),
            ));
    
            ui.spawn((
                MyWidget,
                head.add("Rank"),
                UiLayout::Div::new().margin_b(10.0).pack(),
                UiContent::new((100.0, 30.0)),
            ));
    
            ui.spawn((
                MyWidget,
                head.add("Name"),
                UiLayout::Div::new().margin_b(20.0).pack(),
                UiContent::new((350.0, 45.0))
            ));
    
            let list = head.add("List");
            ui.spawn((
                MyWidget,
                list.clone(),
                UiLayout::Div::new().pad_y(10.0).pack(),
                UiStack::new().gap_x(10.0)
            ));
    
            {
                ui.spawn((
                    MyWidget,
                    list.add("Missions"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((200.0, 30.0))
                ));
    
                ui.spawn((
                    MyWidget,
                    list.add("Kills"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((150.0, 30.0))
                ));
    
                ui.spawn((
                    MyWidget,
                    list.add("Status"),
                    UiLayout::Div::new().pack(),
                    UiContent::new((250.0, 30.0))
                ));
            }
    
        }); 
    }

}

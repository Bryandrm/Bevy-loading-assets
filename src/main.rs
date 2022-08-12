use std::iter::OnceWith;

use bevy::asset::Asset;
use bevy::asset::HandleId;
use bevy::ecs::entity;
use bevy::prelude::*;
use bevy::render::camera;
use bevy_flycam::MovementSettings;
use bevy_flycam::PlayerPlugin;

//struct Monkey <'a>{  HandleMesh:  &'a Handle<Mesh> }
#[derive (Default)]
//struct HeadMesh {  handle_mesh:   Vec<Handle<Mesh>> }
struct HeadMesh {  handle_mesh:   Vec<HandleId> }


/// This example illustrates various ways to load assets
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_plugin(PlayerPlugin)
        .insert_resource(HeadMesh::default())
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0, // default: 12.0
        })
        .add_system(keyboard_input_system)
        .add_system(check_assets)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    meshes: Res<Assets<Mesh>>,
    mut str_mokey: ResMut<HeadMesh> ,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // By default AssetServer will load assets from inside the "assets" folder.
    // For example, the next line will load "ROOT/assets/models/cube/cube.gltf#Mesh0/Primitive0",
    // where "ROOT" is the directory of the Application.
    //
    // This can be overridden by setting the "CARGO_MANIFEST_DIR" environment variable (see
    // https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    // to another directory. When the Application is run through Cargo, "CARGO_MANIFEST_DIR" is
    // automatically set to your crate (workspace) root directory.
    let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
    let sphere_handle = asset_server.load("models/sphere/sphere.gltf#Mesh0/Primitive0");

    // All assets end up in their Assets<T> collection once they are done loading:
    if let Some(sphere) = meshes.get(&sphere_handle) {
        // You might notice that this doesn't run! This is because assets load in parallel without
        // blocking. When an asset has loaded, it will appear in relevant Assets<T>
        // collection.
        info!("{:?}", sphere.primitive_topology());
    } else {
        info!("sphere hasn't loaded yet");
    }

    // You can load all assets in a folder like this. They will be loaded in parallel without
    // blocking
    let mut _scenes: Vec<HandleUntyped> = asset_server.load_folder("models/monkey").unwrap();
    //_scenes.remove(1);
    // Then any asset in the folder can be accessed like this:
    //let monkey_handle = asset_server.get_handle("models/monkey/Helmet/FlightHelmet.gltf#Scene1");
    let monkey_handle = asset_server.get_handle("models/monkey/Monkey.gltf#Mesh0/Primitive0");
    
    println!(" monkey: {:?} ",asset_server.get_load_state(&monkey_handle));
    str_mokey.handle_mesh.push(monkey_handle.id);
    
    // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });

    // monkey
    commands.spawn_bundle(PbrBundle {
        mesh: monkey_handle,
        material:material_handle.clone(),
        transform: Transform::from_xyz(-3.0, 0.0, 0.0),
        
        ..default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: cube_handle,
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // sphere
    commands.spawn_bundle(PbrBundle {
        mesh: sphere_handle,
        material: material_handle,
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });
    // camera
    

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    str_mokey: ResMut<HeadMesh> ,
    entities: Query<Entity,Without<Camera>>,
    asset_server: ResMut<AssetServer>,
    mut asset: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    

    //myassets: Res<Assets<MyAsset>>
) {
    if keyboard_input.pressed(KeyCode::A) {
        //info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::A) {
     
    }

    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released: " );
        entities.iter().for_each(|f| println!("entity {:?} ",f));
        
        for entity in entities.iter() {            
            println!("entity iter: {:?}", entity);            
            let handle = asset_server.get_handle_untyped(str_mokey.handle_mesh[0]);
            asset.remove(handle);
            asset_server.free_unused_assets();
        }     
    }

    if keyboard_input.just_released(KeyCode::S) { 
        commands.spawn_bundle(PbrBundle {
            mesh: asset_server.get_handle(str_mokey.handle_mesh[0]),
            transform: Transform::from_xyz(-3.0, 2.0, 0.0),
            ..default()
        });        
    }

    if keyboard_input.just_released(KeyCode::D) { 
        let mut _scenes: Vec<HandleUntyped> = asset_server.load_folder("models/monkey").unwrap();
        let monkey_handle = asset_server.get_handle("models/monkey/Monkey.gltf#Mesh0/Primitive0");
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.6),
            ..default()
        });
        commands.spawn_bundle(PbrBundle {
            mesh: monkey_handle,
            material:material_handle.clone(),
            transform: Transform::from_xyz(-3.0, 0.0, 0.0),
            
            ..default()
        });
    }

    
}

fn check_assets(
    mut event: EventReader<AssetEvent<Mesh>>,
    str_mokey: ResMut<HeadMesh>,
    asset_server: ResMut<AssetServer>,
){

    for e in event.iter() {
        match e {
            AssetEvent::Created { handle }=> {
                if *handle==asset_server.get_handle(str_mokey.handle_mesh[0]) {
                    println!("asset created");
                }
            }
            AssetEvent::Modified { handle } => 
                if *handle==asset_server.get_handle(str_mokey.handle_mesh[0]){
                    println!("asset modified");
                },
            

            AssetEvent::Removed { handle } => 
                if *handle==asset_server.get_handle(str_mokey.handle_mesh[0]){
                    println!("asset removed");
                },
        }
    }
}
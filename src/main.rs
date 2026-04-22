use bevy::prelude::*;
use bevy_flycam::prelude::*;
use bevy_voxel_world::prelude::*;

#[derive(Resource, Clone, Default)]
struct GameWorld;

impl VoxelWorldConfig for GameWorld {
    type MaterialIndex = u8;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        12
    }

    fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
        ChunkDespawnStrategy::FarAway
    }

    fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
        ChunkSpawnStrategy::Close
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        Box::new(|_, _, _| Box::new(flat_ground))
    }
}

fn flat_ground(pos: IVec3, _: Option<WorldVoxel<u8>>) -> WorldVoxel<u8> {
    if pos.y <= 0 {
        WorldVoxel::Solid(0)
    } else {
        WorldVoxel::Unset
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            VoxelWorldPlugin::with_config(GameWorld),
            PlayerPlugin,
        ))
        .add_systems(PostStartup, add_voxel_camera_to_flycam)
        .run();
}

fn add_voxel_camera_to_flycam(mut commands: Commands, flycams: Query<Entity, With<FlyCam>>) {
    for entity in &flycams {
        commands
            .entity(entity)
            .insert(VoxelWorldCamera::<GameWorld>::default());
    }
}

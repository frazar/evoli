use std::collections::HashMap;
use std::fs::read_dir;

use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    utils::application_root_dir,
};

use crate::components::creatures::CreaturePrefabData;

#[derive(Default)]
pub struct CreaturePrefabs {
    prefabs: HashMap<String, Handle<Prefab<CreaturePrefabData>>>,
}

impl CreaturePrefabs {
    pub fn insert(
        &mut self,
        creature_type: String,
        prefab_handle: Handle<Prefab<CreaturePrefabData>>,
    ) {
        self.prefabs.insert(creature_type, prefab_handle);
    }

    pub fn get_prefab(&self, creature_type: &str) -> Option<&Handle<Prefab<CreaturePrefabData>>> {
        self.prefabs.get(creature_type)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<CreaturePrefabData>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<CreaturePrefabData>>>) {
        self.prefabs = prefabs;
    }
}

// Here we load all prefabs for the different creatures in the game.
// These prefabs are then stored in a resource of type CreaturePrefabs that is used by the spawner system.
// At initialization time, we put temporary keys for the prefabs since they're not loaded yet.
// When their loading is finished, we read the name of the entity inside to change the keys. This is done in the update_prefabs function.
pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut creature_prefabs = CreaturePrefabs::default();
    let mut progress_counter = ProgressCounter::new();
    let prefab_iter = {
        let prefab_dir_path = application_root_dir() + "/resources/prefabs/creatures";
        let prefab_iter = read_dir(prefab_dir_path).unwrap();
        prefab_iter.map(|prefab_dir_entry| {
            world.exec(|loader: PrefabLoader<'_, CreaturePrefabData>| {
                let prefab_path_buf = prefab_dir_entry.unwrap().path();
                let prefab_filename = prefab_path_buf.file_name().unwrap();
                loader.load(
                    "prefabs/creatures/".to_string() + prefab_filename.to_str().unwrap(),
                    RonFormat,
                    (),
                    &mut progress_counter,
                )
            })
        })
    };

    for (count, prefab) in prefab_iter.enumerate() {
        creature_prefabs.insert("temp_prefab_".to_string() + &count.to_string(), prefab);
    }
    world.add_resource(creature_prefabs);
    progress_counter
}

// Once the prefabs are loaded, this function is called to update the ekeys in the CreaturePrefabs struct.
// We use the Named component of the entity to determine which key to use.
pub fn update_prefabs(world: &mut World) {
    let updated_prefabs = {
        let creature_prefabs = world.read_resource::<CreaturePrefabs>();
        let prefabs = creature_prefabs.get_prefabs();
        let mut prefab_resource =
            world.write_resource::<AssetStorage<Prefab<CreaturePrefabData>>>();
        let mut new_prefabs = HashMap::new();
        for (_key, handle) in prefabs.iter() {
            if let Some(prefab) = prefab_resource.get_mut(handle) {
                if let Some(prefab_data) = prefab.entity(0) {
                    let name = prefab_data
                        .data()
                        .unwrap()
                        .name
                        .as_ref()
                        .unwrap()
                        .name
                        .to_string();
                    new_prefabs.insert(name, handle.clone());
                }
            }
        }
        new_prefabs
    };
    world
        .write_resource::<CreaturePrefabs>()
        .set_prefabs(updated_prefabs);
}

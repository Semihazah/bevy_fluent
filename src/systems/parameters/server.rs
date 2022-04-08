use crate::{
    components::{Cache, Queue},
    Localization,
};
use ahash::AHasher;
use bevy::{
    asset::{AssetPath, HandleId},
    ecs::system::SystemParam,
    prelude::*,
    reflect::TypeUuid,
};
use indexmap::IndexSet;
use std::{hash::{Hash, Hasher}, marker::PhantomData};

/// Fluent server
#[derive(SystemParam)]
pub struct FluentServer<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    cache: ResMut<'w, Cache>,
    queue: ResMut<'w, Queue>,
    
    #[system_param(ignore)]
    phantom_data: PhantomData<&'s usize>
}

impl<'w, 's> FluentServer<'w, 's> {
    /// Loads locale fallback chain assets
    pub fn load<'a, I, T>(&self, paths: I) -> Handle<Localization>
    where
        I: IntoIterator<Item = T>,
        T: Into<AssetPath<'a>>,
    {
        let mut handles = IndexSet::new();
        let mut hasher = AHasher::new_with_keys(42, 23);
        for path in paths {
            let asset_path = path.into();
            asset_path.hash(&mut hasher);
            let handle = self.asset_server.load(asset_path);
            handles.insert(handle);
        }
        let id = HandleId::Id(Localization::TYPE_UUID, hasher.finish());
        self.cache.write().insert(id, handles.clone());
        self.queue.write().push_back((id, handles));
        self.asset_server.get_handle(id)
    }
}

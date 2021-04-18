use crate::{components::Queue, Localization};
use ahash::AHasher;
use bevy::{asset::HandleId, ecs::system::SystemParam, prelude::*, reflect::TypeUuid};
use indexmap::IndexSet;
use std::{
    ffi::OsStr,
    hash::{Hash, Hasher},
    path::Path,
};

/// Fluent server
#[derive(SystemParam)]
pub struct FluentServer<'a> {
    asset_server: Res<'a, AssetServer>,
    queue: ResMut<'a, Queue>,
}

impl FluentServer<'_> {
    /// Load locale pack
    pub fn load_pack<'a, I, T>(&self, iter: I) -> Handle<Localization>
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + AsRef<OsStr> + ?Sized,
    {
        let mut hasher = AHasher::new_with_keys(42, 23);
        let mut handles = IndexSet::new();
        for item in iter {
            let path = Path::new(item);
            path.hash(&mut hasher);
            let handle = self.asset_server.load(path);
            handles.insert(handle);
        }
        let id = HandleId::Id(Localization::TYPE_UUID, hasher.finish());
        self.queue.write().push_back((id, handles));
        self.asset_server.get_handle(id)
    }
}

use bevy::{
    ecs::{Command, Component},
    prelude::*,
};
use std::marker::PhantomData;

/// Extension methods for [`Commands`](bevy::ecs::system::Commands)
pub trait CommandsExt {
    fn init_resource<T: Component + FromResources>(&mut self) -> &mut Self;
}

impl CommandsExt for Commands {
    fn init_resource<T: Component + FromResources>(&mut self) -> &mut Self {
        self.add_command(InitResource {
            _phantom_data: PhantomData::<T>,
        })
    }
}

pub struct InitResource<T: Component + FromResources> {
    _phantom_data: PhantomData<T>,
}

impl<T: Component + FromResources> Command for InitResource<T> {
    fn write(self: Box<Self>, _: &mut World, resources: &mut Resources) {
        if !resources.contains::<T>() {
            let resource = T::from_resources(resources);
            resources.insert(resource);
        }
    }
}

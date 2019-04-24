use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystem, ProgressCounter, RonFormat},
    config::Config,
    core::transform::{Transform, TransformBundle},
    derive::PrefabData,
    ecs::{prelude::Entity, Entities, Join, ReadStorage, WriteStorage},
    error::Error,
    prelude::{Builder, World},
    renderer::{
        Camera, DisplayConfig, DrawFlat2D, Pipeline, Projection, RenderBundle, ScreenDimensions,
        SpriteRender, SpriteScenePrefab, Stage,
    },
    utils::application_root_dir,
    Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData, Trans,
};

use crate::components::register_components;
use crate::entities::init_entities;
use crate::resources::add_resources;
use crate::resources::{AnimationId, AnimationPrefabData};

pub struct GameState {
    // A progress tracker to check that assets are loaded
    pub progress_counter: ProgressCounter,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            progress_counter: Default::default(),
        }
    }
}

impl SimpleState for GameState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let world = state_data.world;

        register_components(world);
        add_resources(world);
        init_entities(world, &mut self.progress_counter);

        println!("start");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if self.progress_counter.is_complete() {
            let StateData { world, .. } = data;
            // Execute a pass similar to a system
            world.exec(
                |(entities, animation_sets, mut control_sets): (
                    Entities,
                    ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                    WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                )| {
                    // For each entity that has AnimationSet
                    for (entity, animation_set) in (&entities, &animation_sets).join() {
                        // Creates a new AnimationControlSet for the entity
                        let control_set = get_animation_set(&mut control_sets, entity).unwrap();

                        // Adds the `Idle` animation to AnimationControlSet and loops infinitely
                        control_set.add_animation(
                            AnimationId::Walk,
                            &animation_set.get(&AnimationId::Walk).unwrap(),
                            EndControl::Loop(None),
                            1.0,
                            AnimationCommand::Start,
                        );
                    }
                },
            );
        }
        Trans::None
    }
}
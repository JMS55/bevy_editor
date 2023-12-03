use bevy::{
    app::{App, Plugin, Startup, Update},
    core::DebugName,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut, Resource},
    },
    DefaultPlugins,
};
use bevy_mod_picking::{
    backends::bevy_ui::BevyUiBackend,
    input::InputPlugin,
    picking_core::{CorePlugin, InteractionPlugin},
};
use bevy_quill::{Cx, Element, For, If, QuillPlugin, View, ViewHandle};

pub struct EditorPlugin {}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins((CorePlugin, InputPlugin, InteractionPlugin, BevyUiBackend))
            .add_plugins(QuillPlugin)
            .init_resource::<SceneEntities>()
            .add_systems(Startup, setup_editor)
            .add_systems(Update, update_scene_entities);
    }
}

pub fn setup_editor(mut commands: Commands) {
    commands.spawn(ViewHandle::new(editor_root, ()));
    commands.spawn(Camera2dBundle::default());
}

fn editor_root(_: Cx) -> impl View {
    scene_panel
}

fn scene_panel(mut cx: Cx) -> impl View {
    let scene_entities = &cx.use_resource::<SceneEntities>().0;

    If::new(
        scene_entities.is_empty(),
        "<No entities exist>",
        Element::new().children(For::keyed(
            scene_entities,
            |(entity, _)| *entity,
            |(_, name)| name.clone(),
        )),
    )
}

// ----------------------------------------------------------------------------

#[derive(Resource, Default)]
struct SceneEntities(Vec<(Entity, String)>);

fn update_scene_entities(
    entities: Query<(Entity, DebugName)>,
    mut scene_entities: ResMut<SceneEntities>,
) {
    scene_entities.0.clear();
    scene_entities
        .0
        .extend(entities.iter().map(|(entity, name)| {
            (
                entity,
                match name.name {
                    Some(name) => name.to_string(),
                    None => format!("Entity <{entity:?}>"),
                },
            )
        }));
}

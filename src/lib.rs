use bevy::{
    app::{App, Plugin, Startup, Update},
    core::DebugName,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        system::{Commands, Query, ResMut, Resource},
        world::EntityWorldMut,
    },
    render::color::Color,
    ui::FlexDirection,
    DefaultPlugins,
};
use bevy_mod_picking::{
    backends::bevy_ui::BevyUiBackend,
    events::{Click, Pointer},
    input::InputPlugin,
    picking_core::{CorePlugin, InteractionPlugin},
    prelude::On,
};
use bevy_quill::prelude::{
    AtomHandle, AtomStore, Cx, Element, For, If, PresenterFn, QuillPlugin, StyleHandle, View,
    ViewHandle,
};

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

// ----------------------------------------------------------------------------

fn editor_root(mut cx: Cx) -> impl View {
    let selected_entity = cx.create_atom_init(|| Option::<Entity>::None);

    Element::new()
        .children((
            scene_tree_panel.bind(selected_entity),
            entity_inspector_panel.bind(selected_entity),
        ))
        .styled(StyleHandle::build(|s| s.flex_direction(FlexDirection::Row)))
}

fn scene_tree_panel(cx: Cx<AtomHandle<Option<Entity>>>) -> impl View {
    let selected_entity = *cx.props;

    let on_click_set_selected_entity = move |v: Option<Entity>| {
        move |mut e: EntityWorldMut| {
            e.insert(On::<Pointer<Click>>::run(move |mut atoms: AtomStore| {
                atoms.set(selected_entity, v);
            }));
        }
    };

    let selected_entity = cx.read_atom(selected_entity);
    let entity_widget = move |(entity, name): &(Entity, String)| {
        name.clone()
            .styled(StyleHandle::build(|s| {
                if Some(*entity) == selected_entity {
                    s.background_color(Color::PURPLE)
                } else {
                    s.background_color(Color::MIDNIGHT_BLUE)
                }
                .padding(12.0)
            }))
            .once(on_click_set_selected_entity(Some(*entity)))
    };

    let scene_entities = &cx.use_resource::<SceneEntities>().0;
    If::new(
        scene_entities.is_empty(),
        "No entities exist",
        Element::new()
            .children(For::keyed(
                scene_entities,
                |(entity, _)| *entity,
                entity_widget,
            ))
            .styled(StyleHandle::build(|s| {
                s.flex_direction(FlexDirection::Column)
            })),
    )
    .once(on_click_set_selected_entity(None))
}

fn entity_inspector_panel(cx: Cx<AtomHandle<Option<Entity>>>) -> impl View {
    let selected_entity = cx.read_atom(*cx.props);
    If::new(
        selected_entity.is_some(),
        "TODO: Component widgets",
        "Select an entity to view its components",
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

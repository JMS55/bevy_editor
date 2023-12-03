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
use bevy_quill::{
    Cx, Element, For, If, LocalData, PresenterFn, QuillPlugin, StyleHandle, View, ViewHandle,
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
    let selected_entity = cx.use_local(|| Option::<Entity>::None);

    Element::new()
        .children((
            scene_tree_panel.bind(selected_entity.clone()),
            entity_inspector_panel.bind(selected_entity.get()),
        ))
        .styled(StyleHandle::build(|s| s.flex_direction(FlexDirection::Row)))
}

fn scene_tree_panel(mut cx: Cx<LocalData<Option<Entity>>>) -> impl View {
    let on_click_set_selected_entity = {
        let selected_entity = cx.props.clone();
        move |v: Option<Entity>| {
            let selected_entity = selected_entity.clone();
            move |mut e: EntityWorldMut| {
                let mut selected_entity = selected_entity.clone();
                e.insert(On::<Pointer<Click>>::run(move || selected_entity.set(v)));
            }
        }
    };

    let entity_widget = {
        let selected_entity = cx.props.clone();
        let on_click_set_selected_entity = on_click_set_selected_entity.clone();

        move |(entity, name): &(Entity, String)| {
            name.clone()
                .styled(StyleHandle::build(|s| {
                    if Some(*entity) == selected_entity.get() {
                        s.background_color(Color::PURPLE)
                    } else {
                        s
                    }
                }))
                .once(on_click_set_selected_entity(Some(*entity)))
        }
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

fn entity_inspector_panel(cx: Cx<Option<Entity>>) -> impl View {
    If::new(
        cx.props.is_some(),
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

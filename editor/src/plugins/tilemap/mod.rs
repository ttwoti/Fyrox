use crate::{
    fyrox::{
        core::{algebra::Vector2, pool::Handle, type_traits::prelude::*, Uuid},
        engine::Engine,
        graph::{BaseSceneGraph, SceneGraphNode},
        gui::{BuildContext, UiNode},
        scene::{node::Node, tilemap::TileMap},
    },
    interaction::{make_interaction_mode_button, InteractionMode},
    plugin::EditorPlugin,
    scene::{controller::SceneController, GameScene, Selection},
    settings::Settings,
    Editor, Message,
};

#[derive(TypeUuidProvider)]
#[type_uuid(id = "33fa8ef9-a29c-45d4-a493-79571edd870a")]
pub struct TileMapInteractionMode {
    #[allow(dead_code)]
    tile_map: Handle<Node>,
}

impl InteractionMode for TileMapInteractionMode {
    fn on_left_mouse_button_down(
        &mut self,
        _editor_selection: &Selection,
        _controller: &mut dyn SceneController,
        _engine: &mut Engine,
        _mouse_pos: Vector2<f32>,
        _frame_size: Vector2<f32>,
        _settings: &Settings,
    ) {
        // TODO
    }

    fn on_left_mouse_button_up(
        &mut self,
        _editor_selection: &Selection,
        _controller: &mut dyn SceneController,
        _engine: &mut Engine,
        _mouse_pos: Vector2<f32>,
        _frame_size: Vector2<f32>,
        _settings: &Settings,
    ) {
        // TODO
    }

    fn on_mouse_move(
        &mut self,
        _mouse_offset: Vector2<f32>,
        _mouse_position: Vector2<f32>,
        _editor_selection: &Selection,
        _controller: &mut dyn SceneController,
        _engine: &mut Engine,
        _frame_size: Vector2<f32>,
        _settings: &Settings,
    ) {
        // TODO
    }

    fn deactivate(&mut self, _controller: &dyn SceneController, _engine: &mut Engine) {
        // TODO
    }

    fn make_button(&mut self, ctx: &mut BuildContext, selected: bool) -> Handle<UiNode> {
        make_interaction_mode_button(
            ctx,
            include_bytes!("../../../resources/tile.png"),
            "Edit Tile Map",
            selected,
        )
    }

    fn uuid(&self) -> Uuid {
        Self::type_uuid()
    }
}

#[derive(Default)]
pub struct TileMapEditorPlugin {}

impl EditorPlugin for TileMapEditorPlugin {
    fn on_message(&mut self, message: &Message, editor: &mut Editor) {
        let Some(entry) = editor.scenes.current_scene_entry_mut() else {
            return;
        };

        let Some(selection) = entry.selection.as_graph() else {
            return;
        };

        let Some(game_scene) = entry.controller.downcast_mut::<GameScene>() else {
            return;
        };

        let scene = &mut editor.engine.scenes[game_scene.scene];

        if let Message::SelectionChanged { .. } = message {
            entry
                .interaction_modes
                .remove_typed::<TileMapInteractionMode>();

            for node_handle in selection.nodes().iter() {
                if let Some(collider) = scene.graph.try_get(*node_handle) {
                    if collider.component_ref::<TileMap>().is_none() {
                        continue;
                    }

                    entry.interaction_modes.add(TileMapInteractionMode {
                        tile_map: *node_handle,
                    });

                    break;
                }
            }
        }
    }
}

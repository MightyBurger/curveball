// Copyright 2025 Jordan Johnson
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Thanks to an anonymous Reddit user (they deleted their account) for this block of code to
// work around a Bevy egui problem where clicks would register both for the UI and the
// main screen. I've made a small change to avoid panics on program close.
// https://www.reddit.com/r/bevy/comments/vbll6b/comment/kh2odd7/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

use bevy::prelude::*;
use bevy_egui::EguiContexts;

pub struct EguiBlockingPlugin;

#[derive(Default, Resource)]

pub struct EguiBlockInputState {
    pub wants_keyboard_input: bool,
    pub wants_pointer_input: bool,
}

impl Plugin for EguiBlockingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiBlockInputState>()
            .add_systems(PostUpdate, egui_wants_input);
    }
}

fn egui_wants_input(mut state: ResMut<EguiBlockInputState>, mut contexts: EguiContexts) {
    if let Some(ctx) = contexts.try_ctx_mut() {
        state.wants_keyboard_input = ctx.wants_keyboard_input();
        state.wants_pointer_input = ctx.wants_pointer_input();
    }
}

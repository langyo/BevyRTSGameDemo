use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub(crate) struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.run_unless_resource_exists::<TextProps>());
    }
}

/// Resource handling text properties throughout the app.
#[derive(Resource)]
pub struct TextProps(Handle<Font>);

impl TextProps {
    pub(crate) fn button_text_style(&self) -> TextStyle {
        TextStyle {
            font: self.font(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        }
    }

    fn font(&self) -> Handle<Font> {
        self.0.clone()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Fira_Mono/FiraMono-Medium.ttf");
    commands.insert_resource(TextProps(font));
}

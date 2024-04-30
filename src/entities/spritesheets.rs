use bevy::prelude::*;

/// Add this plugin to load the images and spritesheets
pub struct JoustSpriteSheetPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum SpriteSheetPluginState {
    Loading,
}

impl Plugin for JoustSpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                load_rider_assets,
                load_bird_assets,
                load_parrot_assets,
                load_spurt_assets,
                load_platform_assets,
                load_pter_asset,
                load_egg_asset,
                load_pop_asset,
                load_crushed_asset,
            ).in_set(SpriteSheetPluginState::Loading)
        );
    }
}

fn load_tex_atlas<'a, 'b, T: Component>(
    commands: &mut Commands<'b, 'a>,
    asset_server: &mut Res<AssetServer>,
    assets: &mut ResMut<Assets<TextureAtlas>>,
    tile_size: Vec2,
    rows: usize,
    path: &str,
    component: T,
) -> Entity {
    let tex_handle = asset_server.load(&format!("spritesheets/{}", path));
    let atlas = TextureAtlas::from_grid(tex_handle, tile_size, 1, rows, None, None);
    let atlas_handle = assets.add(atlas);
    println!("Loading asset {}", path);
    commands.spawn((atlas_handle, component)).id()
}

fn load_rider_assets(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    use rider_kinds::*;

    let tile_size = Vec2::new(20.0, 20.0);
    let rows = 7;

    let id = load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        tile_size,
        rows,
        "blue.png",
        BlueRider,
    );
    commands.entity(id).insert(RiderTextureAtlas);
    let id = load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        tile_size,
        rows,
        "grey.png",
        GreyRider,
    );
    commands.entity(id).insert(RiderTextureAtlas);
    let id = load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        tile_size,
        rows,
        "indigo.png",
        IndigoRider,
    );
    commands.entity(id).insert(RiderTextureAtlas);
    let id = load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        tile_size,
        rows,
        "yellow.png",
        YellowRider,
    );
    commands.entity(id).insert(RiderTextureAtlas);
    let id = load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        tile_size,
        rows,
        "red.png",
        RedRider,
    );
    commands.entity(id).insert(RiderTextureAtlas);
}

fn load_bird_assets(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(18.0, 18.0),
        7,
        "bird.png",
        BirdTextureAtlas,
    );
}

fn load_parrot_assets(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(20.0, 20.0),
        4,
        "parrot.png",
        ParrotTextureAtlas,
    );
}

fn load_spurt_assets(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(20.0, 20.0),
        5,
        "spurt.png",
        SpurtTextureAtlas,
    );
}

fn load_platform_assets(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    use platform_kinds::*;
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(500.0, 61.0),
        1,
        "platform_bottom.png",
        PlatformBottom,
    );

    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(150.0, 16.0),
        1,
        "platform_medium.png",
        PlatformMedium,
    );
}

fn load_pter_asset(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(30.0, 12.0),
        3,
        "pter.png",
        PterTextureAtlas,
    );
}

fn load_egg_asset(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(18.0, 18.0),
        7,
        "egg.png",
        EggTextureAtlas,
    );
}

fn load_pop_asset(
    mut commands: Commands, 
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(10.0,10.0),
        9,
        "pop.png",
        PopTextureAtlas,
    );
}

fn load_crushed_asset(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut assets: ResMut<Assets<TextureAtlas>>,
) {
    load_tex_atlas(
        &mut commands,
        &mut asset_server,
        &mut assets,
        Vec2::new(18.0,18.0),
        5,
        "crushed.png",
        CrushedTextureAtlas,
    );
}

#[derive(Component, Copy, Clone)]
pub struct PterTextureAtlas;

#[derive(Component, Copy, Clone)]
pub struct SpurtTextureAtlas;

#[derive(Component, Copy, Clone)]
/// Indicates that this sprite sheet is a bird sprite sheet, and follows the bird sprite sheet layout convention
pub struct BirdTextureAtlas;

#[derive(Component, Copy, Clone)]
pub struct ParrotTextureAtlas;

#[derive(Component, Copy, Clone)]
/// Indicates that this sprite sheet is a rider sprite sheet, and follows the rider sprite sheet layout convention
pub struct RiderTextureAtlas;

#[derive(Component, Copy, Clone)]
pub struct EggTextureAtlas;

#[derive(Component, Copy, Clone)]
pub struct PopTextureAtlas;

/// Crushed Egg texture atlas
#[derive(Component, Copy, Clone)]
pub struct CrushedTextureAtlas;

pub mod platform_kinds {
    use super::*;
    #[derive(Component, Clone, Copy)]
    pub struct PlatformMedium;
    #[derive(Component, Clone, Copy)]
    pub struct PlatformBottom;
}

pub mod rider_kinds {
    use super::*;
    #[derive(Component, Clone, Copy)]
    pub struct RedRider;
    #[derive(Component, Clone, Copy)]
    pub struct GreyRider;
    #[derive(Component, Clone, Copy)]
    pub struct YellowRider;
    #[derive(Component, Clone, Copy)]
    pub struct IndigoRider;
    #[derive(Component, Clone, Copy)]
    pub struct BlueRider;
}

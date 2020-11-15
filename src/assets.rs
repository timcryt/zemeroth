//! TODO: Document the motivation behind this module.

use std::collections::HashMap;

use macroquad::{
    text::{self, Font},
    texture::{load_texture, Texture2D},
};
use once_cell::sync::OnceCell;

use crate::{
    core::{
        battle::{
            ability::Ability,
            component::{ObjType, Prototypes},
            scenario::Scenario,
        },
        campaign,
    },
    sprite_info::SpriteInfo,
    utils::{self, deserialize_from_file},
    ZResult,
};

static INSTANCE: OnceCell<Assets> = OnceCell::new();

pub async fn load_assets() {
    let assets = Assets::load().await.expect("TODO: err msg (important)");
    INSTANCE.set(assets).expect("TODO: err msg");
}

pub fn get() -> &'static Assets {
    INSTANCE.get().expect("TODO: err msg")
}

type SpritesInfo = HashMap<ObjType, SpriteInfo>;

#[derive(Debug)]
pub struct Assets {
    pub images: Images,
    pub font: Font,

    pub sprites_info: SpritesInfo,
    pub sprite_frames: HashMap<ObjType, HashMap<String, Texture2D>>,

    // TODO: core configs
    // TODO: visual configs
    pub prototypes: Prototypes,
    pub demo_scenario: Scenario,
    pub campaign_plan: campaign::Plan,
    pub agent_campaign_info: HashMap<ObjType, campaign::AgentInfo>,
}

impl Assets {
    pub async fn load() -> ZResult<Self> {
        let images = Images::load().await;
        let font = text::load_ttf_font("assets/OpenSans-Regular.ttf").await;
        let sprites_info: SpritesInfo = deserialize_from_file("assets/sprites.ron").await?;
        let sprite_frames = {
            let mut sprite_frames = HashMap::new();
            for (obj_type, SpriteInfo { paths, .. }) in sprites_info.iter() {
                let mut frames = HashMap::new();
                for (frame_name, path) in paths {
                    frames.insert(frame_name.to_string(), load_texture(path).await);
                }
                sprite_frames.insert(obj_type.clone(), frames);
            }
            sprite_frames
        };
        let prototypes = Prototypes::from_str(&utils::read_file("assets/objects.ron").await?);
        let demo_scenario = deserialize_from_file("assets/scenario_01.ron").await?;
        let campaign_plan = deserialize_from_file("assets/campaign_01.ron").await?;
        let agent_campaign_info = deserialize_from_file("assets/agent_campaign_info.ron").await?;
        Ok(Self {
            images,
            font,
            sprites_info,
            sprite_frames,
            prototypes,
            demo_scenario,
            campaign_plan,
            agent_campaign_info,
        })
    }
}

// TODO: rename to Textures?
#[derive(Debug)]
pub struct Images {
    pub selection: Texture2D,
    pub white_hex: Texture2D,
    pub tile: Texture2D,
    pub tile_rocks: Texture2D,
    pub grass: Texture2D,
    pub dot: Texture2D,
    pub blood: Texture2D,
    pub explosion_ground_mark: Texture2D,
    pub shadow: Texture2D,

    pub ability_icons: HashMap<Ability, Texture2D>,

    pub attack_slash: Texture2D,
    pub attack_smash: Texture2D,
    pub attack_pierce: Texture2D,
    pub attack_claws: Texture2D,

    // TODO: HashMap<Effect, Texture2D>
    pub effect_stun: Texture2D,
    pub effect_poison: Texture2D,
    pub effect_bloodlust: Texture2D,

    // TODO: Extract to Icons struct
    pub icon_info: Texture2D,
    pub icon_end_turn: Texture2D,
    pub icon_main_menu: Texture2D,
}

impl Images {
    pub async fn load() -> Self {
        let ability_icons = load_ability_icons().await;
        Self {
            selection: load_texture("assets/img/selection.png").await,
            white_hex: load_texture("assets/img/white_hex.png").await,
            tile: load_texture("assets/img/tile.png").await,
            tile_rocks: load_texture("assets/img/tile_rocks.png").await,
            grass: load_texture("assets/img/grass.png").await,
            dot: load_texture("assets/img/dot.png").await,
            blood: load_texture("assets/img/blood.png").await,
            explosion_ground_mark: load_texture("assets/img/explosion_ground_mark.png").await,
            shadow: load_texture("assets/img/shadow.png").await,
            ability_icons,

            attack_slash: load_texture("assets/img/slash.png").await,
            attack_smash: load_texture("assets/img/smash.png").await,
            attack_pierce: load_texture("assets/img/pierce.png").await,
            attack_claws: load_texture("assets/img/claw.png").await,

            effect_stun: load_texture("assets/img/effect_stun.png").await,
            effect_poison: load_texture("assets/img/effect_poison.png").await,
            effect_bloodlust: load_texture("assets/img/effect_bloodlust.png").await,

            icon_info: load_texture("assets/img/icon_info.png").await,
            icon_end_turn: load_texture("assets/img/icon_end_turn.png").await,
            icon_main_menu: load_texture("assets/img/icon_menu.png").await,
        }
    }
}

pub async fn load_ability_icons() -> HashMap<Ability, Texture2D> {
    let mut map = HashMap::new();
    for (ref ability, name) in &[
        (Ability::Knockback, "knockback"),
        (Ability::Club, "club"),
        (Ability::Jump, "jump"),
        (Ability::LongJump, "long_jump"),
        (Ability::Bomb, "bomb"),
        (Ability::BombPush, "bomb_push"),
        (Ability::BombFire, "bomb_fire"),
        (Ability::BombPoison, "bomb_poison"),
        (Ability::BombDemonic, "bomb_demonic"),
        (Ability::Summon, "summon"),
        (Ability::Dash, "dash"),
        (Ability::Rage, "rage"),
        (Ability::Heal, "heal"),
        (Ability::GreatHeal, "great_heal"),
        (Ability::Bloodlust, "bloodlust"),
    ] {
        let texture = load_texture(&format!("assets/img/icon_ability_{}.png", name)).await;
        map.insert(*ability, texture);
    }
    map
}
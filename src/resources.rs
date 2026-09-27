use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{Color, Resource, Timer};

#[derive(Resource)]
pub struct ObstacleSpawningTimer(pub Timer);

#[derive(Resource)]
pub struct RealTimer(pub Timer);

#[derive(Resource)]
pub struct ScoreOffset(pub f32);

/// Set while a run is suspended in the menu, so that (P) resumes instead of restarting.
#[derive(Resource, Default)]
pub struct RunInProgress(pub bool);

/// The dino sprite sets shipped in the assets folder.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DinoColor {
    #[default]
    Purple,
    Green,
    Red,
    Turquoise,
}

impl DinoColor {
    /// The next color to offer in the settings menu, wrapping back to purple.
    pub fn next(self) -> Self {
        match self {
            Self::Purple => Self::Green,
            Self::Green => Self::Red,
            Self::Red => Self::Turquoise,
            Self::Turquoise => Self::Purple,
        }
    }

    /// Where this color's sprite sheet for `action` lives, e.g. `purple_trex_run.png`.
    pub fn sprite_path(self, action: &str) -> String {
        let name = match self {
            Self::Purple => "purple",
            Self::Green => "green",
            Self::Red => "red",
            Self::Turquoise => "turquoise",
        };
        format!("{name}_trex_{action}.png")
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Purple => "PURPLE",
            Self::Green => "GREEN",
            Self::Red => "RED",
            Self::Turquoise => "TURQUOISE",
        }
    }

    /// Roughly the dino's own color, to tint the settings menu entry with.
    pub fn swatch(self) -> Color {
        match self {
            Self::Purple => Color::srgb(0.6, 0.27, 0.68),
            Self::Green => Color::srgb(0.47, 0.71, 0.16),
            Self::Red => Color::srgb(0.88, 0.32, 0.26),
            Self::Turquoise => Color::srgb(0.13, 0.73, 0.42),
        }
    }
}

#[derive(Resource)]
pub struct GameSettings {
    /// Whether the game gradually gets faster the longer a run lasts.
    pub speed_up_over_time: bool,
    pub dino_color: DinoColor,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            speed_up_over_time: true,
            dino_color: DinoColor::default(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct HealthPickUpImg(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct PterodactylFly(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct PterodactylDie(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct CactusTexture(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoRun(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoDuck(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoJump(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct DinoDie(pub Handle<Image>);
#[cfg(test)]
mod tests {
    use super::*;

    /// Guards the color cycle and the sprite file names it builds: a typo in
    /// either only shows up as a missing texture at runtime.
    #[test]
    fn every_dino_color_cycles_and_has_its_sprite_sheets() {
        let mut color = DinoColor::default();

        for _ in 0..4 {
            for action in ["run", "duck", "jump", "die"] {
                let path = format!(
                    "{}/static/assets/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    color.sprite_path(action)
                );
                assert!(std::path::Path::new(&path).exists(), "missing {path}");
            }
            color = color.next();
        }

        assert_eq!(
            color,
            DinoColor::default(),
            "next() should cycle through all four colors"
        );
    }
}

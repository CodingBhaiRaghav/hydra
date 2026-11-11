use serde::{Deserialize, Serialize};

/// `src/types/game.types.ts` `UnlockedAchievement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UnlockedAchievement {
    pub name: String,
    pub unlock_time: i64,
    pub hardcore_unlock_time: Option<i64>,
    pub image_key: Option<String>,
}

/// `src/types/game.types.ts` `AchievementSet`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AchievementSet {
    pub id: i64,
    pub name: Option<String>,
    pub order: i64,
}

/// `src/types/game.types.ts` `SteamAchievement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SteamAchievement {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub icon: String,
    pub icongray: String,
    pub hidden: bool,
    pub points: Option<i64>,
    pub global_unlock_percentage: Option<f64>,
    pub set: Option<AchievementSet>,
    pub external_set_id: Option<String>,
    pub external_id: Option<String>,
}

/// `UserAchievement["source"]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AchievementSource {
    Steam,
    Epic,
    Retroachievements,
}

/// One shop's contribution to a merged achievement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementShopSource {
    pub shop: crate::GameShop,
    pub object_id: String,
    pub name: String,
    pub unlock_time: i64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AchievementUnlockedOn {
    pub hydra: bool,
    pub retroachievements: bool,
    pub steam: bool,
    pub epic: Option<bool>,
}

/// `src/types/game.types.ts` `UserAchievement`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct UserAchievement {
    #[serde(flatten)]
    pub achievement: SteamAchievement,
    pub unlocked: bool,
    pub unlock_time: Option<i64>,
    pub hardcore_unlock_time: Option<i64>,
    pub source: Option<AchievementSource>,
    pub canonical_achievement_id: Option<String>,
    pub source_object_id: Option<String>,
    pub sources: Option<Vec<AchievementShopSource>>,
    pub unlocked_on: Option<AchievementUnlockedOn>,
    pub image_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_achievement_flattens_steam_fields() {
        let json = serde_json::json!({
            "name": "ACH_WIN",
            "displayName": "Winner",
            "icon": "i.png",
            "icongray": "g.png",
            "hidden": false,
            "unlocked": true,
            "unlockTime": 1_700_000_000
        });

        let achievement: UserAchievement = serde_json::from_value(json).unwrap();
        assert_eq!(achievement.achievement.display_name, "Winner");
        assert!(achievement.unlocked);
        assert_eq!(achievement.unlock_time, Some(1_700_000_000));
    }

    #[test]
    fn defaults_fill_missing_optional_fields() {
        let achievement: SteamAchievement = serde_json::from_str("{}").unwrap();
        assert_eq!(achievement.hidden, false);
        assert_eq!(achievement.points, None);
    }
}

use serde::{Deserialize, Serialize};

use crate::IsoDateTime;

/// `src/types/theme.types.ts` `Theme`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Theme {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    pub is_active: bool,
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_custom_sound: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_sound_path: Option<String>,
    pub created_at: IsoDateTime,
    pub updated_at: IsoDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_round_trips() {
        let theme = Theme {
            id: "t1".into(),
            name: "Neon".into(),
            is_active: true,
            code: ":root{}".into(),
            created_at: "2026-01-01T00:00:00.000Z".into(),
            updated_at: "2026-01-01T00:00:00.000Z".into(),
            ..Theme::default()
        };

        let json = serde_json::to_string(&theme).unwrap();
        assert_eq!(serde_json::from_str::<Theme>(&json).unwrap(), theme);
    }
}

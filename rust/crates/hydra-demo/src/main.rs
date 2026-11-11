//! Runnable tour of the ported crates: `cargo run -p hydra-demo`.

use hydra_hosters::PixelDrainApi;
use hydra_shared::{
    format_bit_rate, format_bytes, get_displayed_play_time_in_milliseconds,
    get_play_time_hours_and_minutes, trim_trailing_slashes, PlayTimeParts,
};
use hydra_store::{GamesSublevel, LevelDb};
use hydra_types::{Game, GameShop};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // src/shared/playtime.ts
    let play_time = PlayTimeParts {
        play_time_in_milliseconds: Some(3_600_000),
        steam_play_time_in_milliseconds: Some(1_800_000),
        epic_play_time_in_milliseconds: None,
    };
    let total = get_displayed_play_time_in_milliseconds(play_time);
    let (hours, minutes) = get_play_time_hours_and_minutes(total);
    println!("playtime        : {hours}h {minutes}m ({total} ms)");

    // src/shared/format-bit-rate.ts, src/shared/index.ts
    println!("download speed  : {}", format_bit_rate(1_250_000.0));
    println!("download size   : {}", format_bytes(1_610_612_736.0));
    println!(
        "trimmed api url : {}",
        trim_trailing_slashes("https://api.example.test///")
    );

    // src/main/services/hosters/pixeldrain.ts
    println!(
        "pixeldrain id   : {}",
        PixelDrainApi::extract_id("https://pixeldrain.com/u/abc123")?
    );

    // src/main/level/*
    let directory = std::env::temp_dir().join(format!("hydra-demo-{}", std::process::id()));
    let db = LevelDb::open(directory.join("level"))?;
    let games = GamesSublevel::new(&db);

    games.put(&Game {
        shop: GameShop::Steam,
        object_id: "504230".into(),
        title: "Celeste".into(),
        play_time_in_milliseconds: total,
        is_deleted: false,
        ..Game::default()
    })?;

    let stored = games
        .get(GameShop::Steam, "504230")?
        .ok_or("game was not stored")?;
    println!("stored game     : {} ({})", stored.title, stored.object_id);
    println!("library count   : {}", games.count()?);
    println!("database at     : {}", directory.join("level").display());

    std::fs::remove_dir_all(&directory).ok();
    Ok(())
}

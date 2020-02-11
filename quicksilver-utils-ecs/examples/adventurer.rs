// Sprite Sheet Info

// PNG image - width: 416, height: 512
// Sprites - 16 rows, 13 columns
// Each sprite - 32x32 pixels

use log::info;
use platter::load_file;
use quicksilver::{
    graphics::{Graphics, Image, PixelFormat},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};
use std::collections::HashMap;

enum Orientation {
    Left,
    Right,
}

#[derive(Eq, Hash, PartialEq)]
enum Animation {
    Idle,
    Run,
    SlashUp,
    SlashDown,
    SlashForward,
    Jump,
    Hit,
    Faint,
}

fn frames() -> HashMap<Animation, Vec<u32>> {
    let mut dat = HashMap::new();
    dat.insert(Animation::Idle, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::Run, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashUp, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashDown, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::SlashForward, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::Jump, vec![1, 1, 1, 1, 1, 1]);
    dat.insert(Animation::Hit, vec![1, 1, 1, 1]);
    dat.insert(Animation::Faint, vec![1, 1, 1, 1, 1, 1, 1]);
    dat
}

fn main() {
    let settings = Settings::default();
    run(settings, app)
}

async fn app(window: Window, gfx: Graphics, mut event_stream: EventStream) -> Result<()> {
    let sprite_sheet_data = load_file("test_assets/sprite_sheet.png").await?;
    // let sprite_image = Image::from_raw(&gfx, Some(&sprite_sheet), 416, 512, PixelFormat::RGBA)?;
    let sprite_image = Image::from_encoded_bytes(&gfx, &sprite_sheet_data);

    'main: loop {
        info!("In the loop");
        break 'main;
    }

    Ok(())
}

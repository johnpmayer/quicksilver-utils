use log::{debug, trace};
use platter::load_file;
use quicksilver::{
    geom::Vector,
    graphics::{FontRenderer, Graphics, Image, VectorFont},
    input::Input,
    run, Result, Settings, Window,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;

use monk::{
    background::BackgroundRender, dialog::*, global::Global, hud::HudRender, interact::*, room::*,
};

fn main() {
    let mut settings = Settings::default();
    settings.size = Vector::new(800., 600.);
    run(settings, app)
}

async fn app(window: Window, gfx: Graphics, input: Input) -> Result<()> {
    let characters_data = load_file("monk_characters.png").await?;
    let characters_image: Image = Image::from_encoded_bytes(&gfx, &characters_data)?;

    let bed_data = load_file("bed.png").await?;
    let bed_image: Image = Image::from_encoded_bytes(&gfx, &bed_data)?;

    let desk_data = load_file("desk.png").await?;
    let desk_image: Image = Image::from_encoded_bytes(&gfx, &desk_data)?;

    let font_data =
        VectorFont::load("Kingthings-Calligraphica/Kingthings_Calligraphica_2.ttf").await?;
    let font: FontRenderer = font_data.to_renderer(&gfx, 36.0)?;

    // Note: these are camera photos of a drawing
    // I had to downsample it to 800x600 pixels, otherwise it was too large for quicksilver to handle
    let bedroom_data = load_file("bedroom.png").await?;
    let bedroom_image: Image = Image::from_encoded_bytes(&gfx, &bedroom_data)?;

    let hall_data = load_file("hall.png").await?;
    let hall_image: Image = Image::from_encoded_bytes(&gfx, &hall_data)?;

    let cellar_data = load_file("cellar.png").await?;
    let cellar_image: Image = Image::from_encoded_bytes(&gfx, &cellar_data)?;

    let garden_data = load_file("garden.png").await?;
    let garden_image: Image = Image::from_encoded_bytes(&gfx, &garden_data)?;

    let room_data = RoomData {
        characters_spritesheet: characters_image,
        bedroom_background: bedroom_image,
        bedroom_bed_sprite: bed_image,
        bedroom_desk_sprite: desk_image,
        hall_background: hall_image,
        cellar_background: cellar_image,
        garden_background: garden_image,
    };

    debug!("Loaded resources");

    let mut world = World::new();

    world.insert(RenderContext {
        gfx: SendWrapper::new(gfx),
        window: SendWrapper::new(window),
    });

    world.insert(InputContext {
        input: SendWrapper::new(input),
    });

    let now = instant::now();

    world.insert(TimeContext { now });

    world.register::<Position>();
    world.register::<SpriteConfig>();
    world.register::<PlayerInputFlag>();
    world.register::<PlayerInteract>();
    world.register::<ObjectInteract>();

    debug!("Registered types");

    debug!("attempt to insert a Global");
    world.insert(Global::new(font, Room::Bedroom));
    let mut sprite_system = RenderSprites;
    let mut move_system = WasdMovement;
    let mut interaction_system = InteractionSystem::new();
    let mut hud_render_system = HudRender; // we could inject the font here instead of the Global resource...
    let mut dialog_render_system = DialogRender;
    let mut background_render_system = BackgroundRender;
    let room_system = RoomSystem {
        room_data: SendWrapper::new(room_data),
    };

    room_system.setup_new_room(&mut world);

    debug!("Entering main loop");

    loop {
        let now: f64 = instant::now();
        *world.write_resource::<TimeContext>() = TimeContext { now };

        trace!("In the loop");

        {
            let ctx = world.get_mut::<InputContext>().expect("has input context");
            let input: &mut Input = &mut ctx.input;
            while let Some(ev) = input.next_event().await {
                debug!("Quicksilver event: {:?}", ev);
            }
        }

        move_system.run_now(&world);
        interaction_system.run_now(&world);

        room_system.setup_new_room(&mut world);

        background_render_system.run_now(&world);
        sprite_system.run_now(&world);
        hud_render_system.run_now(&world);
        dialog_render_system.run_now(&world);

        {
            let ctx = world
                .get_mut::<RenderContext>()
                .expect("has render context");
            ctx.gfx.present(&ctx.window).expect("present")
        }
    }
}

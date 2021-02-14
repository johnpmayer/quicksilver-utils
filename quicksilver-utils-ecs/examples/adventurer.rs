// Sprite Sheet Info

// PNG image - width: 416, height: 512
// Sprites - 16 rows, 13 columns
// Each sprite - 32x32 pixels

use log::{debug, info, trace, Level};
use mergui::{channels::BasicClickable, widgets::ButtonConfig, Context as Gui, FontStyle, MFont};
use platter::load_file;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image, VectorFont},
    input::{Input, Key},
    run, Result, Settings, Window,
};
use quicksilver_utils_ecs::*;
use send_wrapper::SendWrapper;
use specs::prelude::*;
use std::fmt;

use std::collections::HashMap;

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

// Number of milliseconds to display the frame
fn frames() -> HashMap<Animation, Vec<u32>> {
    let mut dat = HashMap::new();
    dat.insert(
        Animation::Idle,
        vec![
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        ],
    );
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
    let mut settings = Settings::default();
    settings.log_level = Level::Debug;
    run(settings, app)
}

struct PauseMenuChannels {
    // noop: SendWrapper<BasicClickable>,
    quit: SendWrapper<BasicClickable>,
}

impl fmt::Debug for PauseMenuChannels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PauseMenuChannels").finish()
    }
}

#[derive(Debug)]
enum AdventurerMode {
    Play,
    PauseMenu(PauseMenuChannels),
}

impl Default for AdventurerMode {
    fn default() -> Self {
        AdventurerMode::Play
    }
}

// enum PauseOption {
//     Nothing,
//     Quit,
// }

// const PAUSE_MENU_OPTIONS: [PauseOption; 2] = [PauseOption::Nothing, PauseOption::Quit];

struct AdventurerKeybindingConfig {
    play_open_menu: Key,
    pause_menu_close_menu: Key,
}

const ADVENTURER_KEYBINDING: AdventurerKeybindingConfig = AdventurerKeybindingConfig {
    play_open_menu: Key::Escape,
    pause_menu_close_menu: Key::Escape,
};

struct ModeSystem {
    keybindings: AdventurerKeybindingConfig,
}

impl<'a> System<'a> for ModeSystem {
    type SystemData = (
        Write<'a, InputContext>,
        Write<'a, AdventurerMode>,
        Write<'a, GuiContext>,
    );

    fn run(
        &mut self,
        (mut input_ctx_resource, mut mode_resource, mut gui_ctx_resource): Self::SystemData,
    ) {
        let input_ctx: &mut InputContext = &mut input_ctx_resource;
        let input: &mut Input = &mut input_ctx.input;

        let gui_ctx: &mut GuiContext = &mut gui_ctx_resource;
        let background: &Image = &gui_ctx.background;
        let font: &MFont = &gui_ctx.font;

        let current_mode: &mut AdventurerMode = &mut mode_resource;
        let mut next_mode: Option<AdventurerMode> = None;

        match current_mode {
            AdventurerMode::Play => {
                if input.key_down(self.keybindings.play_open_menu) {
                    // TODO: all of this is sort of "config"
                    let mut gui = Gui::new();
                    let mut layer = gui.add_layer();

                    debug!("Active layer? {}", layer.get_active());

                    let quit_button = ButtonConfig {
                        background: background.clone(),
                        background_location: Rectangle::new(
                            Vector::new(100., 50.),
                            Vector::new(100., 50.),
                        ),
                        blend_color: Some(Color::GREEN),
                        hover_color: Some(Color::YELLOW),
                        font_style: FontStyle {
                            font: font.clone(),
                            location: Vector::new(20., 30.),
                            color: Color::WHITE,
                        },
                        text: "Quit".into(),
                    };

                    let quit_response = layer.add_widget(quit_button);

                    let channels = PauseMenuChannels {
                        quit: SendWrapper::new(quit_response.channel),
                    };

                    next_mode = Some(AdventurerMode::PauseMenu(channels));
                    gui_ctx.gui = SendWrapper::new(gui);
                    info!("Set new gui for pause menu");
                }
            }
            AdventurerMode::PauseMenu(pause_channels) => {
                if input.key_down(self.keybindings.pause_menu_close_menu) {
                    next_mode = Some(AdventurerMode::Play);
                    gui_ctx.gui = SendWrapper::new(Gui::new());
                    debug!("Cleared gui, exiting pause menu");
                }

                if pause_channels.quit.has_clicked() {
                    unimplemented!("Hard Quit... todo graceful")
                }
            }
        }
        if let Some(next_mode) = next_mode {
            debug!("Switching mode: {:?}", next_mode);
            *current_mode = next_mode;
        }
    }
}

struct GuiContext {
    pub gui: SendWrapper<Gui>,
    pub background: SendWrapper<Image>,
    pub font: SendWrapper<MFont>,
}

impl Default for GuiContext {
    fn default() -> Self {
        panic!("must be injected...")
    }
}

async fn app(window: Window, gfx: Graphics, input: Input) -> Result<()> {
    let sprite_sheet_data = load_file("sprite_sheet.png").await?;
    // let sprite_image = Image::from_raw(&gfx, Some(&sprite_sheet), 416, 512, PixelFormat::RGBA)?;
    let sprite_image: Image = Image::from_encoded_bytes(&gfx, &sprite_sheet_data)?;

    // For mergui
    let button_blank_image: Image = Image::load(&gfx, "100x50-5050b4ff.png").await?;
    let button_base_font = VectorFont::load("font.ttf").await?;
    let button_font = MFont::from_font(&button_base_font, &gfx, 15.0)?;

    debug!("Got the image");

    let mut world = World::new();

    world.insert(RenderContext {
        gfx: SendWrapper::new(gfx),
        window: SendWrapper::new(window),
    });

    world.insert(InputContext {
        input: SendWrapper::new(input),
    });

    world.insert(GuiContext {
        gui: SendWrapper::new(Gui::new()),
        background: SendWrapper::new(button_blank_image),
        font: SendWrapper::new(button_font),
    });

    let now = instant::now();

    world.insert(TimeContext { now });

    world.insert(AdventurerMode::Play);

    world.register::<Position>();
    world.register::<SpriteConfig>();
    world.register::<PlayerInputFlag>();

    // Create the player
    let player_sprite = SpriteConfig {
        image: SendWrapper::new(sprite_image),
        row: 0,
        width: 32,
        height: 32,
        scale: 2.,
        animation: Some(AnimationConfig {
            loop_start_time: now,
            frames: frames()
                .get(&Animation::Idle)
                .expect("frames for idle animation")
                .clone(),
        }),
    };

    world
        .create_entity()
        .with(Position { x: 0., y: 0. })
        .with(player_sprite)
        .with(PlayerInputFlag)
        .build();

    debug!("Created world, components, and entities");

    let mut sprite_system = RenderSprites;
    let mut move_system = WasdMovement;
    let mut mode_system = ModeSystem {
        keybindings: ADVENTURER_KEYBINDING,
    };

    debug!("Entering main loop");

    loop {
        let now: f64 = instant::now();
        *world.write_resource::<TimeContext>() = TimeContext { now };

        {
            let ctx = world
                .get_mut::<RenderContext>()
                .expect("has render context");
            ctx.gfx.clear(Color::from_rgba(200, 200, 200, 1.));
        }

        trace!("In the loop");

        {
            let mut input_ctx = world.fetch_mut::<InputContext>();
            let input: &mut Input = &mut input_ctx.input;
            let mut gui_ctx = world.fetch_mut::<GuiContext>();
            let gui: &mut Gui = &mut gui_ctx.gui;
            let mut render_ctx = world.fetch_mut::<RenderContext>();
            let window: &mut Window = &mut render_ctx.window;
            while let Some(ev) = input.next_event().await {
                gui.event(&ev, &window);
                trace!("Quicksilver event: {:?}", ev);
            }
        }

        mode_system.run_now(&world);
        sprite_system.run_now(&world);

        {
            // TODO: render gui should be a system
            let current_mode: &AdventurerMode = &*world.fetch::<AdventurerMode>();
            debug!("Mode: {:?}.", current_mode);
            match current_mode {
                AdventurerMode::Play => move_system.run_now(&world),
                AdventurerMode::PauseMenu(_) => {
                    debug!("Render gui");
                    let mut gui_ctx = world.fetch_mut::<GuiContext>();
                    let gui: &mut Gui = &mut gui_ctx.gui;
                    let render_ctx: &mut RenderContext = &mut world.fetch_mut::<RenderContext>();
                    gui.render(&mut render_ctx.gfx, &render_ctx.window)?;
                }
            }
        }

        {
            let ctx = world
                .get_mut::<RenderContext>()
                .expect("has render context");
            ctx.gfx.present(&ctx.window).expect("present");
        }
    }
}

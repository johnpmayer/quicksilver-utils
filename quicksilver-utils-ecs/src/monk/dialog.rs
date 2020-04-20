
use specs::prelude::*;
use super::{global::*, room::*};
use crate::*;
use quicksilver::{geom::Rectangle, graphics::Color};
use log::warn;

#[derive(Clone, Copy)]
pub enum Dialog {
    Welcome,
    Greet, // Eventually write custom greetings for each monk... they should suggest who to write to
    SleepConfirm,
    DelegateWheat,
    PendingDelegateWheat,
    NoWheatToBake,
    DelegateBake,
    PendingDelegateBake,
    NoBreadToGive,
    GiveBread,
    ThanksForBread,
    Uninspired,
    DelegatePaper,
    PendingDelegatePaper,
    OldDesk,
    WriteLetter,
    NoMorePaper,
    LearnAboutEastPurpose, // Artisan
    LearnAboutNorthBeer, // Baker
    LearnAboutSouthHops, // Gardener
    LearnAboutInvitingGuests,
    ReplyPurpose,
    ReplyBeer,
    ReplyHops,
    ReplyGuests,
    DelegateHops,
    PendingDelegateHops,
    DelegateBeer,
    PendingDelegateBeer,
    King,
}

// TODO rename, it's just "recipient" now
#[derive(Debug, Clone, Copy)]
enum Monestary { Northern, Southern, Eastern, King }

fn letter_options(progress: &GameProgression) -> Vec<Monestary> {
    let mut letters = Vec::new();
    if progress.know_northern_monestary && !progress.sent_northern_monestary {
        letters.push(Monestary::Northern)
    }
    if progress.know_southern_monestary && !progress.sent_southern_monestary {
        letters.push(Monestary::Southern)
    }
    if progress.know_eastern_monestary && !progress.sent_eastern_monestary {
        letters.push(Monestary::Eastern)
    }
    if progress.know_invite && !progress.sent_invite {
        letters.push(Monestary::King)
    }
    letters
}

impl Dialog {
    fn text(&self, progress: &GameProgression) -> String {
        match self {
            Dialog::Welcome => 
                "Farewell Brother,

                I leave you in charge of this place, the Western Monestary. \
                Over the years, as my health declined, I failed in its upkeep, and \
                this brotherhood has forgotten its purpose. It's up to you to restore this \
                place, and keep our traditions alive!

                (Enter)".to_string(),
            Dialog::Greet => 
                "Hello Brother!

                (Enter)".to_string(),
            Dialog::SleepConfirm => 
                "Are you sure you want to go to sleep?

                (Y/N)".to_string(),
            Dialog::DelegateWheat => 
                "Hello Brother! Shall I begin planting wheat?

                (Y/N)".to_string(),
            Dialog::PendingDelegateWheat => 
                "I'll start growing tomorrow!
                
                (Enter)".to_string(),
            Dialog::NoWheatToBake => 
                "Hello Brother! We have nothing to bake...
                
                (Enter)".to_string(),
            Dialog::DelegateBake => 
                "Hello Brother! Shall I begin baking bread?
                
                (Y,N)".to_string(),
            Dialog::DelegateHops =>
                "Hello Brother! Shall I begin planting hops?
                
                (Y/N)".to_string(),
            Dialog::DelegateBeer =>
                "Hello Brother! Shall I begin brewing beer?
                
                (Y/N)".to_string(),
            Dialog::PendingDelegateBake => 
                "I'll start baking tomorrow!
                
                (Enter)".to_string(),
            Dialog::PendingDelegateHops => 
                "I'll start planting tomorrow!
                
                (Enter)".to_string(),
            Dialog::PendingDelegateBeer => 
                "I'll start brewing tomorrow!
                
                (Enter)".to_string(),
            Dialog::NoBreadToGive => 
                "So hungry...

                (Enter)".to_string(),
            Dialog::GiveBread => 
                "Mmmh, I smell fresh bread! May I have some?

                (Y/N)".to_string(),
            Dialog::ThanksForBread => 
                "Mmmh, delicious!

                (Enter)".to_string(),
            Dialog::Uninspired => 
                "It's peaceful here, but what is there to do?

                (Enter)".to_string(),
            Dialog::DelegatePaper => 
                "Your generosity inspired and reminded me! Our order is dedicated to serving \
                others. But it was more than just bread... We must contact other monestaries! \
                Shall I begin making paper?

                (Y/N)".to_string(),
            Dialog::PendingDelegatePaper => 
                "I'll start making paper tomorrow!

                (Enter)".to_string(),
            Dialog::OldDesk =>
                "An old desk... it hasn't been used for years.
                
                (Enter)".to_string(),
            Dialog::WriteLetter => {
                let letters = letter_options(progress);

                if letters.is_empty() {
                    "Nobody to write to.
                    
                    (Enter)".to_string()
                } else {
                    let options: Vec<String> = letters.iter().enumerate()
                    .map(|(i, mon)| format!("({}): Write to the {:?} monestary.\n", i + 1, mon)).collect();      
                    format!(
                        "{}
                        (Enter): Write to nobody.", options.join("\n"))
                }
            },
            Dialog::NoMorePaper =>
                "The desk papers have been used up. There will be more tomorrow.
                
                (Enter)".to_string(),
            Dialog::LearnAboutEastPurpose =>
                "Our brothers in the Eastern monestary are keepers of knowledge. Perhaps they \
                can remind us of our ancient traditions.
                
                (Enter)".to_string(),
            Dialog::LearnAboutNorthBeer =>
                "I once knew some brothers from the Northern monestary. They are masters of food \
                and drink! Perhaps they're familiar with 'beer'.
                
                (Enter)".to_string(),
            Dialog::LearnAboutSouthHops =>
                "I've heard that our brothers in the Southern monestary have studied the art of \
                gardening for centuries. Maybe they can send us some seeds for 'hops'.
                
                (Enter)".to_string(),
            Dialog::LearnAboutInvitingGuests =>
                "Now that we have beer, we should share it with the world. It is amazing! We \
                must write to the king!
                
                (Enter)".to_string(),
            Dialog::ReplyPurpose =>
                "Your brother is correct, our tradition is to serve others. Specifically, we \
                brew beer!
                - The Eastern Monestary
                
                ... but who can make beer?

                (Enter)".to_string(),
            Dialog::ReplyBeer =>
                "1: Make and cool the wort with water, grains, and hops
                2: Ferment for 2 weeks
                - The Northern Monestary

                ... but we have no hops!

                (Enter)".to_string(),
            Dialog::ReplyHops =>
                "Enclosed are some Hop Rhizomes. Plant horizontally in early Spring.
                - The Southern Monestary
                
                ... it feels like we're close!

                (Enter)".to_string(),
            Dialog::ReplyGuests =>
                "Beer? My, erm, advisors have advised me to come at once!
                - His Majesty, the King
                
                (Enter)".to_string(),
            Dialog::King => 
                "Beer! You have kept the tradition of the this monestary alive!
                
                Congratulations! You have completed the game!

                (Enter)".to_string()
        }
    }

    pub fn process(&self, global: &mut Global, event_cache: &EventCache) -> bool {
        let mut should_close = false;

        match self {
            Dialog::SleepConfirm => {
                if event_cache.key(Key::Y) {
                    if global.progress.delegated_wheat {
                        global.progress.growing_wheat = true
                    }
                    
                    if global.progress.delegated_baking {
                        global.progress.baking_bread = true
                    }

                    if global.progress.gave_to_charity {
                        global.progress.charity_inspiration = true
                    }

                    let had_paper = global.progress.making_paper;

                    if global.progress.delegated_papermaking {
                        global.progress.making_paper = true
                    }

                    if global.progress.making_paper && !had_paper {
                        // relod the desk
                        global.pending_room = Some(Room::Bedroom) 
                    }

                    if global.progress.making_paper {
                        global.progress.has_paper_today = true
                    }

                    let mut expecting_reply = false;

                    if global.progress.sent_eastern_monestary && !global.progress.reply_eastern_purpose {
                        global.progress.reply_eastern_purpose = true;
                        global.dialog = Some(Dialog::ReplyPurpose);
                        expecting_reply = true
                    }

                    if global.progress.sent_northern_monestary && !global.progress.reply_northern_beer {
                        global.progress.reply_northern_beer = true;
                        global.dialog = Some(Dialog::ReplyBeer);
                        expecting_reply = true
                    }

                    if global.progress.sent_southern_monestary && !global.progress.reply_southern_hops {
                        global.progress.reply_southern_hops = true;
                        global.dialog = Some(Dialog::ReplyHops);
                        expecting_reply = true
                    }

                    if global.progress.sent_invite && !global.progress.guests {
                        global.progress.guests = true;
                        global.dialog = Some(Dialog::ReplyGuests);
                        expecting_reply = true
                    }

                    if global.progress.delegated_hops {
                        global.progress.growing_hops = true
                    }

                    if global.progress.delegated_beer {
                        global.progress.brewing_beer = true
                    }

                    should_close = !expecting_reply
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateWheat => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_wheat = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateBake => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_baking = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::GiveBread => {
                if event_cache.key(Key::Y) {
                    global.progress.gave_to_charity = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegatePaper => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_papermaking = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateHops => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_hops = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::DelegateBeer => {
                if event_cache.key(Key::Y) {
                    global.progress.delegated_beer = true;
                    should_close = true
                } else if event_cache.key(Key::N) {
                    should_close = true
                }
            }
            Dialog::LearnAboutEastPurpose => {
                if event_cache.key(Key::Return) {
                    global.progress.know_eastern_monestary = true;
                    should_close = true
                }
            }
            Dialog::LearnAboutNorthBeer => {
                if event_cache.key(Key::Return) {
                    global.progress.know_northern_monestary = true;
                    should_close = true
                }
            }
            Dialog::LearnAboutSouthHops => {
                if event_cache.key(Key::Return) {
                    global.progress.know_southern_monestary = true;
                    should_close = true
                }
            }
            Dialog::LearnAboutInvitingGuests => {
                if event_cache.key(Key::Return) {
                    global.progress.know_invite = true;
                    should_close = true
                }
            }
            Dialog::WriteLetter => {
                let letters = letter_options(&global.progress);

                // Hack, there should only be one letter option at once... should change the type
                if letters.len() > 1 {
                    warn!("Can send multiple letters...")
                }
                if letters.len() > 0 {
                    if event_cache.key(Key::Key1) {
                        let recipient = letters[0];
                        match recipient {
                            Monestary::Eastern => global.progress.sent_eastern_monestary = true,
                            Monestary::Northern => global.progress.sent_northern_monestary = true,
                            Monestary::Southern => global.progress.sent_southern_monestary = true,
                            Monestary::King => global.progress.sent_invite = true
                        }
                        should_close = true
                    }
                }

                if event_cache.key(Key::Return) {
                    should_close = true
                }
            }
            _ => {
                if event_cache.key(Key::Return) {
                    should_close = true
                }
            }
        }

        should_close
    }
}

pub struct DialogRender;

impl<'a> System<'a> for DialogRender {
    type SystemData = (Write<'a, Global>, Write<'a, RenderContext>,);

    fn run(&mut self, (mut global, mut render_ctx_resource): Self::SystemData) {
        let ctx: &mut RenderContext = &mut render_ctx_resource;
        if let Some(dialog) = global.dialog {
            let popup_area = Rectangle::new((100., 100.,), (600., 400.,));
            ctx.gfx.fill_rect(&popup_area, Color::from_rgba(200, 200, 200, 0.9));

            let text_area = Rectangle::new((120., 120.,), (560., 360.,));
            let text = dialog.text(&global.progress);
            global.font.draw_wrapping(&mut ctx.gfx, &text, Some(text_area.size.x), Color::BLACK, text_area.pos).expect("draw text");
        }
    }
}
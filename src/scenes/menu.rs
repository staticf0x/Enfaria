use crate::scenes::{GameScene, OptionsScene, Scene, SceneSwitch, Scenes};
use crate::world::GameWorld;
use egui::*;
use log::info;
use tetra::{Context, Event};

#[derive(Debug, Default)]
pub struct MenuScene {
    play_clicked: bool,
    options_clicked: bool,
    quit_clicked: bool,
}

impl MenuScene {
    pub fn new(_world: &mut GameWorld, _ctx: &mut Context) -> Self {
        MenuScene::default()
    }
}

impl Scene for MenuScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.play_clicked {
            let scene = GameScene::new(world, ctx);
            return Ok(SceneSwitch::Push(Scenes::Game(scene)));
        };

        if self.options_clicked {
            let scene = OptionsScene::new(world, ctx);
            return Ok(SceneSwitch::Push(Scenes::Options(scene)));
        };

        if self.quit_clicked {
            tetra::window::quit(ctx);
        };

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, _world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        let size = tetra::window::get_size(ctx);
        Window::new("Enfaria")
            .title_bar(true)
            .collapsible(false)
            .resizable(false)
            .fixed_pos([(size.0 / 2 - 100) as f32, (size.1 / 2 - 100) as f32])
            .fixed_size([200.0, 200.0])
            .show(ectx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let play = ui.add(Button::new("Play"));
                    if play.clicked() {
                        info!("Clicked play");
                        self.play_clicked = true;
                    };

                    let options = ui.add(Button::new("Options"));
                    if options.clicked() {
                        info!("Options clicked");
                        self.options_clicked = true;
                    }

                    let quit = ui.add(Button::new("Quit"));
                    if quit.clicked() {
                        info!("Quit clicked");
                        self.quit_clicked = true;
                    }
                });
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, _event: Event) -> tetra::Result {
        Ok(())
    }
}

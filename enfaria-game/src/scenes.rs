use crate::scenes::menu::MenuScene;
use crate::world::GameWorld;
use egui::CtxRef;
use tetra::{Context, Event};

pub mod menu;

// Holds the stack of scenes and takes care of actual logic.
pub struct SceneStack {
    world: GameWorld,
    scenes: Vec<Scenes>,
}

impl SceneStack {
    pub fn new(world: GameWorld, initial: Scenes) -> Self {
        Self {
            world,
            scenes: vec![initial],
        }
    }

    fn push(&mut self, scene: Scenes) {
        self.scenes.push(scene)
    }

    fn pop(&mut self) -> Scenes {
        self.scenes.pop().expect("Tried to pop an empty scene stack.")
    }

    // Recursively pass through all scenes that allow drawing previous.
    // Then draw them in order from oldest to newest.
    fn draw_scenes(scenes: &mut [Scenes], world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) {
        assert!(!scenes.is_empty());
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, world, ctx, ectx);
            }
            current.draw(world, ctx, ectx).expect("Failed to draw a scene.");
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, ectx: &mut CtxRef) {
        SceneStack::draw_scenes(&mut self.scenes, &mut self.world, ctx, ectx)
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let change = {
            let current_scene = self.scenes.last_mut().expect("Tried to update empty scene stack.");
            current_scene.update(&mut self.world, ctx)
        };
        match change {
            SceneSwitch::None => {}
            SceneSwitch::Push(s) => self.push(s),
            SceneSwitch::Pop => {
                let _ = self.pop();
            }
            SceneSwitch::Replace(s) => {
                let _ = self.pop();
                self.push(s);
            }
        };
    }

    pub fn event(&mut self, ctx: &mut Context, event: Event) {
        let current_scene = self.scenes.last_mut().expect("Tried to do input for empty scene stack");
        current_scene.event(&mut self.world, ctx, event);
    }
}

// This is what you have to implement for a new scene.
pub trait Scene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> SceneSwitch;
    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result;
    fn event(&mut self, world: &mut GameWorld, ctx: &mut Context, event: Event);
    fn draw_previous(&self) -> bool {
        false
    }
}

// The result of calling scene.update()
pub enum SceneSwitch {
    None,
    Push(Scenes),
    Replace(Scenes),
    Pop,
}

// This is a little boilerplate to avoid dynamic dispatch.
pub enum Scenes {
    Menu(MenuScene),
}

// Failing to add a new scene here will result in a compilation error.
impl Scene for Scenes {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> SceneSwitch {
        match self {
            Scenes::Menu(s) => s.update(world, ctx),
        }
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        match self {
            Scenes::Menu(s) => s.draw(world, ctx, ectx),
        }
    }

    fn event(&mut self, world: &mut GameWorld, ctx: &mut Context, event: Event) {
        match self {
            Scenes::Menu(s) => s.event(world, ctx, event),
        }
    }
}

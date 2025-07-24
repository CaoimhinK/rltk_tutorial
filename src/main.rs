use rltk::{GameState, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;

mod rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        draw_map(&self.ecs, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn register_structs(ecs: &mut World) {
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Viewshed>();
}

fn create_entities(ecs: &mut World) {
    let map = Map::new_map_rooms_and_corridors();

    let (player_x, player_y) = map.rooms[0].center();
    
    ecs.insert(map);

    ecs.create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed { visible_tiles: Vec::new(), range: 8 })
        .build();
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };

    register_structs(&mut gs.ecs);

    create_entities(&mut gs.ecs);

    rltk::main_loop(context, gs)
}

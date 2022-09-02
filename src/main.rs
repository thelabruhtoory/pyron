use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::FullscreenType::True;
use glam::Vec2;

struct MyGame {
    player: graphics::Image,
    enemy: graphics::Image,
    a10: graphics::Image,
    bullet: graphics::Image,
    
    player_pos: Vec2,
    enemy_pos: Vec2,
    a10_pos: Vec2,
    bullet_pos: Vec2,
    
    player_speed: f32,
    enemy_speed: f32,
    a10_speed: f32,
    bullet_speed: f32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let player = graphics::Image::new(ctx, "/images/player.png")?;
        let enemy = graphics::Image::new(ctx, "/images/enemy.png")?;
        let a10 = graphics::Image::new(ctx, "/images/a10.png")?;
        let bullet = graphics::Image::new(ctx, "/images/bullet.png")?;

        let mut player_pos = Vec2::new(0.0, 0.0);
        let mut enemy_pos = Vec2::new(0.0, 0.0);
        let mut a10_pos = Vec2::new(0.0, 0.0);
        let mut bullet_pos = Vec2::new(0.0, 0.0);

        let player_speed = 0.0;
        let enemy_speed = 0.0;
        let a10_speed = 0.0;
        let bullet_speed = 0.0;

        let g = MyGame { 
            player, 
            enemy, 
            a10, 
            bullet,
            player_pos,
            enemy_pos,
            a10_pos,
            bullet_pos,
            player_speed,
            enemy_speed,
            a10_speed,
            bullet_speed,
        };
        Ok(g)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        
        let mut player_dest = Vec2::new(20.0, 10.0);
        let mut enemy_dest = Vec2::new(200.0, 200.0);
        let mut enemy_dest2 = Vec2::new(300.0, 300.0);
        let mut a10_dest = Vec2::new(450.0, 400.0);
        let mut bullet_dest = Vec2::new(130.0, 5.0);

        graphics::clear(ctx, graphics::Color::BLACK);
        graphics::draw(ctx, &self.player, DrawParam::default().dest(player_dest))?;
        graphics::draw(ctx, &self.enemy, DrawParam::default().dest(enemy_dest))?;
        graphics::draw(ctx, &self.enemy, DrawParam::default().dest(enemy_dest2))?;
        graphics::draw(ctx, &self.a10, DrawParam::default().dest(a10_dest))?;
        graphics::draw(ctx, &self.bullet, DrawParam::default().dest(bullet_dest))?;

        graphics::set_window_title(ctx, "pyron");
        graphics::present(ctx)
    }
}

fn main() {
    let window_mode = WindowMode::default()
        .dimensions(3840.0, 2160.0)
        .resizable(true);
    let (mut ctx, event_loop) = ContextBuilder::new("pyron", "author")
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");
    let my_game = MyGame::new(&mut ctx).unwrap();
    event::run(ctx, event_loop, my_game)
}
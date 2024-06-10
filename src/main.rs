pub use bracket_lib::prelude::*;
use obstacle::Obstacle;
use player::Player;

mod player;
mod obstacle;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT:i32 = 50;
const FRAME_DURATION:f32 = 75.0;

struct State {
    mode: GamdMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
    
}

impl State {
    fn new() ->Self {
        State{
            mode: GamdMode::Menu, //游戏的初始状态
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score:0,
        }
    }

    fn play(&mut self,ctx: &mut BTerm){
        ctx.cls_bg(BLUE1);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION{
            self.frame_time = 0.0;
            self.player.gracity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.flap();
        }
        self.player.renders(ctx);
        ctx.print(0, 0, "Press Space to Fly");

        ctx.print(0, 1, &format!("Score: {}",self.score));

        self.obstacle.render(ctx, self.player.x);
        if  self.player.x >self.obstacle.x {
            self.score += 1;

            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);

        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player){
            self.mode = GamdMode::End;
        }
        
    } 
    
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "Welcome to Flappy Cube!!! XD");
        ctx.print_centered(10, "(P) Play the game");
        ctx.print_centered(12, "(Q) Quit the game");

        if let Some(key) = ctx.key{
            match key { 
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
                
            }
        }
    }
    
    fn restart(&mut self){
        self.mode = GamdMode::Playing;
        self.player = Player::new(5,25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(7, "YOU DEAD!!!");
        ctx.print_centered(8, &format!("you earned {} points",self.score));
        ctx.print_centered(10, "(P) Play the game");
        ctx.print_centered(12, "(Q) Quit the game");

        if let Some(key) = ctx.key{
            match key { 
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
                
            }
        }
    }
}

enum GamdMode {  //枚举游戏的三种状态
    Menu,
    Playing,
    End,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GamdMode::Menu => self.main_menu(ctx),
            GamdMode::End => self.dead(ctx),
            GamdMode::Playing => self.play(ctx),
            
        }
    }
}



fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Cube")
        .build()?;

    main_loop(context, State::new())
}
 
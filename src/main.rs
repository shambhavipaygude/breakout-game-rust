use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::input::keyboard;
use ggez::{Context, ContextBuilder, GameResult};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_SIZE: f32 = 20.0;
const PADDLE_SPEED: f32 = 600.0; 
const BALL_SPEED: f32 = 300.0;
const BLOCK_WIDTH: f32 = 75.0; 
const BLOCK_HEIGHT: f32 = 25.0; 
const BLOCK_SPACING: f32 = 5.0; 

struct Block {
    x: f32,
    y: f32,
    color: Color,
    points: u32,
}

struct GameState {
    paddle_pos: f32,
    ball_pos: (f32, f32),
    ball_vel: (f32, f32),
    blocks: Vec<Block>,
    score: u32,
    high_score: u32, 
    game_over: bool,
}

impl GameState {
    pub fn new() -> GameResult<GameState> {
        let mut blocks = Vec::new();
        let colors = [
            (Color::from_rgb(255, 0, 0), 20),    // Red
            (Color::from_rgb(255, 165, 0), 10),  // Orange
            (Color::from_rgb(255, 255, 0), 5),   // Yellow
            (Color::from_rgb(0, 128, 0), 5),     // Green
            (Color::from_rgb(0, 0, 255), 10),    // Blue
            (Color::from_rgb(75, 0, 130), 20),   // Indigo
        ];

        for (i, &(color, points)) in colors.iter().enumerate() {
            for col in 0..(SCREEN_WIDTH / (BLOCK_WIDTH + BLOCK_SPACING)) as usize {
                let x = col as f32 * (BLOCK_WIDTH + BLOCK_SPACING);
                let y = (i as f32 * (BLOCK_HEIGHT + 5.0)) + 50.0; 
                blocks.push(Block { x, y, color, points });
            }
        }

        let s = GameState {
            paddle_pos: (SCREEN_WIDTH - PADDLE_WIDTH) / 2.0,
            ball_pos: (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            ball_vel: (BALL_SPEED, BALL_SPEED),
            blocks,
            score: 0,
            high_score: 0, 
            game_over: false,
        };
        Ok(s)
    }

    pub fn reset(&mut self) {
        self.paddle_pos = (SCREEN_WIDTH - PADDLE_WIDTH) / 2.0;
        self.ball_pos = (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.ball_vel = (BALL_SPEED, BALL_SPEED);
        if self.score > self.high_score {
            self.high_score = self.score; 
        }
        self.score = 0;
        self.game_over = false;

        let colors = [
            (Color::from_rgb(255, 0, 0), 20),    // Red
            (Color::from_rgb(255, 165, 0), 10),  // Orange
            (Color::from_rgb(255, 255, 0), 5),   // Yellow
            (Color::from_rgb(0, 128, 0), 5),     // Green
            (Color::from_rgb(0, 0, 255), 10),    // Blue
            (Color::from_rgb(75, 0, 130), 20),   // Indigo
        ];

        self.blocks.clear();
        for (i, &(color, points)) in colors.iter().enumerate() {
            for col in 0..(SCREEN_WIDTH / (BLOCK_WIDTH + BLOCK_SPACING)) as usize {
                let x = col as f32 * (BLOCK_WIDTH + BLOCK_SPACING);
                let y = (i as f32 * (BLOCK_HEIGHT + 5.0)) + 50.0; 
                self.blocks.push(Block { x, y, color, points });
            }
        }
    }
}

impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            if keyboard::is_key_pressed(ctx, KeyCode::Return) {
                self.reset();
            }
            return Ok(());
        }

        // Paddle movement
        if keyboard::is_key_pressed(ctx, KeyCode::Left) && self.paddle_pos > 0.0 {
            self.paddle_pos -= PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) && self.paddle_pos < SCREEN_WIDTH - PADDLE_WIDTH {
            self.paddle_pos += PADDLE_SPEED * ggez::timer::delta(ctx).as_secs_f32();
        }

        // Ball movement
        self.ball_pos.0 += self.ball_vel.0 * ggez::timer::delta(ctx).as_secs_f32();
        self.ball_pos.1 += self.ball_vel.1 * ggez::timer::delta(ctx).as_secs_f32();

        // Ball collision with screen edges
        if self.ball_pos.0 <= 0.0 || self.ball_pos.0 >= SCREEN_WIDTH - BALL_SIZE {
            self.ball_vel.0 = -self.ball_vel.0;
        }
        if self.ball_pos.1 <= 0.0 {
            self.ball_vel.1 = -self.ball_vel.1;
        }

        // Ball collision with paddle
        if self.ball_pos.1 >= SCREEN_HEIGHT - PADDLE_HEIGHT - BALL_SIZE
            && self.ball_pos.0 + BALL_SIZE >= self.paddle_pos
            && self.ball_pos.0 <= self.paddle_pos + PADDLE_WIDTH
        {
            self.ball_vel.1 = -self.ball_vel.1;
        }

        // Ball goes out of bounds (end game)
        if self.ball_pos.1 >= SCREEN_HEIGHT {
            self.game_over = true;
        }

        // Ball collision with blocks
        self.blocks.retain(|block| {
            let hit = self.ball_pos.0 + BALL_SIZE >= block.x
                && self.ball_pos.0 <= block.x + BLOCK_WIDTH
                && self.ball_pos.1 + BALL_SIZE >= block.y
                && self.ball_pos.1 <= block.y + BLOCK_HEIGHT;

            if hit {
                self.score += block.points;
                self.ball_vel.1 = -self.ball_vel.1;
            }
            !hit
        });

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        // Draw paddle
        let paddle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.paddle_pos, SCREEN_HEIGHT - PADDLE_HEIGHT, PADDLE_WIDTH, PADDLE_HEIGHT),
            Color::WHITE,
        )?;
        graphics::draw(ctx, &paddle, graphics::DrawParam::default())?;

        // Draw ball
        let ball = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(self.ball_pos.0, self.ball_pos.1, BALL_SIZE, BALL_SIZE),
            Color::WHITE,
        )?;
        graphics::draw(ctx, &ball, graphics::DrawParam::default())?;

        // Draw blocks
        for block in &self.blocks {
            let block_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                graphics::Rect::new(block.x, block.y, BLOCK_WIDTH, BLOCK_HEIGHT),
                block.color,
            )?;
            graphics::draw(ctx, &block_mesh, graphics::DrawParam::default())?;

            // Draw border (transparent)
            let border_mesh = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(2.0),
                graphics::Rect::new(block.x, block.y, BLOCK_WIDTH, BLOCK_HEIGHT),
                Color::from_rgba(0, 0, 0, 0), // Transparent color for border
            )?;
            graphics::draw(ctx, &border_mesh, graphics::DrawParam::default())?;
        }

        // Draw scores
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        let high_score_text = graphics::Text::new(format!("High Score: {}", self.high_score));
        
        let score_pos_x = (SCREEN_WIDTH - score_text.dimensions(ctx).w) / 2.0; // Centered
        let score_pos_y = 10.0;
        graphics::draw(ctx, &score_text, (ggez::mint::Point2 { x: score_pos_x, y: score_pos_y },))?;
        
        let high_score_pos_x = (SCREEN_WIDTH - high_score_text.dimensions(ctx).w) / 2.0; // Centered
        let high_score_pos_y = 30.0;
        graphics::draw(ctx, &high_score_text, (ggez::mint::Point2 { x: high_score_pos_x, y: high_score_pos_y },))?;

        if self.game_over {
            let game_over_text = graphics::Text::new("Game Over! Press Enter to restart.");
            let text_rect = game_over_text.dimensions(ctx);
            let x = (SCREEN_WIDTH - text_rect.w) / 2.0;
            let y = (SCREEN_HEIGHT - text_rect.h) / 2.0;
            graphics::draw(ctx, &game_over_text, (ggez::mint::Point2 { x, y },))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("breakout", "Author")
        .window_setup(ggez::conf::WindowSetup::default().title("Breakout"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}

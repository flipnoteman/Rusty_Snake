use ggez;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics;
use ggez::mint;
use ggez::input::keyboard::{is_key_pressed, KeyCode};
use ggez::graphics::{draw, DrawMode, PxScale};
use ggez::mint::Vector2;
use ggez::timer::check_update_time;
use crate::mint::Point2;
use rand::{self, thread_rng, Rng};

const CELL_SIZE: f32 = 40.0;
const CELL_MULTIPLIER: f32 = 50.0;
const FIELD_SIZE_X: f32 = CELL_SIZE * CELL_MULTIPLIER;
const FIELD_SIZE_Y: f32 = FIELD_SIZE_X - 480.0;


fn check_collision_with_apple(head_pos: &Point2<f32>, body_parts: &mut Vec<BodyPart>, apple_pos: &mut Point2<f32>) -> bool{
    let collide: bool;
    if head_pos.x as i32 == apple_pos.x as i32 && head_pos.y as i32 == apple_pos.y as i32{
        collide = true;
        let mut new_point = Point2{x: gen_rand(FIELD_SIZE_X), y: gen_rand(FIELD_SIZE_Y)};
        while body_touching_apple(&body_parts, &new_point){
            new_point.x = gen_rand(FIELD_SIZE_X);
            new_point.y = gen_rand(FIELD_SIZE_Y);
        }
        apple_pos.x = new_point.x;
        apple_pos.y = new_point.y;
    }else{
        collide = false;
    }
    collide
}

fn body_touching_apple(body: &Vec<BodyPart>, apple_pos: &Point2<f32>) -> bool{
    let mut collided = false;
    for body_part in body {
        if body_part.point == *apple_pos {
            collided = true;
            break;
        }else{
            collided = false;
        }
    }
    collided
}

fn gen_rand(bound: f32) -> f32 {
    let mut rng = thread_rng();
    let num: f32;
    let cell_multiplier_minus_twelve = CELL_MULTIPLIER - 12.0;

    if bound == FIELD_SIZE_Y {
        num = (rng.gen_range(0..cell_multiplier_minus_twelve as i32) * CELL_SIZE as i32) as f32;
    }else{
        num = (rng.gen_range(0..(CELL_MULTIPLIER) as i32) * CELL_SIZE as i32) as f32;
    }
    num
}

#[derive(Clone, Copy)]
struct BodyPart {
    point: Point2<f32>,
}

struct Body {
    parts: Vec<BodyPart>,
}

struct MainState {
    snake_head_pos: Point2<f32>,
    apple_pos: Point2<f32>,
    dir: mint::Vector2<f32>,
    body: Body,
    game_over: bool,
}

impl BodyPart{
    fn new(x: &mut f32, y: &mut f32) -> BodyPart {
        BodyPart {
            point: Point2{x: *x, y: *y},
        }
    }
}

impl Body {
    fn new() -> Body {
        let vec: Vec<BodyPart> = vec![];
        Body{
            parts: vec,
        }
    }
}

fn ate_itself(body: &Body, head_pos: Point2<f32>, game_state: &mut bool){
    for seg in &body.parts{
        if seg.point == head_pos {
            *game_state = true;
        }
    }

}

impl MainState {
    pub fn new() -> MainState{
        let starting_pos = Point2{x: 4.0 * CELL_SIZE, y: 5.0 * CELL_SIZE};
        MainState{
            snake_head_pos: starting_pos,
            apple_pos: Point2{x: gen_rand(FIELD_SIZE_X), y: gen_rand(FIELD_SIZE_Y)},
            dir: Vector2{ x: (1.0), y: (0.0) },
            body: Body::new(),
            game_over: false,
        }
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self, ctx: &mut Context) -> GameResult<> {
        while check_update_time(ctx, 16) {
            if !self.game_over {
                if is_key_pressed(ctx, KeyCode::Up) && self.dir.y != 1.0 {
                    self.dir = mint::Vector2 { x: (0.0), y: (-1.0) };
                }
                if is_key_pressed(ctx, KeyCode::Down) && self.dir.y != -1.0 && !(is_key_pressed(ctx, KeyCode::Right) || is_key_pressed(ctx, KeyCode::Left)) {
                    self.dir = mint::Vector2 { x: (0.0), y: (1.0) };
                }
                if is_key_pressed(ctx, KeyCode::Left) && self.dir.x != 1.0  && !(is_key_pressed(ctx, KeyCode::Up) && (is_key_pressed(ctx, KeyCode::Up))){
                    self.dir = mint::Vector2 { x: (-1.0), y: (0.0) };
                }
                if is_key_pressed(ctx, KeyCode::Right) && self.dir.x != -1.0 && !(is_key_pressed(ctx, KeyCode::Up) && (is_key_pressed(ctx, KeyCode::Up))){
                    self.dir = mint::Vector2 { x: (1.0), y: (0.0) };
                }

                let new_body_part: BodyPart = BodyPart::new(&mut self.snake_head_pos.x, &mut self.snake_head_pos.y);
                self.body.parts.insert(0, new_body_part);
                self.body.parts[0].point = self.snake_head_pos;

                self.snake_head_pos.x += self.dir.x * CELL_SIZE;
                self.snake_head_pos.y += self.dir.y * CELL_SIZE;


                if self.snake_head_pos.x < 0.0 {
                    self.snake_head_pos.x = FIELD_SIZE_X - CELL_SIZE;
                }
                if self.snake_head_pos.x > FIELD_SIZE_X - CELL_SIZE {
                    self.snake_head_pos.x = 0.0;
                }
                if self.snake_head_pos.y < 0.0 {
                    self.snake_head_pos.y = FIELD_SIZE_Y - CELL_SIZE;
                }
                if self.snake_head_pos.y > FIELD_SIZE_Y - CELL_SIZE {
                    self.snake_head_pos.y = 0.0;
                }

                self.body.parts.pop();

                let collided = check_collision_with_apple(&self.snake_head_pos, &mut self.body.parts, &mut self.apple_pos);
                if collided {
                    self.body.parts.push(new_body_part);
                }

                ate_itself(&self.body, self.snake_head_pos, &mut self.game_over);
            } else {
                let screen_w_half = graphics::drawable_size(ctx).0 * 0.5;
                let screen_h = graphics::drawable_size(ctx).1;
                if (ggez::input::mouse::position(ctx).x > 700.0 && ggez::input::mouse::position(ctx).x < 1300.0)
                    && (ggez::input::mouse::position(ctx).y < 1200.0 && ggez::input::mouse::position(ctx).y > 1000.0)
                && ggez::input::mouse::button_pressed(ctx, MouseButton::Left){
                    self.body.parts.clear();
                    self.snake_head_pos = Point2{x: screen_w_half * 2.0, y: screen_h * 0.5};
                    self.apple_pos = Point2{x: gen_rand(FIELD_SIZE_X), y: gen_rand(FIELD_SIZE_Y)};
                    self.dir = Vector2{ x: (1.0), y: (0.0) };
                    self.game_over = false;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<>{
        if !self.game_over {
            graphics::clear(ctx, graphics::Color::BLACK);

            let draw_param = graphics::DrawParam::default();

            // Head Drawing
            let head_rect = graphics::Rect::new(self.snake_head_pos.x, self.snake_head_pos.y, CELL_SIZE, CELL_SIZE);
            let head_mesh = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                head_rect,
                graphics::Color::GREEN)?;

            // Body Drawing

            for body_part in &self.body.parts {
                let part_rect = graphics::Rect::new(body_part.point.x, body_part.point.y, CELL_SIZE, CELL_SIZE);
                let part_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    part_rect,
                    graphics::Color::GREEN
                )?;
                draw_param.dest::<Point2<f32>>(body_part.point.into());
                draw(ctx, &part_mesh, draw_param)?;
            }

            // Apple Drawing
            let apple_rect = graphics::Rect::new(self.apple_pos.x, self.apple_pos.y, CELL_SIZE, CELL_SIZE);
            let apple_mesh = graphics::Mesh::new_rectangle(ctx,
                                                           DrawMode::fill(),
                                                           apple_rect,
                                                           graphics::Color::RED)?;

            draw_param.dest::<Point2<f32>>(self.snake_head_pos.into());
            draw(ctx, &head_mesh, draw_param)?;
            draw_param.dest::<Point2<f32>>(self.apple_pos.into());
            draw(ctx, &apple_mesh, draw_param)?;

            graphics::present(ctx)?;
        }else{
            graphics::clear(ctx, graphics::Color::BLACK);

            let color = graphics::Color::from_rgb_u32(200);

            let wobble_string = "Rusty Snake";
            let mut wobble = graphics::Text::default();
            for ch in wobble_string.chars(){
                wobble.add(
                    graphics::TextFragment::new(ch).scale(graphics::PxScale::from(200.0))
                );
            }
            let wobble_width = wobble.width(ctx);
            graphics::queue_text(
                ctx,
                &wobble,
                Vector2{x: 0.0, y: 0.0},
                Some(color)
            );
            let death_message = "You have eaten yourself!";
            let mut death_message_text = graphics::Text::default();
            for ch in death_message.chars(){
                death_message_text.add(
                  graphics::TextFragment::new(ch).scale(graphics::PxScale::from(50.0))
                );
            }
            graphics::queue_text(
                ctx,
                &death_message_text,
                Vector2{x: death_message_text.width(ctx) * 0.5 - 65.0, y: 250.0},
                Option::from(graphics::Color::RED)
            );

            let button_color: graphics::Color;
            let text_color: graphics::Color;
            if (ggez::input::mouse::position(ctx).x > 700.0 && ggez::input::mouse::position(ctx).x < 1300.0)
                && (ggez::input::mouse::position(ctx).y < 1200.0 && ggez::input::mouse::position(ctx).y > 1000.0) {
                button_color = graphics::Color::from_rgb(50, 50, 50);
                text_color = graphics::Color::from_rgb(100, 100, 100);

            }else{
                button_color = graphics::Color::from_rgb(100, 100, 100);
                text_color = graphics::Color::from_rgb(50, 50, 50);
            }

            let screen_w = graphics::drawable_size(ctx).0;

            let button_w = 600.0;
            let button = graphics::Rect::new(screen_w * 0.5 - button_w * 0.5, 1000.0, button_w, 200.0);
            let button_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), button, button_color)?;
            draw(ctx, &button_mesh, graphics::DrawParam::default())?;

            let reset_text_fragment = graphics::TextFragment::new("Reset").scale(PxScale::from(100.0));
            let reset_text = graphics::Text::new(reset_text_fragment);



            graphics::queue_text(
                ctx,
                &reset_text,
                Vector2{x: 450.0, y: 750.0},
                Option::from(text_color)

            );

            graphics::draw_queued_text(
                ctx,
                graphics::DrawParam::new()
                    .dest(Vector2 { x: graphics::drawable_size(ctx).0 * 0.5 - wobble_width * 0.5, y: 300.0 }),
                None,
                graphics::FilterMode::Nearest,
            )?;
            graphics::present(ctx)?;
            ggez::timer::yield_now();
        }
        Ok(())
    }

}


fn main() {
    let cb = ContextBuilder::new("rusty_snake", "Sam");
    let (mut ctx, game_loop) = cb.build().unwrap();

    let state = MainState::new();


    graphics::set_window_title(&ctx, "Rusty Snake");
    graphics::set_mode(&mut ctx, WindowMode {
        width: FIELD_SIZE_X,
        height: FIELD_SIZE_Y,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        min_height: 0.0,
        max_width: 0.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        resize_on_scale_factor_change: false
    }).expect("PANIC ON THE SET_MODE FOR WINDOW AAAHHHH!!!");
    let screen_rect = graphics::Rect::new(0.0, 0.0, FIELD_SIZE_X, FIELD_SIZE_Y);
    graphics::set_screen_coordinates(&mut ctx, screen_rect).expect("PANIC ON THE SET_SCREEN_COORDINATES!!!");

    event::run(ctx, game_loop, state);
}

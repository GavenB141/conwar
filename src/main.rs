use ggez::*;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::input::mouse::MouseButton;
use ggez::glam::Vec2;
//use rand::Rng;

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
        cells: vec![(25,23),(26,23),(27,23),(31,23),(32,23),(33,23),(23,25),(23,26),(23,27),(28,25),(28,26),(28,27),(30,25),(30,26),(30,27),(35,25),(35,26),(35,27),(25,30),(26,30),(27,30),(31,30),(32,30),(33,30),(25,28),(26,28),(27,28),(31,28),(32,28),(33,28),(23,31),(23,32),(23,33),(28,31),(28,32),(28,33),(30,31),(30,32),(30,33),(35,31),(35,32),(35,33),(25,35),(26,35),(27,35),(31,35),(32,35),(33,35)],
        active: false,
        x:0,
        y:0,
        generation: 1,
        moved: false
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("GoL", "Gaven Behrends")
        .default_conf(c)
        .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

struct State {
    dt: std::time::Duration,
    cells: Vec<(i16, i16)>,
    active: bool,
    x: i16,
    y: i16,
    generation: i32,
    moved: bool
}

impl State {
    fn next_gen(&mut self) -> Vec<(i16, i16)>{
        let mut new:Vec<(i16,i16)> = Vec::new();
        let mut embryos:Vec<(i16,i16)> = Vec::new();

        for cell in &self.cells {
            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    if self.is_alive((cell.0+i, cell.1+j)) {
                        count += 1;
                    }else{
                        let mut is_in = false;
                        for each in &embryos {
                            if *each == (cell.0+i, cell.1+j) {
                                is_in = true;
                            }
                        }
                        if !is_in {embryos.push((cell.0+i, cell.1+j));}
                    }
                }
            }
            if count == 2 || count == 3 {
                new.push(*cell);
            }
        }

        for cell in embryos {
            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    if self.is_alive((cell.0+i, cell.1+j)) {
                        count += 1;
                    }
                }
            }
            if count == 3 {
                new.push(cell);
            }
        }
        self.generation += 1;
        new
    }

    fn toggle(&mut self, cell:(i16, i16)) {
        for i in 0..self.cells.len() {
            if cell == self.cells[i] {
                self.cells.remove(i);
                return;
            }
        }
        self.cells.push(cell);
    }

    fn is_alive(&self, cell:(i16, i16)) -> bool {
        for each in &self.cells {
            if each == &cell {
                return true;
            }
        }
        false
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        while ctx.time.check_update_time(5) {
            if self.active {
                self.cells = self.next_gen();
            }
        }
        if ctx.mouse.button_just_pressed(MouseButton::Left) && !self.active{
            let pos = ctx.mouse.position();
            let x = pos.x as i16/10 - self.x;
            let y = pos.y as i16/10 - self.y;
            self.toggle((x, y));
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        
        for cell in &self.cells {
            if cell.0 >= -self.x && cell.1 >= -self.y && cell.0 < -self.x + 60 && cell.1 < -self.y + 60 {
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(((cell.0 + self.x) * 10).into(), ((cell.1 + self.y) * 10).into(), 9.0, 9.0),
                    graphics::Color::WHITE,
                )?;
            
                canvas.draw(&rect, graphics::DrawParam::default());
            }
        }

        if !self.active {
            let pos = ctx.mouse.position();
            let x = pos.x as i16/10;
            let y = pos.y as i16/10;
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                graphics::Rect::new((x * 10).into(), (y * 10).into(), 10.0, 10.0),
                graphics::Color::WHITE,
            )?;
        
            canvas.draw(&rect, graphics::DrawParam::default());
        }

        //UI
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(600.0, 0.0, 200.0, 600.0),
            graphics::Color::from_rgb(50, 50, 50),
        )?;
    
        canvas.draw(&rect, graphics::DrawParam::default());

        let pos = ctx.mouse.position();
        let x = pos.x as i16/10 - self.x;
        let y = pos.y as i16/10 - self.y;

        let mut text = graphics::Text::new(format!("Generation:\n  {}\n\nPopulation:\n  {}\n\nLocation:\n  ({}, {})", self.generation, self.cells.len(), x, y));
        text.set_bounds(Vec2::new(190.0, 560.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Begin,
            v_align: ggez::graphics::TextAlign::Begin,
        }).set_scale(22.0);
        canvas.draw(&text, graphics::DrawParam::from([605.0, 20.0]).color((255,255,255)));

        if self.x != 0 || self.y != 0 {self.moved = true;}
        let mut text:graphics::Text = graphics::Text::new("");
        if !self.moved {text.add("WASD to move camera\n\n");}
        if self.active {text.add("Press E to Pause");}
        else {text.add("Press E to Resume");}
        text.set_bounds(Vec2::new(190.0, 60.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Middle,
            v_align: ggez::graphics::TextAlign::End,
        }).set_scale(18.0);
        canvas.draw(&text, graphics::DrawParam::from([700.0, 570.0]).color((255,255,255)));

        canvas.finish(ctx)?;
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::E) => self.active = !self.active,
            Some(KeyCode::W) => self.y += 1,
            Some(KeyCode::A) => self.x += 1,
            Some(KeyCode::S) => self.y -= 1,
            Some(KeyCode::D) => self.x -= 1,
            _ => ()
        }
        Ok(())
    }
}
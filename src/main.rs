use ggez::*;
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::input::mouse::MouseButton;
use ggez::glam::Vec2;

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
        cells: vec![(1,4,true),(1,5,true),(36,3,true),(36,2,true),(2,4,true),(2,5,true),(35,2,true),(35,3,true),(14,2,true),(14,8,true),
            (17,4,true),(17,6,true),(22,2,true),(22,4,true),(15,5,true),(16,3,true),(16,7,true),(21,3,true),(18,5,true),(11,5,true),(12,3,true),
            (13,2,true),(12,7,true),(13,8,true),(25,0,true),(25,1,true),(25,5,true),(25,6,true),(17,5,true),(21,2,true),(22,3,true),(23,1,true),
            (21,4,true),(23,5,true),(11,4,true),(11,6,true),(32,22,false),(32,23,false),(32,27,false),(32,28,false),(34,27,false),(34,23,false),
            (35,26,false),(35,24,false),(36,26,false),(36,25,false),(35,25,false),(36,24,false),(39,23,false),(40,23,false),(40,24,false),
            (40,22,false),(41,21,false),(41,25,false),(42,23,false),(43,20,false),(44,20,false),(45,21,false),(46,22,false),(46,23,false),
            (46,24,false),(45,25,false),(44,26,false),(43,26,false),(55,24,false),(56,24,false),(56,23,false),(55,23,false),(22,26,false),
            (21,26,false),(21,25,false),(22,25,false)
        ],
        active: false,
        x:0,
        y:0,
        generation: 1,
        moved: false,
        mode: true
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("GoW", "Gaven Behrends")
        .default_conf(c)
        .window_setup(conf::WindowSetup::default().title("Game of War"))
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}

struct State {
    dt: std::time::Duration,
    cells: Vec<(i16, i16, bool)>,
    active: bool,
    x: i16,
    y: i16,
    generation: i32,
    moved: bool,
    mode: bool
}

impl State {
    fn next_gen(&mut self) -> Vec<(i16, i16, bool)>{
        let mut new:Vec<(i16,i16,bool)> = Vec::new();
        let mut embryos:Vec<(i16,i16)> = Vec::new();

        for cell in &self.cells {
            let mut red = 0;
            let mut blue = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    if self.is_alive((cell.0+i, cell.1+j)).is_some() {
                        if self.cells[self.is_alive((cell.0+i, cell.1+j)).unwrap()].2 {blue += 1;}
                        else {red += 1;}
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
            let (allies, enemies) = match cell.2 {
                true => (blue, red),
                false => (red, blue),
            };

            if enemies == 0 {
                if allies == 2 || allies == 3 {
                    new.push(*cell);
                }
            }else if allies > enemies + 1 {
                new.push(*cell);
            }
        }

        for cell in embryos {
            let mut red = 0;
            let mut blue = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {continue;}
                    if self.is_alive((cell.0+i, cell.1+j)).is_some() {
                        if self.cells[self.is_alive((cell.0+i, cell.1+j)).unwrap()].2 {blue += 1;}
                        else {red += 1;}
                    }
                }
            }
            if red + blue == 3 {
                new.push((cell.0, cell.1, blue > red));
            }
        }
        self.generation += 1;
        new
    }

    fn toggle(&mut self, cell:(i16, i16, bool)) {
        for i in 0..self.cells.len() {
            if (cell.0, cell.1) == (self.cells[i].0, self.cells[i].1) {
                self.cells.remove(i);
                return;
            }
        }
        self.cells.push(cell);
    }

    fn is_alive(&self, cell:(i16, i16)) -> Option<usize> {
        let mut i = 0;
        for each in &self.cells {
            if (each.0, each.1) == cell {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        while ctx.time.check_update_time(5) {
            if self.active && self.cells.len() > 0 {
                self.cells = self.next_gen();
            }
        }
        if ctx.mouse.button_just_pressed(MouseButton::Left) && !self.active{
            let pos = ctx.mouse.position();
            let x = pos.x as i16/10 - self.x;
            let y = pos.y as i16/10 - self.y;
            self.toggle((x, y, self.mode));
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        
        for cell in &self.cells {
            if cell.0 >= -self.x && cell.1 >= -self.y && cell.0 < -self.x + 60 && cell.1 < -self.y + 60 {
                let color:ggez::graphics::Color = match cell.2 {
                    true => graphics::Color::BLUE,
                    false => graphics::Color::RED
                };
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(((cell.0 + self.x) * 10).into(), ((cell.1 + self.y) * 10).into(), 9.0, 9.0),
                    color,
                )?;
            
                canvas.draw(&rect, graphics::DrawParam::default());
            }
        }

        if !self.active {
            let pos = ctx.mouse.position();
            let x = pos.x as i16/10;
            let y = pos.y as i16/10;

            let color:ggez::graphics::Color = match self.mode {
                true => graphics::Color::BLUE,
                false => graphics::Color::RED
            };
            
            let rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                graphics::Rect::new((x * 10).into(), (y * 10).into(), 10.0, 10.0),
                color,
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

        let mut red = 0;
        let mut blue = 0;
        for cell in &self.cells {
            if cell.2 {blue += 1;}
            else {red += 1;}
        }

        let mut text = graphics::Text::new(format!("Generation:\n  {}\n\nLocation:\n  ({}, {})", self.generation, x, y));
        text.set_bounds(Vec2::new(190.0, 560.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Begin,
            v_align: ggez::graphics::TextAlign::Begin,
        }).set_scale(22.0);
        canvas.draw(&text, graphics::DrawParam::from([605.0, 20.0]).color((255,255,255)));

        let mut text = graphics::Text::new(format!("Red:  {}", red));
        text.set_bounds(Vec2::new(190.0, 560.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Begin,
            v_align: ggez::graphics::TextAlign::Begin,
        }).set_scale(22.0);
        canvas.draw(&text, graphics::DrawParam::from([605.0, 220.0]).color((255,0,0)));

        let mut text = graphics::Text::new(format!("Blue: {}", blue));
        text.set_bounds(Vec2::new(190.0, 560.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Begin,
            v_align: ggez::graphics::TextAlign::Begin,
        }).set_scale(22.0);
        canvas.draw(&text, graphics::DrawParam::from([605.0, 260.0]).color((0,128,255)));

        if self.x != 0 || self.y != 0 {self.moved = true;}
        let mut text:graphics::Text = graphics::Text::new("");
        if !self.moved {
            text.add("WASD to move camera\n\n");
        }
        if self.active {
            text.add("Press E to Pause\n\n");
        } else {
            text.add("Press Q to Toggle\n\n"); text.add("Press E to Resume\n\n");
        }
        text.add("Press C to Clear");


        text.set_bounds(Vec2::new(190.0, 120.0)).set_layout(ggez::graphics::TextLayout {
            h_align: ggez::graphics::TextAlign::Middle,
            v_align: ggez::graphics::TextAlign::End,
        }).set_scale(18.0);
        canvas.draw(&text, graphics::DrawParam::from([700.0, 570.0]).color((255,255,255)));

        canvas.finish(ctx)?;
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Q) => self.mode = !self.mode,
            Some(KeyCode::E) => self.active = !self.active,
            Some(KeyCode::C) => self.cells.clear(),
            Some(KeyCode::P) => {
                let mut output = String::new();
                for cell in &self.cells {
                    output.push_str(format!("({},{},{}),", cell.0,cell.1,cell.2).as_str());
                }
                println!("{}",output);
            },
            Some(KeyCode::W) => self.y += 1,
            Some(KeyCode::A) => self.x += 1,
            Some(KeyCode::S) => self.y -= 1,
            Some(KeyCode::D) => self.x -= 1,
            _ => ()
        }
        Ok(())
    }
}
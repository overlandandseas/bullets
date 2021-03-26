use rand::Rng;
use std::io::{self, Write};

use std::time::Duration;
use settimeout::set_timeout;
use async_std::task;
use std::time::{Instant};

const MAX_FPS: f64 = 144.0;
const MILLIS_TO_WAIT: f64 =  (1.0 / MAX_FPS) * 1000.0;

#[derive(Debug)]
struct Bullet {
    x: u16,
    y: u16,
    vx: u16,
    vy: u16
}

struct Game {
    bullets: Vec<Bullet>
}

impl Game {

    fn update(self) -> Game {
        Game { bullets: self.bullets.iter().map(|b| b.update()).collect() }
    }

    fn new() -> Game {
        Game { bullets: Vec::new() }
    }
}

impl Bullet {

    fn update(&self) -> Bullet {
        Bullet::new(
            self.x + self.vx,
            self.y + self.vy,
            self.vx,
            self.vy
        )
    }

    fn new(x: u16, y: u16, vx: u16, vy: u16) -> Bullet {
        Bullet {
            x: x,
            y: y,
            vx: vx,
            vy: vy
        }
    }
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}


async fn recurse(iterations: u16, game: Game, previous: Instant) {

    set_timeout(Duration::from_millis(MILLIS_TO_WAIT as u64)).await;
    let new_game = game.update();

    let fps = 1.0 / previous.elapsed().as_millis() as f64 * 1000.0;
    print!("\rFPS: {}", fps as u64);
    task::block_on(recurse(iterations + 1, new_game, Instant::now()));
}


fn main() {
    print!("Enter number of bullets to spawn:\t");
    io::stdout().flush().expect("expect the unexpected");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => {
            let mut game = Game::new();
            let mut rng = rand::thread_rng();
            for _ in times(i) {

                let bullet = Bullet::new(rng.gen_range(1, 10), rng.gen_range(1, 10), rng.gen_range(1, 10), rng.gen_range(1, 10));

                game.bullets.push(bullet);
                let percent = game.bullets.len() as f64 / i as f64 * 100.0;
                print!("\rGenerating bullets...{}%", percent as u64);

            }
            println!("\n---");
            task::block_on(recurse(0, game, Instant::now()));
        },
        Err(..) => println!("meh meh meh: {}", trimmed),
    };

}

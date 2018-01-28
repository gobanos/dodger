extern crate rand;

mod utils {
    #[derive(Debug)]
    pub struct Rect {
        pub width: u32,
        pub height: u32,
    }

    #[derive(Debug)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Debug)]
    pub struct Body {
        pub rect: Rect,
        pub pos: Position,
    }

    impl Body {
        pub fn new(width: u32, height: u32, x: i32, y: i32) -> Body {
            Body {
                rect: Rect { width, height },
                pos: Position { x, y },
            }
        }
    }
}

use utils::*;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
pub struct Game {
    pub screen: Rect,
    pub player: Player,
    pub baddies: Vec<Baddie>,
}

#[derive(Debug)]
pub struct Player {
    pub body: Body,
}

#[derive(Debug)]
pub struct Baddie {
    pub body: Body,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            screen: Rect { width, height },
            player: Player::new(Body::new(2, 2, width as i32 / 2, 1)),
            baddies: Vec::new(),
        }
    }

    fn spawn_baddie(&mut self) {
        self.baddies.push(Baddie::random(Range::new(1, 4), &self.screen));
    }

    pub fn turn(&mut self, x: i32) -> bool {
        // move player
        self.player.body.pos.x += x;

        // spawn a baddie
        self.spawn_baddie();

        // move & filter baddies
        self.baddies.iter_mut().for_each(|baddie| baddie.make_move());
        self.baddies.retain(|baddie| baddie.visible());

        // check collisions
        self.baddies.iter().any(|baddie| self.player.collides(baddie))
    }
}

impl Player {
    pub fn new(body: Body) -> Player {
        Player {
            body
        }
    }

    pub fn collides(&self, _baddie: &Baddie) -> bool {
        false
    }
}

impl Baddie {
    pub fn new(body: Body) -> Baddie {
        Baddie {
            body
        }
    }

    pub fn random(size: Range<u32>, screen: &Rect) -> Baddie {
        let mut rng = rand::thread_rng();
        let size = size.ind_sample(&mut rng);
        let x = Range::new(0, (screen.width - size) as i32).ind_sample(&mut rng);
        let y = screen.height as i32 + 1;

        Baddie::new(Body::new(size, size, x, y))
    }

    pub fn make_move(&mut self) {
        self.body.pos.y -= 1;
    }

    pub fn visible(&self) -> bool {
        self.body.pos.y > -(self.body.rect.height as i32) + 1
    }
}
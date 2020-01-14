use ncurses::*;
use std::convert::TryInto;

#[derive(Copy, Clone)]
struct Arena {
    tiles: [usize; 9],
    turn: usize,
}

impl Arena {
    fn draw(&self) {
        let mut i: i32 = 1;
        for tile in &self.tiles {
            if i % 3 == 1 {
                addstr("|");
            }
            match *tile {
                1 => addstr("x|"),
                2 => addstr("o|"),
                _ => addstr("-|"),
            };
            if i % 3 == 0 {
                addstr("\n");
            }
            i += 1;
        }
    }
    fn insert(&mut self, cursor: Cursor) {
        let index: usize = (cursor.y + (cursor.x * 3)).try_into().unwrap();
        if self.tiles[index] == 0 {
            self.tiles[index] = self.turn % 2 + 1;
            self.turn += 1;
        }
    }
    fn win(self) {

    }
}

#[derive(Copy, Clone)]
struct Cursor {
    x: i32,
    y: i32,
}

impl Cursor {
    fn traverse(&mut self, direction: i32) {
        match &direction {
            0 => self.x -= 1,
            1 => self.x += 1,
            2 => self.y -= 1,
            3 => self.y += 1,
            _ => (),
        }
        if self.x < 0 {
            self.x = 2;
        }
        if self.x > 2 {
            self.x = 0;
        }
        if self.y < 0 {
            self.y = 2;
        }
        if self.y > 2 {
            self.y = 0;
        }
    }
    fn step(self, window: WINDOW) {
        wmove(window, self.x, self.y * 2 + 1);
    }
}

fn main() {
    // init
    let window = initscr();
    let mut arena = Arena {
        tiles: [0; 9],
        turn: 1,
    };
    let mut cursor = Cursor { x: 0, y: 0 };

    loop {
        // drawing
        clear();
        arena.draw();
        refresh();
        cursor.step(window);

        // input
        match getch() {
            // up
            65 => cursor.traverse(0),
            // down
            66 => cursor.traverse(1),
            // right
            67 => cursor.traverse(3),
            // left
            68 => cursor.traverse(2),
            // place
            10 => arena.insert(cursor),
            // dont do anything
            _ => (),
        }
    }
}

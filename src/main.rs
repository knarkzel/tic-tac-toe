use ncurses::*;

#[derive(Copy, Clone)]
struct Arena {
    tiles: [i32; 9],
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
    fn insert(&mut self, x: i32, y: i32, value: i32) {
	let index = x + (y * 3);
	self.tiles[index] = value;
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
    fn step(&self, window: WINDOW) {
        wmove(window, self.x, self.y * 2 + 1);
    }
}

fn main() {
    // init
    let window = initscr();
    let arena = Arena { tiles: [1; 9] };
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
            // dont do anything
            _ => (),
        }
    }
}

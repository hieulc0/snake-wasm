mod queue;

struct Board {
    width: usize,
    height: usize,
    buf: Vec<Vec<i32>>
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            buf: vec![vec![0; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn render(&self) -> Vec<Vec<i32>> {
        return vec![ vec![0; self.width]; self.height];
    }
}

enum Command {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Snake {
    buf: queue::Queue,
    size: usize,
}

impl Snake {
    pub fn new(size: usize) -> Self {
        Self {
            buf: queue::Queue::new(size),
            size,
        }
    }

    pub fn head(&self) -> (i32, i32) {
        return self.buf.head();
    }

    pub fn eat(&self, command: Command) {
    }

    pub fn proceed(&mut self, command: &Command) {
        let mut head = self.buf.head();
        
        match command {
            Command::UP => {
                head.0 -= 1;
            }
            Command::DOWN => {
                head.0 += 1;
            }
            Command::LEFT => {
                head.1 -= 1;
            }
            Command::RIGHT => {
                head.1 += 1;
            }
        }

        self.buf.push(head);
        self.buf.pop();
    }
}




pub struct Game {
    board: Board,
    snake: Snake,
    food: (i32, i32),
    command: Command,
}

impl Game {
    pub fn new() -> Self {
        let width = 64;
        let height = 27;
        let mut board: Board = Board::new(width, height);
        let mut snake: Snake = Snake::new(width*height);
        let mut food = (0, 0);

        Self {
            board,
            snake,
            food,
            command: Command::RIGHT,
        }
    }

    pub fn run(&mut self) {
        let one_second = std::time::Duration::from_secs(1);
        //loop {
            self.snake.proceed(&self.command);
            std::thread::sleep(one_second);
        //}
    }

    pub fn command(&mut self, command: Command) {
        self.command = command;
    }

    pub fn render(&self) -> Vec<Vec<i32>> {
        let width = self.board.width();
        let height = self.board.height();

        let board = Board::new(width, height);
        return board.render();
    }
}




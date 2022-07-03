pub struct Queue {
    head: usize,
    tail: usize,
    buf: Vec<(i32, i32)>,
    size: usize
}

impl Queue {
    pub fn new(size: usize) -> Self {
        Self {
            head: 0,
            tail: 0,
            buf: vec![(0, 0); size+1],
            size: size,
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.size == 0 || self.head == self.tail;
    }

    fn next(&self, pos: usize) -> usize {
        let mut new_pos = pos + 1;
        if new_pos == self.size {
            new_pos = 0;
        }
        return new_pos;
    }

    fn next_tail(&self) -> usize {
        return self.next(self.tail);
    }

    fn next_head(&self) -> usize {
        return self.next(self.head);
    }
        
    pub fn is_full(&self) -> bool {
        return self.next_head() == self.tail;
    }


    pub fn push(&mut self, value: (i32, i32)) {
        if self.is_full() {
            panic!("queue is full!");
        }
        self.head = self.next_head();
        self.buf[self.head] = value;
    }

    pub fn pop(&mut self) -> (i32, i32) {
        if self.is_empty() {
            panic!("queue is empty!");
        }
        self.tail = self.next_tail();
        return self.buf[self.tail];
    }

    pub fn peek(&self) -> (i32, i32) {
        if self.is_empty() {
            panic!("queue is empty!");
        }
        return self.buf[self.next_tail()];
    }

    pub fn head(&self) -> (i32, i32) {
        if self.is_empty() {
            panic!("queue is empty!");
        }
        return self.buf[self.head];
    }
}


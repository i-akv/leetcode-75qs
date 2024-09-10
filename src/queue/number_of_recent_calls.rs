use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while self.queue.front().map_or(false, |x| *x < t) {
            self.queue.pop_front();
        }
        self.queue.push_back(t + 3000);
        self.queue.len() as i32
    }
}
use std::time::{Duration, Instant};

pub struct ProgressLoader {
    horizontal_elements: usize,
    vertical_elements: usize,
    progress: usize,
    working: bool,
    last_frame_time: Option<Duration>,
    start_time: Option<Instant>,
}

impl ProgressLoader {
    pub fn new(horizontal_elements: usize, vertical_elements: usize) -> ProgressLoader {
        ProgressLoader {
            horizontal_elements,
            vertical_elements,
            progress: 0,
            working: false,
            last_frame_time: None,
            start_time: None,
        }
    }

    pub fn semi_increment(&mut self) {
        self.working = true;
        self.start_time = Some(Instant::now());
        self.update_progress();
    }

    pub fn increment(&mut self) {
        self.working = false;
        self.progress += 1;
        self.update_progress();
    }

    fn update_progress(&mut self) {
        let percentage = (self.progress as f32
            / (self.horizontal_elements * self.vertical_elements) as f32)
            * 100.0;

        let mut line = 0;
        for i in 0..(self.vertical_elements * self.horizontal_elements) {
            if i < self.progress {
                print!("\x1b[32m█\x1b[0m");
            } else {
                if self.working {
                    print!("\x1b[1;33m▒\x1b[0m");
                    self.working = false;
                } else {
                    print!("░");
                }
            }
            if i % self.horizontal_elements == self.horizontal_elements - 1 {
                if i == (self.horizontal_elements * self.vertical_elements) / 2 - 1 {
                    print!("  {:.2}%  ", percentage);
                    self.last_frame_time = Some(self.start_time.unwrap().elapsed());
                    if let Some(duration) = self.last_frame_time {
                        print!("{:?}", duration);
                    }
                }
                line += 1;
                println!();
            }
        }
        println!();
    }
}

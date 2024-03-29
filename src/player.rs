pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            width: 50.0,
            height: 50.0,
            speed: 1.5,
        }
    }
    pub fn speed_incr(&mut self, val: f32) {
        self.speed += val;
        if self.speed > 5.0 {self.speed = 5.0;}
    }
    pub fn speed_decr(&mut self, val: f32) {
        self.speed -= val;
        if self.speed < 0.0 {self.speed = 0.0;} 
    }
}

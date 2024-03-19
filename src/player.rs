pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
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
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    //pub fn shoot(){}
}

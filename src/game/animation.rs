use super::robot::sprite::SpriteAnimation;



#[derive(Clone, Copy, Debug)]
pub enum AnimationDirection {
    Forwards,
    Backwards,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        Self::Forwards
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ComponentAnimation {
    pub direction: AnimationDirection,
    pub frames: usize,
    pub frame: usize,
    pub changed: bool,
    pub clamp: bool,
}

impl ComponentAnimation {
    pub fn new(frames: usize) -> Self {
        Self {
            frames,
            ..Self::default()
        }
    }

    pub fn clamp(&mut self) {
        self.clamp = true;
    }

    pub fn wrap(&mut self) {
        self.clamp = false;
    }

    pub fn set_direction(&mut self, direction: AnimationDirection) {
        self.direction = direction;
        self.changed = true;
    }

    pub fn set_frame(&mut self, frame: usize) {
        self.frame = frame;
        self.changed = true;
    }

    pub fn set_discrete(&mut self, value: f32, max: f32) {
        let steps = self.frames as f32;
        let index = ((value * steps) / max).ceil() as usize;
        self.frame = index % self.frames;
        self.changed = true;
    }

    pub fn play(&mut self) {
        match self.direction {
            AnimationDirection::Forwards => self.add(),
            AnimationDirection::Backwards => self.sub(),
        }
    }

    fn add(&mut self) {
        self.frame += 1;
        if self.frame >= self.frames {
            if !self.clamp {
                self.frame = 0;
            } else {
                self.frame = self.frames - 1;
            }
        }
    }

    fn sub(&mut self) {
        self.frame = self.frame.saturating_sub(1);
        if self.frame == 0 {
            if !self.clamp {
                self.frame = self.frames - 1;
            } else {
                self.frame = 0;
            }
        }
    }

    pub fn play_once(&mut self) {
        if self.changed {
            self.play();
            self.changed = false;
        }
    }
}


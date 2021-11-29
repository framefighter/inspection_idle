#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationDirection {
    Forward,
    Backward,
}

impl Default for AnimationDirection {
    fn default() -> Self {
        AnimationDirection::Forward
    }
}

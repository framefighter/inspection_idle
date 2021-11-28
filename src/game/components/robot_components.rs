use bevy_inspector_egui::Inspectable;


#[derive(Debug, Inspectable, Default)]
pub struct Motors {
    pub linear_speed: f32,
    pub angular_speed: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct CameraZoom {
    max_zoom: f32,
    zoom_speed: f32,
    zoom: f32,
}

#[derive(Debug, Inspectable, Default)]
pub struct ImageQuality {
    width: f32,
    height: f32,
    noise: f32
}


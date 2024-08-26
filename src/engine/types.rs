#[derive(Clone)]
pub struct WindowSize {
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Default)]
pub struct XYPair {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct ObjectInfo {
    // in case we want to support window resizing in the
    // future, the objects might want to know the size.
    // this is however a bad example, because it doesn't
    // need to be set per game object, so let's fix that later
    pub window_size: WindowSize,
}

pub enum DeviceTaskKind {
    Render,
    FilmConvert,
    Shader
}

pub struct DeviceTask {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    pub kind: DeviceTaskKind

    // acquire_tile
    
}


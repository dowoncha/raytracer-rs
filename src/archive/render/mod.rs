pub enum RenderTileTask {
    PathTrace,
    Denoise
}

pub struct RenderTile {
    pub task: RenderTileTask,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    start_sample: i32,
    num_samples: i32,
    sample: i32,
    resolution: i322,
    offset: i32,
    stride: i32,
    tile_index: i32,

    // buffer: device_ptr
    // buffers: Box<RenderBuffers>
}
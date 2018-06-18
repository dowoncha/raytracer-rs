enum TileState {
    Render,
    Rendered,
    Denoise,
    Denoised,
    Done
}

struct Tile {
    pub index: i32,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub device: i32,
    state: TileState
}
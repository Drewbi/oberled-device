#[derive(PartialEq, Copy, Clone)]
pub enum PixelState {
    Off,
    On
}

#[derive(PartialEq, Copy, Clone)]
pub enum Orientation {
    Landscape,
    Portrait,
    LandscapeFlipped,
    PortraitFlipped,
}
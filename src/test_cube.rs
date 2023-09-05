use super::{ColouredVertex, PositionVertex, Normal};

pub const COLOURED_VERTICES: [ColouredVertex; 8] = [
    ColouredVertex {position: [-0.5, -0.5, -0.5], colour: [0.84, 0.01, 0.44, 1.0]},
    ColouredVertex {position: [0.5, -0.5, -0.5], colour: [0.61, 0.31, 0.59, 1.0]},
    ColouredVertex {position: [-0.5, -0.5, 0.5], colour: [0.61, 0.31, 0.59, 1.0]},
    ColouredVertex {position: [0.5, -0.5, 0.5], colour: [0.0, 0.22, 0.66, 1.0]},
    ColouredVertex {position: [-0.5, 0.5, -0.5], colour: [0.84, 0.01, 0.44, 1.0]},
    ColouredVertex {position: [0.5, 0.5, -0.5], colour: [0.61, 0.31, 0.59, 1.0]},
    ColouredVertex {position: [-0.5, 0.5, 0.5], colour: [0.61, 0.31, 0.59, 1.0]},
    ColouredVertex {position: [0.5, 0.5, 0.5], colour: [0.0, 0.22, 0.66, 1.0]}
];

pub const UNCOLOURED_VERTICES: [PositionVertex; 8] = [
    PositionVertex {position: [-0.5, -0.5, -0.5]},
    PositionVertex {position: [0.5, -0.5, -0.5]},
    PositionVertex {position: [-0.5, -0.5, 0.5]},
    PositionVertex {position: [0.5, -0.5, 0.5]},
    PositionVertex {position: [-0.5, 0.5, -0.5]},
    PositionVertex {position: [0.5, 0.5, -0.5]},
    PositionVertex {position: [-0.5, 0.5, 0.5]},
    PositionVertex {position: [0.5, 0.5, 0.5]}
];

pub const NORMALS: [Normal; 8] = [
    Normal {normal: [-1.0, -1.0, -1.0]},
    Normal {normal: [1.0, -1.0, -1.0]},
    Normal {normal: [-1.0, -1.0, 1.0]},
    Normal {normal: [1.0, -1.0, 1.0]},
    Normal {normal: [-1.0, 1.0, -1.0]},
    Normal {normal: [1.0, 1.0, -1.0]},
    Normal {normal: [-1.0, 1.0, 1.0]},
    Normal {normal: [1.0, 1.0, 1.0]},
];

pub const INDICES: [u32; 36] = [
    0, 4, 1,
    4, 5, 1,

    1, 5, 3,
    5, 7, 3,

    3, 7, 2,
    7, 6, 2,

    2, 6, 0,
    6, 4, 0,

    0, 2, 1,
    2, 3, 1,
    
    4, 6, 5,
    6, 7, 5
];
macro_rules! coord {
    ($x: expr, $y: expr) => {{
        use crate::board::Coordinates;
        Coordinates { x: $x, y: $y }
    }};
}
pub(crate) use coord;

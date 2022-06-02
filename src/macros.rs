macro_rules! coord {
    // macth like arm for macro
    ($x:expr, $y:expr) => {{
        use crate::board::Coordinates;

        Coordinates { x: $x, y: $y }
    }};
}
pub(crate) use coord;

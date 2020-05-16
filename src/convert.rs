use nalgebra::Point2;

pub fn axial_to_world(coords: Point2<i64>) -> Point2<f64> {
    let x = coords[0] as f64;
    let y = (2 * coords[1] - coords[0]) as f64;
    Point2::new(x, y)
}

pub fn world_to_axial(coords: Point2<f64>) -> Point2<i64> {
    let (x, y) = (coords[0], coords[1]);
    let x_floor = x.floor();
    let i = x_floor as i64;
    let j = ((y + x_floor) / 2.0).floor() as i64;

    let origin = axial_to_world(Point2::new(i, j));
    let local_x = x - origin[0];
    let local_y = (y - origin[1]) / 2.0;

    let result = if local_x + local_y < 0.25 {
        [i - 1, j - 1]
    } else if local_x + (1.0 - local_y) < 0.25 {
        [i - 1, j]
    } else if (1.0 - local_x) + (1.0 - local_y) < 0.25 {
        [i + 1, j + 1]
    } else if (1.0 - local_x) + local_y < 0.25 {
        [i + 1, j]
    } else {
        [i, j]
    };

    Point2::from(result)
}

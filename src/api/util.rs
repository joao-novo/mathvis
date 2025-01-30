pub fn in_axis_range(val: f64, (start, end): (f64, f64)) -> bool {
    start <= val && val <= end
}

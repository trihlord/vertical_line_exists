pub fn vertical_line_exists(mut points: Vec<(i32, i32)>) -> bool {
    let half_len = points.len() / 2;
    points.sort_by_key(|point| point.1);
    points.dedup_by_key(|point| point.1);
    points.len().eq(&half_len)
}

#[cfg(test)]
mod tests {
    use super::vertical_line_exists;

    #[test]
    fn it_works() {
        let points = vec![(1, 1), (2, 5), (4, 5), (5, 1)];
        let result = vertical_line_exists(points);
        assert_eq!(result, true)
    }

    #[test]
    fn it_panics() {
        let points = vec![(1, 1), (2, 2), (4, 5), (5, 1)];
        let result = vertical_line_exists(points);
        assert_eq!(result, false)
    }
}

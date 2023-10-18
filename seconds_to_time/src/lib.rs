// src/lib.rs

pub fn convert_seconds(seconds: i32) -> (i32, i32, i32, i32) {
    let days = seconds / (24 * 3600);
    let mut remain_seconds = seconds % (24 * 3600);
    let hours = remain_seconds / 3600;
    remain_seconds %= 3600;
    let minutes = remain_seconds / 60;
    remain_seconds %= 60;
    (days, hours, minutes, remain_seconds)
}

#[cfg(test)]
mod tests {

    use super::*;

    //
    // Test Technique ::: Equivalence Class & Boundary value Analysis
    //
    // [1s, ....,  59s] s ==> Seconds partition
    // [60s, ...,  3599] s ==> Minutes partition
    // [3600, ..., 86399] s ==> Hours partition
    // [86400, ... ] ===> Days partition
    // 
    #[test]
    fn test_seconds_partition_low_boundary() {
        let expected = (0, 0, 0, 1);
        let (days, hours, minutes, seconds) = convert_seconds(1);
        assert_eq!((days, hours, minutes, seconds ), expected)
    }

    #[test]
    fn test_seconds_partition_high_boundary() {
        let expected = (0, 0, 0, 59);
        let (days, hours, minutes, seconds) = convert_seconds(59);
        assert_eq!((days, hours, minutes, seconds ), expected)
    }

    #[test]
    fn test_minutes_partition_low_boundary() {
        let expected = (0, 0, 1, 0);
        let (days, hours, minutes, seconds) = convert_seconds(60);
        assert_eq!((days, hours, minutes, seconds ), expected)
    }

    #[test]
    fn test_minutes_partition_high_boundary() {
        let expected = (0, 0, 59, 59);
        let (days, hours, minutes, seconds) = convert_seconds(3599);
        assert_eq!((days, hours, minutes, seconds), expected)
    }

    #[test]
    fn test_hours_partition_low_boundary() {
        let expected = (0, 1, 0, 0);
        let (days, hours, minutes, seconds) = convert_seconds(3600);
        assert_eq!((days, hours, minutes, seconds ), expected)
    }

    #[test]
    fn test_hours_partition_high_boundary() {
        let expected = (0, 23, 59, 59);
        let (days, hours, minutes, seconds) = convert_seconds(86399);
        assert_eq!((days, hours, minutes, seconds), expected)
    }

    #[test]
    fn test_days_partition_low_boundary() {
        let expected = (1, 0, 0, 0);
        let (days, hours, minutes, seconds) = convert_seconds(86400);
        assert_eq!((days, hours, minutes, seconds ), expected)
    }

}

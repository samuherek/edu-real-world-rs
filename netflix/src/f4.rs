/// Identify if there is an increasing or decreasing trend
///
/// Time complexity -> O(n)
/// Space complexity -> O(1)

fn has_trend(data: &[i32]) -> bool {
    let dec_trend = data.windows(2).all(|v| v[0] >= v[1]);
    let inc_trend = data.windows(2).all(|v| v[0] <= v[1]);
    dec_trend || inc_trend
}

mod test {
    #[test]
    fn test() {
        let movie_ratings = vec![
            vec![1, 2, 2, 3],
            vec![4, 5, 6, 3, 4],
            vec![8, 8, 7, 6, 5, 4, 4, 1],
        ];
        assert_eq!(true, super::has_trend(&movie_ratings[0]));
        assert_eq!(false, super::has_trend(&movie_ratings[1]));
        assert_eq!(true, super::has_trend(&movie_ratings[2]));
    }
}

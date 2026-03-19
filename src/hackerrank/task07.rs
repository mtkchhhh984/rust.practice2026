// https://www.hackerrank.com/challenges/between-two-sets/problem
fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    let start = *a.iter().max().unwrap();
    let end = *b.iter().min().unwrap();

    for x in start..=end {
        let valid_for_a = a.iter().all(|value| x % value == 0);
        let valid_for_b = b.iter().all(|value| value % x == 0);

        if valid_for_a && valid_for_b {
            count += 1;
        }
    }

    count
}

#[test]
fn test7() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];
    let real = get_total_x(&a, &b);
    let expected = 3;
    assert_eq!(real, expected);
}
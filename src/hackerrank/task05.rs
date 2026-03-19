// https://www.hackerrank.com/challenges/apple-and-orange/problem
fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &apple in apples {
        let position = a + apple;
        if position >= s && position <= t {
            apple_count += 1;
        }
    }

    for &orange in oranges {
        let position = b + orange;
        if position >= s && position <= t {
            orange_count += 1;
        }
    }

    (apple_count, orange_count)
}

#[test]
fn test5() {
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let real = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
    let expected = (1, 1);

    assert_eq!(real, expected);
}
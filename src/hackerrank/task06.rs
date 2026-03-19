// https://www.hackerrank.com/challenges/kangaroo/problem
fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

#[test]
fn test6_yes() {
    let real = kangaroo(0, 3, 4, 2);
    let expected = "YES".to_string();
    assert_eq!(real, expected);
}

#[test]
fn test6_no() {
    let real = kangaroo(0, 2, 5, 3);
    let expected = "NO".to_string();
    assert_eq!(real, expected);
}
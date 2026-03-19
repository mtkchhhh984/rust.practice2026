// https://www.hackerrank.com/challenges/sock-merchant/problem
fn sock_merchant(ar: &[i32]) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for &sock in ar {
        *map.entry(sock).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for &count in map.values() {
        pairs += count / 2;
    }

    pairs
}

#[test]
fn test10() {
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
    let real = sock_merchant(&ar);
    let expected = 3;
    assert_eq!(real, expected);
}
// https://www.hackerrank.com/challenges/migratory-birds/problem
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut count = [0; 6]; // індекси 1..5

    for &bird in arr {
        count[bird as usize] += 1;
    }

    let mut max_count = 0;
    let mut result = 0;

    for i in 1..=5 {
        if count[i] > max_count {
            max_count = count[i];
            result = i as i32;
        }
    }

    result
}

#[test]
fn test9() {
    let arr = vec![1, 4, 4, 4, 5, 3];
    let real = migratory_birds(&arr);
    let expected = 4;
    assert_eq!(real, expected);
}
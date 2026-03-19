// https://www.hackerrank.com/challenges/grading/problem
fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for &grade in grades {
        if grade < 38 {
            result.push(grade);
        } else {
            let next_multiple = ((grade / 5) + 1) * 5;
            if next_multiple - grade < 3 {
                result.push(next_multiple);
            } else {
                result.push(grade);
            }
        }
    }

    result
}

#[test]
fn test4() {
    let grades = vec![73, 67, 38, 33];
    let real = grading_students(&grades);
    let expected = vec![75, 67, 40, 33];
    assert_eq!(real, expected);
}
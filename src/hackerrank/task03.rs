fn staircase(n: i32) -> String {
    let mut result = String::new();

    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push_str(&spaces);
        result.push_str(&hashes);

        if i < n {
            result.push('\n');
        }
    }

    result
}

#[test]
fn test3() {
    let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
    let real = staircase(6);
    assert_eq!(real, expected);
}
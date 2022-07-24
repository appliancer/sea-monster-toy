use super::*;

#[test]
fn decimal_precision() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         1, 10,      2
                           deposit,         1, 12,      2.0
                           deposit,         1, 13,      2.0001
                           deposit,         1, 14,      2.0001
                           deposit,         1, 15,      2.9999
                           deposit,         1, 16,      2.010
                           withdrawal,      1, 17,      3.1
"#;

    const WANT: &str = r#"client,available,held,total,locked
1,9.9101,0,9.9101,false
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn dispute_resolve() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         2,  6,     22
                           dispute,         2,  6,
                           resolve,         2,  6,
"#;

    const WANT: &str = r#"client,available,held,total,locked
2,22,0,22,false
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn dispute_chargeback() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         3,  7,      2
                           deposit,         3,  8,      4
                           dispute,         3,  7,
                           chargeback,      3,  7,
                           deposit,         3,  9,     30
"#;

    const WANT: &str = r#"client,available,held,total,locked
3,4,0,4,true
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn dispute_after_withdrawal() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         4,  2,     13
                           deposit,         4,  3,     11
                           withdrawal,      4,  8,     20
                           dispute,         4,  2,
"#;

    const WANT: &str = r#"client,available,held,total,locked
4,-9,13,4,false
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn chargeback_after_withdrawal() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         4,  2,     13
                           deposit,         4,  3,     11
                           withdrawal,      4,  8,     20
                           dispute,         4,  2,
                           chargeback,      4,  2,
"#;

    const WANT: &str = r#"client,available,held,total,locked
4,-9,0,-9,true
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn overdraw_attempt() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         5,  1,     15
                           withdrawal,      5,  4,     10
                           withdrawal,      5,  5,     10
"#;

    const WANT: &str = r#"client,available,held,total,locked
5,5,0,5,false
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn invalid_transactions() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         6,  2,     13
                           deposit,         6,  3,     11
                           resolve,         6,  2,
                           chargeback,      6,  2,
                           dispute,         6, 50,
                           dispute,         6,  4,
"#;

    const WANT: &str = r#"client,available,held,total,locked
6,24,0,24,false
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    assert_eq!(String::from_utf8(buf).unwrap(), WANT);
}

#[test]
fn multiple_clients() {
    const INPUT: &str = r#"type,       client, tx, amount
                           deposit,         1,  1,      9
                           deposit,         1,  2,     14
                           deposit,         2,  3,     28
                           withdrawal,      1,  4,      8
                           deposit,         3,  5,     95
                           deposit,         4,  6,     21
                           deposit,         4,  7,      7
                           dispute,         4,  6,
                           dispute,         1,  2,
                           dispute,         3,  5,
                           resolve,         3,  5,
                           chargeback,      4,  6,
"#;

    const WANT: &str = r#"client,available,held,total,locked
1,1,14,15,false
2,28,0,28,false
3,95,0,95,false
4,7,0,7,true
"#;

    let mut buf = Vec::new();
    run(INPUT.as_bytes(), &mut buf).unwrap();

    let actual = String::from_utf8(buf).unwrap();
    let actual_lines: Vec<&str> = actual.lines().collect();
    assert_eq!(actual_lines.len(), WANT.lines().count());

    for want_line in WANT.lines() {
        assert!(
            actual_lines.contains(&want_line),
            "actual output does not contain line {}\nactual output:\n{}",
            want_line,
            actual
        );
    }
}

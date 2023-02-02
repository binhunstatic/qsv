use crate::workdir::Workdir;

#[test]
fn tojsonl_simple() {
    let wrk = Workdir::new("tojsonl_simple");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "oldest_child", "boy", "weight"],
            svec!["1", "Mark", "Charlotte", "Tom", "true", "150.2"],
            svec!["2", "John", "Ann", "Jessika", "false", "175.5"],
            svec!["3", "Bob", "Monika", "Jerry", "true", "199.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true,"weight":150.2}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false,"weight":175.5}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true,"weight":199.5}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["false", "John"],
            svec!["false", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["t", "Mark"],
            svec!["f", "John"],
            svec!["f", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_upper_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["T", "Mark"],
            svec!["F", "John"],
            svec!["F", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_1or0() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
            svec!["0", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_not_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["false", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // not treated as boolean since col1's domain has three values
    // True, False and false
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":"True","col2":"Mark"}
{"col1":"False","col2":"John"}
{"col1":"false","col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_is_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["False", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // this is treated as boolean since col1's domain has two values
    // True and False
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_yes() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["yes", "Mark"],
            svec!["no", "John"],
            svec!["no", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boolean_yes_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["y", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_nested() {
    let wrk = Workdir::new("tojsonl_nested");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "children"],
            svec!["1", "Mark", "Charlotte", "\"Tom\""],
            svec!["2", "John", "Ann", "\"Jessika\",\"Antony\",\"Jack\""],
            svec!["3", "Bob", "Monika", "\"Jerry\",\"Karol\""],
            svec![
                "4",
                "John\nSmith",
                "Jane \"Smiley\" Doe",
                "\"Jack\",\"Jill\r\n \"Climber\""
            ],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","children":"\"Tom\""}
{"id":2,"father":"John","mother":"Ann","children":"\"Jessika\",\"Antony\",\"Jack\""}
{"id":3,"father":"Bob","mother":"Monika","children":"\"Jerry\",\"Karol\""}
{"id":4,"father":"John\nSmith","mother":"Jane \"Smiley\" Doe","children":"\"Jack\",\"Jill\r\n \"Climber\""}"#;

    assert_eq!(got, expected);
}

#[test]
fn tojsonl_boston() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.jsonl");

    assert_eq!(got, expected.replace("\r\n", "\n").trim_end());
}

// rustfmt-force_format_strings: true
// rustfmt-error_on_line_overflow: false
// Long string literals

fn main() -> &'static str {
    let str = "AAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAAAAAA AAAAAAAAAAAAAAAAAAAAAAaAA \
               AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaAa";
    let str = "AAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAaAa";
    let str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    let too_many_lines = "Hello";

    // Make sure we don't break after an escape character.
    let odd_length_name =
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";
    let even_length_name =
        "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

    let really_long_variable_name = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    let raw_string = r#"Do
not
remove
formatting"#;

    filename.replace(" ", "\\");

    let xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx =
        funktion("yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");

    let unicode = "a̐éö̲\r\n";
    let unicode2 = "Löwe 老虎 Léopard";
    let unicode3 = "中华Việt Nam";
    let unicode4 = "☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃☃";

    "stuffin'"
}

fn issue682() {
    let a = "hello \\ o/";
    let b = a.replace("\\ ", "\\");
}

fn issue716() {
    println!("forall x. mult(e(), x) = x /\\
              forall x. mult(x, x) = e()");
}

fn issue_1282() {
    {
        match foo {
            Permission::AndroidPermissionAccessLocationExtraCommands => {
                "android.permission.ACCESS_LOCATION_EXTRA_COMMANDS"
            }
        }
    }
}

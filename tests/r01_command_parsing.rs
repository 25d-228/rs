use std::error::Error;
use std::panic::{AssertUnwindSafe, catch_unwind};

use rs::{Command, CommandName, KeyError, MAX_KEY_BYTES, ParseError, parse_command};

fn assert_error_type<T: Error>() {}

#[test]
fn parses_get_and_delete_into_typed_commands() {
    match parse_command("GET alpha") {
        Ok(Command::Get(key)) => assert_eq!(key.as_str(), "alpha"),
        other => panic!("expected typed GET command, got {other:?}"),
    }

    match parse_command("DELETE users/東京") {
        Ok(Command::Delete(key)) => assert_eq!(key.as_str(), "users/東京"),
        other => panic!("expected typed DELETE command, got {other:?}"),
    }
}

#[test]
fn accepts_leading_trailing_and_unicode_field_whitespace() {
    match parse_command("\u{2003}\tGET\nalpha\u{2003}") {
        Ok(Command::Get(key)) => assert_eq!(key.as_str(), "alpha"),
        other => panic!("expected a GET command, got {other:?}"),
    }
}

#[test]
fn rejects_empty_or_whitespace_only_input() {
    assert_eq!(parse_command(""), Err(ParseError::EmptyInput));
    assert_eq!(
        parse_command(" \t\n\u{2003}"),
        Err(ParseError::EmptyInput)
    );
}

#[test]
fn unknown_command_wins_before_arity_checks() {
    assert_eq!(
        parse_command("get alpha extra"),
        Err(ParseError::UnknownCommand {
            command: "get".to_owned(),
        })
    );
}

#[test]
fn reports_missing_keys_for_each_known_command() {
    assert_eq!(
        parse_command("GET"),
        Err(ParseError::MissingKey {
            command: CommandName::Get,
        })
    );
    assert_eq!(
        parse_command("DELETE"),
        Err(ParseError::MissingKey {
            command: CommandName::Delete,
        })
    );
}

#[test]
fn reports_only_the_first_unexpected_argument() {
    assert_eq!(
        parse_command("DELETE alpha first-extra second-extra"),
        Err(ParseError::UnexpectedArgument {
            command: CommandName::Delete,
            argument: "first-extra".to_owned(),
        })
    );
}

#[test]
fn arity_error_precedes_key_validation() {
    let too_long = "x".repeat(MAX_KEY_BYTES + 1);
    let input = format!("GET {too_long} extra");

    assert_eq!(
        parse_command(&input),
        Err(ParseError::UnexpectedArgument {
            command: CommandName::Get,
            argument: "extra".to_owned(),
        })
    );
}

#[test]
fn wraps_key_validation_failure() {
    assert_eq!(
        parse_command("GET ab\0cd"),
        Err(ParseError::InvalidKey(KeyError::ContainsControl {
            byte_index: 2,
        }))
    );
}

#[test]
fn command_names_are_case_sensitive() {
    assert_eq!(
        parse_command("Delete alpha"),
        Err(ParseError::UnknownCommand {
            command: "Delete".to_owned(),
        })
    );
}

#[test]
fn parse_error_implements_error_display_and_source() {
    assert_error_type::<ParseError>();

    let nested = ParseError::InvalidKey(KeyError::Empty);
    let source = nested.source().expect("InvalidKey must expose its source");
    assert!(!source.to_string().trim().is_empty());
    assert!(ParseError::EmptyInput.source().is_none());

    for error in [
        ParseError::EmptyInput,
        ParseError::UnknownCommand {
            command: "FETCH".to_owned(),
        },
        ParseError::MissingKey {
            command: CommandName::Get,
        },
        ParseError::UnexpectedArgument {
            command: CommandName::Delete,
            argument: "extra".to_owned(),
        },
        nested,
    ] {
        assert!(
            !error.to_string().trim().is_empty(),
            "Display must identify {error:?}"
        );
    }
}

#[test]
fn documented_external_inputs_do_not_panic() {
    let inputs = [
        "",
        "   ",
        "GET",
        "DELETE",
        "FETCH key",
        "GET key extra",
        "GET ab\0cd",
        "GET café",
        "\u{2003}DELETE\talpha\n",
    ];

    for input in inputs {
        let outcome = catch_unwind(AssertUnwindSafe(|| parse_command(input)));
        assert!(outcome.is_ok(), "parser panicked for input {input:?}");
    }
}

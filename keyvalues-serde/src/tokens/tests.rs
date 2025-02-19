use keyvalues_parser::{Obj, Value, Vdf};

use std::borrow::Cow;

use crate::tokens::{
    naive::{NaiveToken, NaiveTokenStream},
    Token, TokenStream,
};

// "outer"
// {
//     "sequence start"
//     {
//         "inner key"    "inner val"
//     }
//     "sequence start"    "some other inner val"
// }
#[test]
fn vdf_from_token_stream_basics() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::str("sequence start"),
        NaiveToken::SeqBegin,
        NaiveToken::ObjBegin,
        NaiveToken::str("inner key"),
        NaiveToken::str("inner val"),
        NaiveToken::ObjEnd,
        NaiveToken::str("some other inner val"),
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    let ideal = {
        let mut sequence_obj = Obj::new();
        sequence_obj.insert(
            Cow::from("inner key"),
            vec![Value::Str {
                string: Cow::from("inner val"),
                line: 0,
            }],
        );

        let mut outer_val = Obj::new();
        outer_val.insert(
            Cow::from("sequence start"),
            vec![
                Value::Obj(sequence_obj),
                Value::Str {
                    string: Cow::from("some other inner val"),
                    line: 0,
                },
            ],
        );

        Vdf {
            key: Cow::from("outer"),
            value: Value::Obj(outer_val),
        }
    };

    let actual = Vdf::from(&naive_token_stream);
    assert_eq!(actual, ideal);
}

#[test]
#[should_panic]
fn invalid_vdf_nested_seq() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::str("nested sequence"),
        NaiveToken::SeqBegin,
        NaiveToken::str("the calm before the storm"),
        NaiveToken::SeqBegin,
        NaiveToken::SeqEnd,
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    let _ = Vdf::from(&naive_token_stream);
}

#[test]
#[should_panic]
fn invalid_vdf_obj_key() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::ObjBegin,
        NaiveToken::ObjEnd,
        NaiveToken::ObjEnd,
    ]);

    let _ = Vdf::from(&naive_token_stream);
}

#[test]
#[should_panic]
fn invalid_vdf_seq_key() {
    let naive_token_stream = NaiveTokenStream(vec![
        NaiveToken::str("outer"),
        NaiveToken::ObjBegin,
        NaiveToken::SeqBegin,
        NaiveToken::SeqEnd,
        NaiveToken::ObjEnd,
    ]);

    let _ = Vdf::from(&naive_token_stream);
}

#[test]
fn token_stream_from_vdf() {
    let s = r#"
"Outer Key"
{
    "Inner Key" "Inner Value"
    "Inner Key"
    {
    }
}
        "#;
    let vdf = Vdf::parse(s).unwrap();
    let token_stream = TokenStream::from(vdf);
    assert_eq!(
        token_stream,
        TokenStream(vec![
            Token::Key(Cow::from("Outer Key")),
            Token::ObjBegin,
            Token::Key(Cow::from("Inner Key")),
            Token::SeqBegin,
            Token::Str(Cow::from("Inner Value")),
            Token::ObjBegin,
            Token::ObjEnd,
            Token::SeqEnd,
            Token::ObjEnd,
        ])
    );
}

/*!

The Gutenberg post parser.

The Gutenberg post parser is a parser combinator. Thus it provides
mulitple parsers, aka combinators. They are based on the [nom]
project.

The writing of combiantors heavily relies on Rust macros. Don't be
surprise! To learn more, consult the [documentation].

[nom]: https://github.com/Geal/nom/
[documentation]: https://docs.rs/nom/%2A/nom/

*/

use super::Input;
use super::ast::Node;
use super::combinators;
#[cfg(feature = "wasm")] use alloc::Vec;
use nom::ErrorKind;

named_attr!(
    #[doc="
        Axiom of the grammar: Recognize a list of blocks.
    "],
    pub block_list<Input, Vec<Node>>,
    fold_into_vector_many0!(
        alt_complete!(
            block
          | phrase
        ),
        vec![]
    )
);

named_attr!(
    #[doc=""],
    phrase<Input, Node>,
    map_res!(
        alt_complete!(
            take_until!("<!--")
          | call!(combinators::id)
        ),
        phrase_mapper
    )
);

#[inline(always)]
fn phrase_mapper<'a>(input: Input<'a>) -> Result<Node<'a>, ErrorKind> {
    if input.is_empty() {
        Err(ErrorKind::Custom(42u32))
    } else {
        Ok(Node::Phrase(input))
    }
}

named_attr!(
    #[doc="
        Recognize a block.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::{ast::Node, parser::block};

        let input = &b\"<!-- wp:ns/foo {\\\"abc\\\": \\\"xyz\\\"} --><!-- /wp:ns/foo -->\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\"\"[..],

                // The Abstract Syntax Tree.
                Node::Block {
                    name: (&b\"ns\"[..], &b\"foo\"[..]),
                    attributes: Some(&b\"{\\\"abc\\\": \\\"xyz\\\"}\"[..]),
                    children: vec![]
                }
            )
        );

        assert_eq!(block(input), output);
        ```
    "],
    pub block<Input, Node>,
    do_parse!(
        tag!("<!--") >>
        opt!(whitespaces) >>
        tag!("wp:") >>
        name: block_name >>
        whitespaces >>
        attributes: opt!(block_attributes) >>
        opt!(whitespaces) >>
        result: alt!(
            // Balanced block.
            do_parse!(
                tag!("-->") >>
                children: fold_into_vector_many0!(
                    alt_complete!(
                        block
                      | phrase
                    ),
                    vec![]
                ) >>
                tag!("<!--") >>
                opt!(whitespaces) >>
                tag!("/wp:") >>
                _closing_name: block_name >>
                opt!(whitespaces) >>
                tag!("-->") >>
                (
                    // @todo: Need to check that `closing_name` is equal to `name`.
                    Node::Block {
                        name: name,
                        attributes: attributes,
                        children: children
                    }
                )
            )
            // Void block.
          | do_parse!(
                tag!("/-->") >>
                (
                    Node::Block {
                        name: name,
                        attributes: attributes,
                        children: vec![]
                    }
                )
            )
        ) >>
        (result)
    )
);

named_attr!(
    #[doc="
        Recognize a fully-qualified block name.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::block_name;

        let input = &b\"foo/bar baz\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\" baz\"[..],

                // The Abstract Syntax Tree.
                (&b\"foo\"[..], &b\"bar\"[..])
            )
        );

        assert_eq!(block_name(input), output);
        ```
    "],
    pub block_name<Input, (Input, Input)>,
    alt!(
        namespaced_block_name |
        core_block_name
    )
);

named_attr!(
    #[doc="
        Recognize a namespaced block name.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::namespaced_block_name;

        let input = &b\"foo/bar baz\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\" baz\"[..],

                // The Abstract Syntax Tree.
                (&b\"foo\"[..], &b\"bar\"[..])
            )
        );

        assert_eq!(namespaced_block_name(input), output);
        ```
    "],
    pub namespaced_block_name<Input, (Input, Input)>,
    tuple!(
        block_name_part,
        preceded!(
            tag!("/"),
            block_name_part
        )
    )
);

named_attr!(
    #[doc="
        Recognize a globally-namespaced block name.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::core_block_name;

        let input = &b\"foo bar\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\" bar\"[..],

                // The Abstract Syntax Tree.
                (&b\"core\"[..], &b\"foo\"[..])
            )
        );

        assert_eq!(core_block_name(input), output);
        ```
    "],
    pub core_block_name<Input, (Input, Input)>,
    map_res!(
        block_name_part,
        |block_name_part| -> Result<(Input, Input), ()> {
            Ok((&b"core"[..], block_name_part))
        }
    )
);

named_attr!(
    #[doc="
        Recognize a block name part.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::block_name_part;

        let input = &b\"foo bar\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\" bar\"[..],

                // The parsed data.
                &b\"foo\"[..]
            )
        );

        assert_eq!(block_name_part(input), output);
        ```
    "],
    pub block_name_part,
    recognize!(
        pair!(
            is_a!("abcdefghijklmnopqrstuvwxyz"),
            take_while!(combinators::is_alphanumeric_extended)
        )
    )
);

named_attr!(
    #[doc="
        Recognize block attributes.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::block_attributes;

        let input = &b\"{\\\"foo\\\": \\\"bar\\\"}-->\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\"-->\"[..],

                // The parsed data.
                &b\"{\\\"foo\\\": \\\"bar\\\"}\"[..]
            )
        );

        assert_eq!(block_attributes(input), output);
        ```
    "],
    pub block_attributes,
    preceded!(
        peek!(tag!("{")),
        take_till_terminated!(
            "}",
            preceded!(
                opt!(whitespaces),
                alt_complete!(
                    tag!("/-->") |
                    tag!("-->")
                )
            )
        )
    )
);

named_attr!(
    #[doc="
        Recognize whitespaces.

        # Examples

        ```
        extern crate gutenberg_post_parser;

        use gutenberg_post_parser::parser::whitespaces;

        let input = &b\" \\n\\r\\t xyz\"[..];
        let output = Ok(
            (
                // The remaining data.
                &b\"xyz\"[..],

                // The parsed data.
                &b\" \\n\\r\\t \"[..]
            )
        );

        assert_eq!(whitespaces(input), output);
        ```
    "],
    pub whitespaces,
    is_a!(" \n\r\t")
);


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::ast::Node;

    #[test]
    fn test_block_list() {
        let input = &b"abc <!-- wp:foo --><!-- wp:bar /--> def <!-- /wp:foo --> ghi"[..];
        let output = Ok(
            (
                &b""[..],
                vec![
                    Node::Phrase(&b"abc "[..]),
                    Node::Block {
                        name: (&b"core"[..], &b"foo"[..]),
                        attributes: None,
                        children: vec![
                            Node::Block {
                                name: (&b"core"[..], &b"bar"[..]),
                                attributes: None,
                                children: vec![]
                            },
                            Node::Phrase(&b" def "[..])
                        ]
                    },
                    Node::Phrase(&b" ghi"[..])
                ]
            )
        );

        assert_eq!(block_list(input), output);
    }

    #[test]
    fn test_block_balanced_default_namespace_without_attributes() {
        let input = &b"<!-- wp:foo --><!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_coerce_namespace_without_attributes() {
        let input = &b"<!-- wp:ns/foo --><!-- /wp:ns/foo -->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: None,
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_coerce_namespace_with_attributes() {
        let input = &b"<!-- wp:ns/foo {\"abc\": \"xyz\"} --><!-- /wp:ns/foo -->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: Some(&b"{\"abc\": \"xyz\"}"[..]),
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_with_children() {
        let input = &b"<!-- wp:foo --><!-- wp:bar {\"abc\": true} /--><!-- wp:baz --><!-- wp:qux /--><!-- /wp:baz --><!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                children: vec![
                    Node::Block {
                        name: (&b"core"[..], &b"bar"[..]),
                        attributes: Some(&b"{\"abc\": true}"[..]),
                        children: vec![]
                    },
                    Node::Block {
                        name: (&b"core"[..], &b"baz"[..]),
                        attributes: None,
                        children: vec![
                            Node::Block {
                                name: (&b"core"[..], &b"qux"[..]),
                                attributes: None,
                                children: vec![]
                            }
                        ]
                    }
                ]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_balanced_with_phrasing_children() {
        let input = &b"<!-- wp:foo --> abc <!-- wp:bar {\"abc\": true} /--> def <!-- wp:baz --> ghi <!-- wp:qux /--> jkl <!-- /wp:baz --> mno <!-- /wp:foo -->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                children: vec![
                    Node::Phrase(&b" abc "[..]),
                    Node::Block {
                        name: (&b"core"[..], &b"bar"[..]),
                        attributes: Some(&b"{\"abc\": true}"[..]),
                        children: vec![]
                    },
                    Node::Phrase(&b" def "[..]),
                    Node::Block {
                        name: (&b"core"[..], &b"baz"[..]),
                        attributes: None,
                        children: vec![
                            Node::Phrase(&b" ghi "[..]),
                            Node::Block {
                                name: (&b"core"[..], &b"qux"[..]),
                                attributes: None,
                                children: vec![]
                            },
                            Node::Phrase(&b" jkl "[..]),
                        ]
                    },
                    Node::Phrase(&b" mno "[..])
                ]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_default_namespace_without_attributes() {
        let input = &b"<!-- wp:foo /-->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"core"[..], &b"foo"[..]),
                attributes: None,
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_coerce_namespace_without_attributes() {
        let input = &b"<!-- wp:ns/foo /-->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: None,
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_block_void_coerce_namespace_with_attributes() {
        let input = &b"<!-- wp:ns/foo {\"abc\": \"xyz\"} /-->"[..];
        let output = Ok((
            &b""[..],
            Node::Block {
                name: (&b"ns"[..], &b"foo"[..]),
                attributes: Some(&b"{\"abc\": \"xyz\"}"[..]),
                children: vec![]
            }
        ));

        assert_eq!(block(input), output);
    }

    #[test]
    fn test_namespaced_block_name() {
        let input = &b"foo_bar/baz42 x"[..];
        let output = Ok((&b" x"[..], (&b"foo_bar"[..], &b"baz42"[..])));

        assert_eq!(namespaced_block_name(input), output);
        assert_eq!(block_name(input), output);
    }

    #[test]
    fn test_core_block_name() {
        let input = &b"foo x"[..];
        let output = Ok((&b" x"[..], (&b"core"[..], &b"foo"[..])));

        assert_eq!(core_block_name(input), output);
        assert_eq!(block_name(input), output);
    }

    #[test]
    fn test_block_name_part_shortest() {
        let input = &b"a x"[..];
        let output = Ok((&b" x"[..], &b"a"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part_only_alpha() {
        let input = &b"abc xyz"[..];
        let output = Ok((&b" xyz"[..], &b"abc"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part_only_alphanumeric() {
        let input = &b"a0b1c xyz"[..];
        let output = Ok((&b" xyz"[..], &b"a0b1c"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_name_part() {
        let input = &b"a0b_1c- xyz"[..];
        let output = Ok((&b" xyz"[..], &b"a0b_1c-"[..]));

        assert_eq!(block_name_part(input), output);
    }

    #[test]
    fn test_block_attributes_simple_object() {
        let input = &b"{\"foo\": \"bar\"}-->"[..];
        let output = Ok((&b"-->"[..], &b"{\"foo\": \"bar\"}"[..]));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_object() {
        let input = &b"{\"foo\": \"bar\", \"baz\": [1, 2]}-->"[..];
        let output = Ok((&b"-->"[..], &b"{\"foo\": \"bar\", \"baz\": [1, 2]}"[..]));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_nested_objects() {
        let input = &b"{\"foo\": {\"bar\": \"baz\"} }-->"[..];
        let output = Ok((&b"-->"[..], &b"{\"foo\": {\"bar\": \"baz\"} }"[..]));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_surrounded_by_spaces() {
        let input = &b"{\"foo\": true} \t\r\n-->"[..];
        let output = Ok((&b" \t\r\n-->"[..], &b"{\"foo\": true}"[..]));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_block_attributes_object_with_auto_close() {
        let input = &b"{\"foo\": \"bar\", \"baz\": [1, 2]}/-->"[..];
        let output = Ok((&b"/-->"[..], &b"{\"foo\": \"bar\", \"baz\": [1, 2]}"[..]));

        assert_eq!(block_attributes(input), output);
    }

    #[test]
    fn test_whitespaces() {
        let input = &b" \n\r\t xyz"[..];
        let output = Ok((&b"xyz"[..], &b" \n\r\t "[..]));

        assert_eq!(whitespaces(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok() {
        named!(
            parser,
            take_till_terminated!(
                "d",
                tag!("c")
            )
        );

        let input = &b"abcdcba"[..];
        let output: ::nom::IResult<_, _> = Ok((&b"cba"[..], &b"abcd"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_at_position_0() {
        named!(
            parser,
            take_till_terminated!(
                "a",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"bcdcba"[..], &b"a"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_at_position_eof_minus_one() {
        named!(
            parser,
            take_till_terminated!(
                "b",
                tag!("a")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"a"[..], &b"abcdcb"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_ok_with_multiple_substring() {
        named!(
            parser,
            take_till_terminated!(
                "c",
                tag!("b")
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((&b"ba"[..], &b"abcdc"[..]));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_error() {
        named!(
            parser,
            take_till_terminated!(
                "a",
                tag!("z")
            )
        );

        use ::nom::{ErrorKind, Needed, need_more_err};

        let input = &b"abcdcba"[..];
        let output = need_more_err(input, Needed::Unknown, ErrorKind::Custom(42u32));

        assert_eq!(parser(input), output);
    }

    #[test]
    fn test_take_till_terminated_optional() {
        named!(
            parser<Input, Option<Input>>,
            opt!(
                complete!(
                    take_till_terminated!(
                        "a",
                        tag!("z")
                    )
                )
            )
        );

        let input = &b"abcdcba"[..];
        let output = Ok((input, None));

        assert_eq!(parser(input), output);
    }
}

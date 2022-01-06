#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate plex;
use plex::{lexer, parser};
pub enum Token {
    Ident(String),
    Print,
    Integer(i64),
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semi,
    Whitespace,
    Comment,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Token {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Token::Ident(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Ident");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Token::Print,) => ::core::fmt::Formatter::write_str(f, "Print"),
            (&Token::Integer(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&Token::Equals,) => ::core::fmt::Formatter::write_str(f, "Equals"),
            (&Token::Plus,) => ::core::fmt::Formatter::write_str(f, "Plus"),
            (&Token::Minus,) => ::core::fmt::Formatter::write_str(f, "Minus"),
            (&Token::Star,) => ::core::fmt::Formatter::write_str(f, "Star"),
            (&Token::Slash,) => ::core::fmt::Formatter::write_str(f, "Slash"),
            (&Token::LParen,) => ::core::fmt::Formatter::write_str(f, "LParen"),
            (&Token::RParen,) => ::core::fmt::Formatter::write_str(f, "RParen"),
            (&Token::Semi,) => ::core::fmt::Formatter::write_str(f, "Semi"),
            (&Token::Whitespace,) => ::core::fmt::Formatter::write_str(f, "Whitespace"),
            (&Token::Comment,) => ::core::fmt::Formatter::write_str(f, "Comment"),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Token {
    #[inline]
    fn clone(&self) -> Token {
        match (&*self,) {
            (&Token::Ident(ref __self_0),) => {
                Token::Ident(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Token::Print,) => Token::Print,
            (&Token::Integer(ref __self_0),) => {
                Token::Integer(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Token::Equals,) => Token::Equals,
            (&Token::Plus,) => Token::Plus,
            (&Token::Minus,) => Token::Minus,
            (&Token::Star,) => Token::Star,
            (&Token::Slash,) => Token::Slash,
            (&Token::LParen,) => Token::LParen,
            (&Token::RParen,) => Token::RParen,
            (&Token::Semi,) => Token::Semi,
            (&Token::Whitespace,) => Token::Whitespace,
            (&Token::Comment,) => Token::Comment,
        }
    }
}
fn main() {
    fn next_token<'a>(input: &'a str) -> Option<(Token, &'a str)> {
        #[allow(non_camel_case_types)]
        enum State {
            S_,
            S__9_,
            S__28_,
            S__29_,
            S__2a_,
            S__2b_,
            S__2d_,
            S__2f_,
            S_0,
            S__3b_,
            S__3d_,
            S_pri,
            S___,
            S__2f__2f_,
            S__2f__2a__2a__2f_,
            S_print,
            S__2f__2a__2a_,
            S_prin,
            S__2f__2a_,
            Error,
            S_pr,
            S_p,
            S_A,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for State {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                {
                    *self
                }
            }
        }
        fn transition(state: State, ch: char) -> State {
            match state {
                State::S_ => match ch {
                    '\u{9}'..='\u{a}' => State::S__9_,
                    '\u{d}' => State::S__9_,
                    '\u{20}' => State::S__9_,
                    '\u{28}' => State::S__28_,
                    '\u{29}' => State::S__29_,
                    '\u{2a}' => State::S__2a_,
                    '\u{2b}' => State::S__2b_,
                    '\u{2d}' => State::S__2d_,
                    '\u{2f}' => State::S__2f_,
                    '\u{30}'..='\u{39}' => State::S_0,
                    '\u{3b}' => State::S__3b_,
                    '\u{3d}' => State::S__3d_,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{6f}' => State::S_A,
                    '\u{70}' => State::S_p,
                    '\u{71}'..='\u{7a}' => State::S_A,
                    _ => State::S___,
                },
                State::S__9_ => match ch {
                    '\u{9}'..='\u{a}' => State::S__9_,
                    '\u{d}' => State::S__9_,
                    '\u{20}' => State::S__9_,
                    _ => State::Error,
                },
                State::S__28_ => match ch {
                    _ => State::Error,
                },
                State::S__29_ => match ch {
                    _ => State::Error,
                },
                State::S__2a_ => match ch {
                    _ => State::Error,
                },
                State::S__2b_ => match ch {
                    _ => State::Error,
                },
                State::S__2d_ => match ch {
                    _ => State::Error,
                },
                State::S__2f_ => match ch {
                    '\u{2a}' => State::S__2f__2a_,
                    '\u{2f}' => State::S__2f__2f_,
                    _ => State::Error,
                },
                State::S_0 => match ch {
                    '\u{30}'..='\u{39}' => State::S_0,
                    _ => State::Error,
                },
                State::S__3b_ => match ch {
                    _ => State::Error,
                },
                State::S__3d_ => match ch {
                    _ => State::Error,
                },
                State::S_pri => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{6d}' => State::S_A,
                    '\u{6e}' => State::S_prin,
                    '\u{6f}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
                State::S___ => match ch {
                    _ => State::Error,
                },
                State::S__2f__2f_ => match ch {
                    '\u{a}' => State::Error,
                    _ => State::S__2f__2f_,
                },
                State::S__2f__2a__2a__2f_ => match ch {
                    _ => State::Error,
                },
                State::S_print => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
                State::S__2f__2a__2a_ => match ch {
                    '\u{2a}' => State::S__2f__2a__2a_,
                    '\u{2f}' => State::S__2f__2a__2a__2f_,
                    _ => State::S__2f__2a_,
                },
                State::S_prin => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{73}' => State::S_A,
                    '\u{74}' => State::S_print,
                    '\u{75}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
                State::S__2f__2a_ => match ch {
                    '\u{2a}' => State::S__2f__2a__2a_,
                    _ => State::S__2f__2a_,
                },
                State::Error => match ch {
                    _ => State::Error,
                },
                State::S_pr => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{68}' => State::S_A,
                    '\u{69}' => State::S_pri,
                    '\u{6a}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
                State::S_p => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{71}' => State::S_A,
                    '\u{72}' => State::S_pr,
                    '\u{73}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
                State::S_A => match ch {
                    '\u{30}'..='\u{39}' => State::S_A,
                    '\u{41}'..='\u{5a}' => State::S_A,
                    '\u{5f}' => State::S_A,
                    '\u{61}'..='\u{7a}' => State::S_A,
                    _ => State::Error,
                },
            }
        }
        fn accepting(state: State) -> Option<u32> {
            match state {
                State::S__9_ => Some(0u32),
                State::S__28_ => Some(11u32),
                State::S__29_ => Some(12u32),
                State::S__2a_ => Some(9u32),
                State::S__2b_ => Some(7u32),
                State::S__2d_ => Some(8u32),
                State::S__2f_ => Some(10u32),
                State::S_0 => Some(4u32),
                State::S__3b_ => Some(13u32),
                State::S__3d_ => Some(6u32),
                State::S_pri => Some(5u32),
                State::S___ => Some(14u32),
                State::S__2f__2f_ => Some(2u32),
                State::S__2f__2a__2a__2f_ => Some(1u32),
                State::S_print => Some(3u32),
                State::S_prin => Some(5u32),
                State::S_pr => Some(5u32),
                State::S_p => Some(5u32),
                State::S_A => Some(5u32),
                _ => None,
            }
        }
        let mut state = State::S_;
        let mut remaining = input.char_indices();
        let mut last_match = None;
        loop {
            if let Some(which) = accepting(state) {
                last_match = Some((which, remaining.clone()));
            }
            if let State::Error = state {
                break;
            }
            if let Some((_, ch)) = remaining.next() {
                state = transition(state, ch);
            } else {
                break;
            }
        }
        if let Some((which, mut remaining)) = last_match {
            let ix = if let Some((ix, _)) = remaining.next() {
                ix
            } else {
                input.len()
            };
            let text = &input[..ix];
            let rule_result = match which {
                0u32 => Token::Whitespace,
                1u32 => Token::Comment,
                2u32 => Token::Comment,
                3u32 => Token::Print,
                4u32 => {
                    if let Ok(i) = text.parse() {
                        Token::Integer(i)
                    } else {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["integer ", " is out of range"],
                            &match (&text,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ))
                    }
                }
                5u32 => Token::Ident(text.to_owned()),
                6u32 => Token::Equals,
                7u32 => Token::Plus,
                8u32 => Token::Minus,
                9u32 => Token::Star,
                10u32 => Token::Slash,
                11u32 => Token::LParen,
                12u32 => Token::RParen,
                13u32 => Token::Semi,
                14u32 => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["unexpected character: "],
                    &match (&text,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                )),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            Some((rule_result, &input[ix..]))
        } else {
            None
        }
    }
}

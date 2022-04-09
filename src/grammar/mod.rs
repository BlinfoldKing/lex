/*
 * TODO: write custom parser,
 * using nom for quicker prototyping
 */
pub mod token;

pub mod atom;
pub mod atom_tests;
pub mod boolean;
pub mod boolean_tests;
pub mod comment;
pub mod comment_tests;
pub mod inline_comment;
pub mod inline_comment_tests;
pub mod keyword;
pub mod keyword_tests;
pub mod list;
pub mod list_tests;
pub mod number;
pub mod number_tests;
pub mod operator;
pub mod operator_tests;
pub mod string;
pub mod string_tests;
pub mod variable;
pub mod variable_tests;
pub mod wildcard;
pub mod wildcard_tests;

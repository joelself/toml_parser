   Compiling tomllib v0.1.0 (file:///Users/joel.self/Projects/toml_parser)
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate nom;
extern crate regex;
mod ast {
    #[prelude_import]
    use std::prelude::v1::*;
    pub mod structs {
        #[prelude_import]
        use std::prelude::v1::*;
        use std::fmt;
        use std::fmt::Display;
        use std::option::Option;
        use nom::IResult;
        use types::{DateTime, TimeOffset, TimeOffsetAmount};
        /// Compares two Options that contain comparable structs
        ///
        /// # Examples
        ///
        /// ```
        /// # extern crate tomllib;
        /// let (a, b) = (Some("value"), Some("value"));
        /// assert!(tomllib::ast::structs::comp_opt(&a, &b));
        /// ```
        pub fn comp_opt<T: Eq>(left: &Option<T>, right: &Option<T>) -> bool {
            match (left, right) {
                (&Some(ref i), &Some(ref j)) if i == j => true,
                (&None, &None) => true,
                _ => false,
            }
        }
        pub enum ErrorCode {
            BasicString = 0,
            MLBasicString = 1,
            LiteralString = 2,
            MLLiteralString = 3,
        }
        pub struct MyResult<'a>(pub IResult<&'a str, Toml<'a>>);
        impl <'a> Display for MyResult<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let MyResult(ref res) = *self;
                match res {
                    &IResult::Done(_, ref o) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&o,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    ref a =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&a,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Debug::fmt)],
                                                               })),
                }
            }
        }
        pub struct Toml<'a> {
            pub exprs: Vec<NLExpression<'a>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Toml<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Toml { exprs: ref __self_0_0 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Toml<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Toml { exprs: ref __self_0_0 } => {
                        let mut builder = __arg_0.debug_struct("Toml");
                        let _ = builder.field("exprs", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Toml<'a> {
            fn eq(&self, other: &Toml<'a>) -> bool {
                self.exprs == other.exprs
            }
        }
        impl <'a> Display for Toml<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                for i in 0..self.exprs.len() - 1 {
                    match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                        static __STATIC_FMTSTR:
                                                                               &'static [&'static str]
                                                                               =
                                                                            &[""];
                                                                        __STATIC_FMTSTR
                                                                    },
                                                                    &match (&self.exprs[i],)
                                                                         {
                                                                         (__arg0,)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                     })) {
                        ::std::result::Result::Ok(val) => val,
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(::std::convert::From::from(err))
                        }
                    };
                }
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &[""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.exprs[self.exprs.len()
                                                                                  -
                                                                                  1],)
                                                               {
                                                               (__arg0,) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct NLExpression<'a> {
            pub nl: &'a str,
            pub expr: Expression<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for NLExpression<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    NLExpression { nl: ref __self_0_0, expr: ref __self_0_1 }
                    => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for NLExpression<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    NLExpression { nl: ref __self_0_0, expr: ref __self_0_1 }
                    => {
                        let mut builder =
                            __arg_0.debug_struct("NLExpression");
                        let _ = builder.field("nl", &&(*__self_0_0));
                        let _ = builder.field("expr", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for NLExpression<'a> {
            fn eq(&self, other: &NLExpression<'a>) -> bool {
                self.nl == other.nl && self.expr == other.expr
            }
        }
        impl <'a> Display for NLExpression<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.nl,
                                                                  &self.expr)
                                                               {
                                                               (__arg0,
                                                                __arg1) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct Expression<'a> {
            pub ws: WSSep<'a>,
            pub keyval: Option<KeyVal<'a>>,
            pub table: Option<TableType<'a>>,
            pub comment: Option<Comment<'a>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Expression<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Expression {
                    ws: ref __self_0_0,
                    keyval: ref __self_0_1,
                    table: ref __self_0_2,
                    comment: ref __self_0_3 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                        (*__self_0_3).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Expression<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Expression {
                    ws: ref __self_0_0,
                    keyval: ref __self_0_1,
                    table: ref __self_0_2,
                    comment: ref __self_0_3 } => {
                        let mut builder = __arg_0.debug_struct("Expression");
                        let _ = builder.field("ws", &&(*__self_0_0));
                        let _ = builder.field("keyval", &&(*__self_0_1));
                        let _ = builder.field("table", &&(*__self_0_2));
                        let _ = builder.field("comment", &&(*__self_0_3));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Expression<'a> {
            fn eq(&self, other: &Expression<'a>) -> bool {
                self.ws == other.ws && comp_opt(&self.keyval, &other.keyval)
                    && comp_opt(&self.table, &other.table) &&
                    comp_opt(&self.comment, &other.comment)
            }
        }
        impl <'a> Display for Expression<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match (&self.ws, &self.keyval, &self.table, &self.comment) {
                    (ws, &None, &None, &None) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,)
                                                                   {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (ws, &None, &None, &Some(ref c)) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,
                                                                      &c) {
                                                                   (__arg0,
                                                                    __arg1) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (ws, &Some(ref k), &None, &Some(ref c)) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        "",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,
                                                                      &k,
                                                                      &ws.ws2,
                                                                      &c) {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2,
                                                                    __arg3) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg3,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (ws, &Some(ref k), &None, &None) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,
                                                                      &k,
                                                                      &ws.ws2)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (ws, &None, &Some(ref t), &Some(ref c)) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        "",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,
                                                                      &t,
                                                                      &ws.ws2,
                                                                      &c) {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2,
                                                                    __arg3) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg3,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (ws, &None, &Some(ref t), &None) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&ws.ws1,
                                                                      &t,
                                                                      &ws.ws2)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    _ => {
                        ::std::rt::begin_unwind_fmt(::std::fmt::Arguments::new_v1({
                                                                                      static __STATIC_FMTSTR:
                                                                                             &'static [&'static str]
                                                                                             =
                                                                                          &["Invalid expression: ws1: \"",
                                                                                            "\", ws2: \"",
                                                                                            "\", keyval: ",
                                                                                            ", table: ",
                                                                                            ", comment: "];
                                                                                      __STATIC_FMTSTR
                                                                                  },
                                                                                  &match (&self.ws.ws1,
                                                                                          &self.ws.ws2,
                                                                                          &self.keyval,
                                                                                          &self.table,
                                                                                          &self.comment)
                                                                                       {
                                                                                       (__arg0,
                                                                                        __arg1,
                                                                                        __arg2,
                                                                                        __arg3,
                                                                                        __arg4)
                                                                                       =>
                                                                                       [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                    ::std::fmt::Display::fmt),
                                                                                        ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                    ::std::fmt::Display::fmt),
                                                                                        ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                        ::std::fmt::ArgumentV1::new(__arg3,
                                                                                                                    ::std::fmt::Debug::fmt),
                                                                                        ::std::fmt::ArgumentV1::new(__arg4,
                                                                                                                    ::std::fmt::Debug::fmt)],
                                                                                   }),
                                                    {
                                                        static _FILE_LINE:
                                                               (&'static str,
                                                                u32) =
                                                            ("src/ast/structs.rs",
                                                             112u32);
                                                        &_FILE_LINE
                                                    })
                    }
                }
            }
        }
        pub enum StrType { Basic, MLBasic, Literal, MLLiteral, }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::PartialEq for StrType {
            #[inline]
            fn eq(&self, __arg_0: &StrType) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as i32;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as i32;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&StrType::Basic, &StrType::Basic) => true,
                            (&StrType::MLBasic, &StrType::MLBasic) => true,
                            (&StrType::Literal, &StrType::Literal) => true,
                            (&StrType::MLLiteral, &StrType::MLLiteral) =>
                            true,
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { false }
                }
            }
            #[inline]
            fn ne(&self, __arg_0: &StrType) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*self)
                        } as i32;
                    let __arg_1_vi =
                        unsafe {
                            ::std::intrinsics::discriminant_value(&*__arg_0)
                        } as i32;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*__arg_0) {
                            (&StrType::Basic, &StrType::Basic) => false,
                            (&StrType::MLBasic, &StrType::MLBasic) => false,
                            (&StrType::Literal, &StrType::Literal) => false,
                            (&StrType::MLLiteral, &StrType::MLLiteral) =>
                            false,
                            _ => unsafe { ::std::intrinsics::unreachable() }
                        }
                    } else { true }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::cmp::Eq for StrType {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match (&*self,) {
                    (&StrType::Basic,) => { }
                    (&StrType::MLBasic,) => { }
                    (&StrType::Literal,) => { }
                    (&StrType::MLLiteral,) => { }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for StrType {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&StrType::Basic,) => {
                        let mut builder = __arg_0.debug_tuple("Basic");
                        builder.finish()
                    }
                    (&StrType::MLBasic,) => {
                        let mut builder = __arg_0.debug_tuple("MLBasic");
                        builder.finish()
                    }
                    (&StrType::Literal,) => {
                        let mut builder = __arg_0.debug_tuple("Literal");
                        builder.finish()
                    }
                    (&StrType::MLLiteral,) => {
                        let mut builder = __arg_0.debug_tuple("MLLiteral");
                        builder.finish()
                    }
                }
            }
        }
        pub enum Value<'a> {
            Integer(&'a str),
            Float(&'a str),
            Boolean(&'a str),
            DateTime(DateTime<'a>),
            Array(Box<Array<'a>>),
            String(&'a str, StrType),
            InlineTable(Box<InlineTable<'a>>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Value<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match (&*self,) {
                    (&Value::Integer(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&Value::Float(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&Value::Boolean(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&Value::DateTime(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&Value::Array(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&Value::String(ref __self_0, ref __self_1),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                        (*__self_1).assert_receiver_is_total_eq();
                    }
                    (&Value::InlineTable(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Value<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&Value::Integer(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Integer");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Value::Float(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Float");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Value::Boolean(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Boolean");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Value::DateTime(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("DateTime");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Value::Array(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Array");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&Value::String(ref __self_0, ref __self_1),) => {
                        let mut builder = __arg_0.debug_tuple("String");
                        let _ = builder.field(&&(*__self_0));
                        let _ = builder.field(&&(*__self_1));
                        builder.finish()
                    }
                    (&Value::InlineTable(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("InlineTable");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Value<'a> {
            fn eq(&self, other: &Value<'a>) -> bool {
                match (self, other) {
                    (&Value::Integer(ref i), &Value::Integer(ref j)) if i == j
                    => true,
                    (&Value::Float(ref i), &Value::Float(ref j)) if i == j =>
                    true,
                    (&Value::Boolean(ref i), &Value::Boolean(ref j)) if i == j
                    => true,
                    (&Value::DateTime(ref i), &Value::DateTime(ref j)) if
                    i == j => true,
                    (&Value::Array(ref i), &Value::Array(ref j)) if i == j =>
                    true,
                    (&Value::String(ref i, ref t),
                     &Value::String(ref j, ref s)) if i == j => true,
                    (&Value::InlineTable(ref i), &Value::InlineTable(ref j))
                    if i == j => true,
                    _ => false,
                }
            }
        }
        impl <'a> Display for Value<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    &Value::Integer(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &Value::Float(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &Value::Boolean(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &Value::DateTime(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &Value::Array(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &Value::String(ref i, ref t) => {
                        match t {
                            &StrType::Basic =>
                            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                          static __STATIC_FMTSTR:
                                                                                 &'static [&'static str]
                                                                                 =
                                                                              &["\"",
                                                                                "\""];
                                                                          __STATIC_FMTSTR
                                                                      },
                                                                      &match (&i,)
                                                                           {
                                                                           (__arg0,)
                                                                           =>
                                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                        ::std::fmt::Display::fmt)],
                                                                       })),
                            &StrType::MLBasic =>
                            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                          static __STATIC_FMTSTR:
                                                                                 &'static [&'static str]
                                                                                 =
                                                                              &["\"\"\"",
                                                                                "\"\"\""];
                                                                          __STATIC_FMTSTR
                                                                      },
                                                                      &match (&i,)
                                                                           {
                                                                           (__arg0,)
                                                                           =>
                                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                        ::std::fmt::Display::fmt)],
                                                                       })),
                            &StrType::Literal =>
                            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                          static __STATIC_FMTSTR:
                                                                                 &'static [&'static str]
                                                                                 =
                                                                              &["\'",
                                                                                "\'"];
                                                                          __STATIC_FMTSTR
                                                                      },
                                                                      &match (&i,)
                                                                           {
                                                                           (__arg0,)
                                                                           =>
                                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                        ::std::fmt::Display::fmt)],
                                                                       })),
                            &StrType::MLLiteral =>
                            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                          static __STATIC_FMTSTR:
                                                                                 &'static [&'static str]
                                                                                 =
                                                                              &["\'\'\'",
                                                                                "\'\'\'"];
                                                                          __STATIC_FMTSTR
                                                                      },
                                                                      &match (&i,)
                                                                           {
                                                                           (__arg0,)
                                                                           =>
                                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                        ::std::fmt::Display::fmt)],
                                                                       })),
                        }
                    }
                    &Value::InlineTable(ref i) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&i,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
        pub enum TableType<'a> { Standard(Table<'a>), Array(Table<'a>), }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for TableType<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match (&*self,) {
                    (&TableType::Standard(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&TableType::Array(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for TableType<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&TableType::Standard(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Standard");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&TableType::Array(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Array");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for TableType<'a> {
            fn eq(&self, other: &TableType<'a>) -> bool {
                match (self, other) {
                    (&TableType::Standard(ref i), &TableType::Standard(ref j))
                    if i == j => true,
                    (&TableType::Array(ref i), &TableType::Array(ref j)) if
                    i == j => true,
                    _ => false,
                }
            }
        }
        impl <'a> Display for TableType<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    &TableType::Standard(ref t) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["[",
                                                                        "]"];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&t,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &TableType::Array(ref t) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["[[",
                                                                        "]]"];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&t,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
        impl <'a> PartialEq for TimeOffset<'a> {
            fn eq(&self, other: &TimeOffset<'a>) -> bool {
                match (self, other) {
                    (&TimeOffset::Z, &TimeOffset::Z) => true,
                    (&TimeOffset::Time(ref i), &TimeOffset::Time(ref j)) if
                    (i == j) => true,
                    _ => false,
                }
            }
        }
        impl <'a> Display for TimeOffset<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    &TimeOffset::Z =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["Z"];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match () {
                                                                   () => [],
                                                               })),
                    &TimeOffset::Time(ref t) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&t,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
        pub struct Comment<'a> {
            pub text: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Comment<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Comment { text: ref __self_0_0 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Comment<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Comment { text: ref __self_0_0 } => {
                        let mut builder = __arg_0.debug_struct("Comment");
                        let _ = builder.field("text", &&(*__self_0_0));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Comment<'a> {
            fn eq(&self, other: &Comment<'a>) -> bool {
                self.text == other.text
            }
        }
        impl <'a> Display for Comment<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["#"];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.text,)
                                                               {
                                                               (__arg0,) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct WSSep<'a> {
            pub ws1: &'a str,
            pub ws2: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for WSSep<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    WSSep { ws1: ref __self_0_0, ws2: ref __self_0_1 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for WSSep<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    WSSep { ws1: ref __self_0_0, ws2: ref __self_0_1 } => {
                        let mut builder = __arg_0.debug_struct("WSSep");
                        let _ = builder.field("ws1", &&(*__self_0_0));
                        let _ = builder.field("ws2", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for WSSep<'a> {
            fn eq(&self, other: &WSSep<'a>) -> bool {
                self.ws1 == other.ws1 && self.ws2 == other.ws2
            }
        }
        pub struct KeyVal<'a> {
            pub key: &'a str,
            pub keyval_sep: WSSep<'a>,
            pub val: Value<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for KeyVal<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    KeyVal {
                    key: ref __self_0_0,
                    keyval_sep: ref __self_0_1,
                    val: ref __self_0_2 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for KeyVal<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    KeyVal {
                    key: ref __self_0_0,
                    keyval_sep: ref __self_0_1,
                    val: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("KeyVal");
                        let _ = builder.field("key", &&(*__self_0_0));
                        let _ = builder.field("keyval_sep", &&(*__self_0_1));
                        let _ = builder.field("val", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for KeyVal<'a> {
            fn eq(&self, other: &KeyVal<'a>) -> bool {
                self.key == other.key && self.keyval_sep == other.keyval_sep
                    && self.val == other.val
            }
        }
        impl <'a> Display for KeyVal<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "",
                                                                    "=", ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.key,
                                                                  &self.keyval_sep.ws1,
                                                                  &self.keyval_sep.ws2,
                                                                  &self.val) {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2,
                                                                __arg3) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg3,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct WSKeySep<'a> {
            pub ws: WSSep<'a>,
            pub key: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for WSKeySep<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    WSKeySep { ws: ref __self_0_0, key: ref __self_0_1 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for WSKeySep<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    WSKeySep { ws: ref __self_0_0, key: ref __self_0_1 } => {
                        let mut builder = __arg_0.debug_struct("WSKeySep");
                        let _ = builder.field("ws", &&(*__self_0_0));
                        let _ = builder.field("key", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for WSKeySep<'a> {
            fn eq(&self, other: &WSKeySep<'a>) -> bool {
                self.ws == other.ws && self.key == other.key
            }
        }
        impl <'a> Display for WSKeySep<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", ".",
                                                                    ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.ws.ws1,
                                                                  &self.ws.ws2,
                                                                  &self.key) {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct Table<'a> {
            pub ws: WSSep<'a>,
            pub key: &'a str,
            pub subkeys: Vec<WSKeySep<'a>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Table<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Table {
                    ws: ref __self_0_0,
                    key: ref __self_0_1,
                    subkeys: ref __self_0_2 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Table<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Table {
                    ws: ref __self_0_0,
                    key: ref __self_0_1,
                    subkeys: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("Table");
                        let _ = builder.field("ws", &&(*__self_0_0));
                        let _ = builder.field("key", &&(*__self_0_1));
                        let _ = builder.field("subkeys", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Table<'a> {
            fn eq(&self, other: &Table<'a>) -> bool {
                self.ws == other.ws && self.key == other.key &&
                    self.subkeys == other.subkeys
            }
        }
        impl <'a> Display for Table<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                    static __STATIC_FMTSTR:
                                                                           &'static [&'static str]
                                                                           =
                                                                        &["",
                                                                          ""];
                                                                    __STATIC_FMTSTR
                                                                },
                                                                &match (&self.ws.ws1,
                                                                        &self.key)
                                                                     {
                                                                     (__arg0,
                                                                      __arg1)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                  ::std::fmt::Display::fmt),
                                                                      ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 })) {
                    ::std::result::Result::Ok(val) => val,
                    ::std::result::Result::Err(err) => {
                        return ::std::result::Result::Err(::std::convert::From::from(err))
                    }
                };
                for key in &self.subkeys {
                    match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                        static __STATIC_FMTSTR:
                                                                               &'static [&'static str]
                                                                               =
                                                                            &[""];
                                                                        __STATIC_FMTSTR
                                                                    },
                                                                    &match (&key,)
                                                                         {
                                                                         (__arg0,)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                     })) {
                        ::std::result::Result::Ok(val) => val,
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(::std::convert::From::from(err))
                        }
                    };
                }
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &[""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.ws.ws2,)
                                                               {
                                                               (__arg0,) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct Time<'a> {
            pub hour: &'a str,
            pub minute: &'a str,
            pub second: &'a str,
            pub fraction: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Time<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Time {
                    hour: ref __self_0_0,
                    minute: ref __self_0_1,
                    second: ref __self_0_2,
                    fraction: ref __self_0_3 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                        (*__self_0_3).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Time<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Time {
                    hour: ref __self_0_0,
                    minute: ref __self_0_1,
                    second: ref __self_0_2,
                    fraction: ref __self_0_3 } => {
                        let mut builder = __arg_0.debug_struct("Time");
                        let _ = builder.field("hour", &&(*__self_0_0));
                        let _ = builder.field("minute", &&(*__self_0_1));
                        let _ = builder.field("second", &&(*__self_0_2));
                        let _ = builder.field("fraction", &&(*__self_0_3));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Time<'a> {
            fn eq(&self, other: &Time<'a>) -> bool {
                self.hour == other.hour && self.minute == other.minute &&
                    self.second == other.second &&
                    self.fraction == other.fraction
            }
        }
        impl <'a> Display for Time<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.fraction == "" {
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        ":",
                                                                        ":"];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.hour,
                                                                      &self.minute,
                                                                      &self.second)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt)],
                                                               }))
                } else {
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        ":",
                                                                        ":",
                                                                        "."];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.hour,
                                                                      &self.minute,
                                                                      &self.second,
                                                                      &self.fraction)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2,
                                                                    __arg3) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg3,
                                                                                                ::std::fmt::Display::fmt)],
                                                               }))
                }
            }
        }
        impl <'a> PartialEq for TimeOffsetAmount<'a> {
            fn eq(&self, other: &TimeOffsetAmount<'a>) -> bool {
                self.pos_neg == other.pos_neg && self.hour == other.hour &&
                    self.minute == other.minute
            }
        }
        impl <'a> Display for TimeOffsetAmount<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "",
                                                                    ":"];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.pos_neg,
                                                                  &self.hour,
                                                                  &self.minute)
                                                               {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct FullDate<'a> {
            pub year: &'a str,
            pub month: &'a str,
            pub day: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for FullDate<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    FullDate {
                    year: ref __self_0_0,
                    month: ref __self_0_1,
                    day: ref __self_0_2 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for FullDate<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    FullDate {
                    year: ref __self_0_0,
                    month: ref __self_0_1,
                    day: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("FullDate");
                        let _ = builder.field("year", &&(*__self_0_0));
                        let _ = builder.field("month", &&(*__self_0_1));
                        let _ = builder.field("day", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for FullDate<'a> {
            fn eq(&self, other: &FullDate<'a>) -> bool {
                self.year == other.year && self.month == other.month &&
                    self.day == other.day
            }
        }
        impl <'a> Display for FullDate<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "-",
                                                                    "-"];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.year,
                                                                  &self.month,
                                                                  &self.day) {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        impl <'a> PartialEq for DateTime<'a> {
            fn eq(&self, other: &DateTime<'a>) -> bool {
                self.year == other.year && self.month == other.month &&
                    self.day == other.day && self.hour == other.hour &&
                    self.minute == other.minute && self.second == other.second
                    && self.fraction == other.fraction &&
                    self.offset == other.offset
            }
        }
        impl <'a> Display for DateTime<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "-",
                                                                    "-", "T",
                                                                    ":", ":",
                                                                    ".", ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.year,
                                                                  &self.month,
                                                                  &self.day,
                                                                  &self.hour,
                                                                  &self.minute,
                                                                  &self.second,
                                                                  &self.fraction,
                                                                  &self.offset)
                                                               {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2,
                                                                __arg3,
                                                                __arg4,
                                                                __arg5,
                                                                __arg6,
                                                                __arg7) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg3,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg4,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg5,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg6,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg7,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct CommentNewLines<'a> {
            pub pre_ws_nl: &'a str,
            pub comment: Comment<'a>,
            pub newlines: &'a str,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for CommentNewLines<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    CommentNewLines {
                    pre_ws_nl: ref __self_0_0,
                    comment: ref __self_0_1,
                    newlines: ref __self_0_2 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for CommentNewLines<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    CommentNewLines {
                    pre_ws_nl: ref __self_0_0,
                    comment: ref __self_0_1,
                    newlines: ref __self_0_2 } => {
                        let mut builder =
                            __arg_0.debug_struct("CommentNewLines");
                        let _ = builder.field("pre_ws_nl", &&(*__self_0_0));
                        let _ = builder.field("comment", &&(*__self_0_1));
                        let _ = builder.field("newlines", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for CommentNewLines<'a> {
            fn eq(&self, other: &CommentNewLines<'a>) -> bool {
                self.pre_ws_nl == other.pre_ws_nl &&
                    self.comment == other.comment &&
                    self.newlines == other.newlines
            }
        }
        impl <'a> Display for CommentNewLines<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "",
                                                                    ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.pre_ws_nl,
                                                                  &self.comment,
                                                                  &self.newlines)
                                                               {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub enum CommentOrNewLines<'a> {
            Comment(CommentNewLines<'a>),
            NewLines(&'a str),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for CommentOrNewLines<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match (&*self,) {
                    (&CommentOrNewLines::Comment(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                    (&CommentOrNewLines::NewLines(ref __self_0),) => {
                        (*__self_0).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for CommentOrNewLines<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match (&*self,) {
                    (&CommentOrNewLines::Comment(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("Comment");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                    (&CommentOrNewLines::NewLines(ref __self_0),) => {
                        let mut builder = __arg_0.debug_tuple("NewLines");
                        let _ = builder.field(&&(*__self_0));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for CommentOrNewLines<'a> {
            fn eq(&self, other: &CommentOrNewLines<'a>) -> bool {
                match (self, other) {
                    (&CommentOrNewLines::Comment(ref i),
                     &CommentOrNewLines::Comment(ref j)) if i == j => true,
                    (&CommentOrNewLines::NewLines(ref i),
                     &CommentOrNewLines::NewLines(ref j)) if i == j => true,
                    _ => false,
                }
            }
        }
        impl <'a> Display for CommentOrNewLines<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    &CommentOrNewLines::Comment(ref c) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&c,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    &CommentOrNewLines::NewLines(ref n) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&n,) {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
        pub struct ArrayValue<'a> {
            pub val: Value<'a>,
            pub array_sep: Option<WSSep<'a>>,
            pub comment_nl: Option<CommentOrNewLines<'a>>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for ArrayValue<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    ArrayValue {
                    val: ref __self_0_0,
                    array_sep: ref __self_0_1,
                    comment_nl: ref __self_0_2 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                        (*__self_0_2).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for ArrayValue<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    ArrayValue {
                    val: ref __self_0_0,
                    array_sep: ref __self_0_1,
                    comment_nl: ref __self_0_2 } => {
                        let mut builder = __arg_0.debug_struct("ArrayValue");
                        let _ = builder.field("val", &&(*__self_0_0));
                        let _ = builder.field("array_sep", &&(*__self_0_1));
                        let _ = builder.field("comment_nl", &&(*__self_0_2));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for ArrayValue<'a> {
            fn eq(&self, other: &ArrayValue<'a>) -> bool {
                self.val == other.val &&
                    comp_opt(&self.array_sep, &other.array_sep) &&
                    comp_opt(&self.comment_nl, &other.comment_nl)
            }
        }
        impl <'a> Display for ArrayValue<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match (&self.array_sep, &self.comment_nl) {
                    (&Some(ref s), &None) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        ","];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.val,
                                                                      &s.ws1,
                                                                      &s.ws2)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (&Some(ref s), &Some(ref c)) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        "",
                                                                        ",",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.val,
                                                                      &s.ws1,
                                                                      &s.ws2,
                                                                      &c) {
                                                                   (__arg0,
                                                                    __arg1,
                                                                    __arg2,
                                                                    __arg3) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg2,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg3,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (&None, &Some(ref c)) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["",
                                                                        ""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.val,
                                                                      &c) {
                                                                   (__arg0,
                                                                    __arg1) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                    (&None, &None) =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &[""];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.val,)
                                                                   {
                                                                   (__arg0,)
                                                                   =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
        pub struct Array<'a> {
            pub values: Vec<ArrayValue<'a>>,
            pub ws: WSSep<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for Array<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    Array { values: ref __self_0_0, ws: ref __self_0_1 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for Array<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    Array { values: ref __self_0_0, ws: ref __self_0_1 } => {
                        let mut builder = __arg_0.debug_struct("Array");
                        let _ = builder.field("values", &&(*__self_0_0));
                        let _ = builder.field("ws", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for Array<'a> {
            fn eq(&self, other: &Array<'a>) -> bool {
                self.values == other.values && self.ws == other.ws
            }
        }
        impl <'a> Display for Array<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                    static __STATIC_FMTSTR:
                                                                           &'static [&'static str]
                                                                           =
                                                                        &["["];
                                                                    __STATIC_FMTSTR
                                                                },
                                                                &match (&self.ws.ws1,)
                                                                     {
                                                                     (__arg0,)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 })) {
                    ::std::result::Result::Ok(val) => val,
                    ::std::result::Result::Err(err) => {
                        return ::std::result::Result::Err(::std::convert::From::from(err))
                    }
                };
                for val in self.values.iter() {
                    match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                        static __STATIC_FMTSTR:
                                                                               &'static [&'static str]
                                                                               =
                                                                            &[""];
                                                                        __STATIC_FMTSTR
                                                                    },
                                                                    &match (&val,)
                                                                         {
                                                                         (__arg0,)
                                                                         =>
                                                                         [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                      ::std::fmt::Display::fmt)],
                                                                     })) {
                        ::std::result::Result::Ok(val) => val,
                        ::std::result::Result::Err(err) => {
                            return ::std::result::Result::Err(::std::convert::From::from(err))
                        }
                    };
                }
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "]"];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.ws.ws2,)
                                                               {
                                                               (__arg0,) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct TableKeyVal<'a> {
            pub keyval: KeyVal<'a>,
            pub kv_sep: WSSep<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for TableKeyVal<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    TableKeyVal {
                    keyval: ref __self_0_0, kv_sep: ref __self_0_1 } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for TableKeyVal<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    TableKeyVal {
                    keyval: ref __self_0_0, kv_sep: ref __self_0_1 } => {
                        let mut builder = __arg_0.debug_struct("TableKeyVal");
                        let _ = builder.field("keyval", &&(*__self_0_0));
                        let _ = builder.field("kv_sep", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for TableKeyVal<'a> {
            fn eq(&self, other: &TableKeyVal<'a>) -> bool {
                self.keyval == other.keyval && self.kv_sep == other.kv_sep
            }
        }
        impl <'a> Display for TableKeyVal<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_fmt(::std::fmt::Arguments::new_v1({
                                                              static __STATIC_FMTSTR:
                                                                     &'static [&'static str]
                                                                     =
                                                                  &["", "",
                                                                    ""];
                                                              __STATIC_FMTSTR
                                                          },
                                                          &match (&self.keyval,
                                                                  &self.kv_sep.ws1,
                                                                  &self.kv_sep.ws2)
                                                               {
                                                               (__arg0,
                                                                __arg1,
                                                                __arg2) =>
                                                               [::std::fmt::ArgumentV1::new(__arg0,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg1,
                                                                                            ::std::fmt::Display::fmt),
                                                                ::std::fmt::ArgumentV1::new(__arg2,
                                                                                            ::std::fmt::Display::fmt)],
                                                           }))
            }
        }
        pub struct InlineTable<'a> {
            pub keyvals: Option<Vec<TableKeyVal<'a>>>,
            pub ws: WSSep<'a>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::cmp::Eq for InlineTable<'a> {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                match *self {
                    InlineTable { keyvals: ref __self_0_0, ws: ref __self_0_1
                    } => {
                        (*__self_0_0).assert_receiver_is_total_eq();
                        (*__self_0_1).assert_receiver_is_total_eq();
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl <'a> ::std::fmt::Debug for InlineTable<'a> {
            fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
             -> ::std::fmt::Result {
                match *self {
                    InlineTable { keyvals: ref __self_0_0, ws: ref __self_0_1
                    } => {
                        let mut builder = __arg_0.debug_struct("InlineTable");
                        let _ = builder.field("keyvals", &&(*__self_0_0));
                        let _ = builder.field("ws", &&(*__self_0_1));
                        builder.finish()
                    }
                }
            }
        }
        impl <'a> PartialEq for InlineTable<'a> {
            fn eq(&self, other: &InlineTable<'a>) -> bool {
                comp_opt(&self.keyvals, &other.keyvals) && self.ws == other.ws
            }
        }
        fn write_table_vector<'a>(kvs: &Vec<TableKeyVal<'a>>, ws: &WSSep<'a>,
                                  f: &mut fmt::Formatter) -> fmt::Result {
            match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                static __STATIC_FMTSTR:
                                                                       &'static [&'static str]
                                                                       =
                                                                    &["{"];
                                                                __STATIC_FMTSTR
                                                            },
                                                            &match (&ws.ws1,)
                                                                 {
                                                                 (__arg0,) =>
                                                                 [::std::fmt::ArgumentV1::new(__arg0,
                                                                                              ::std::fmt::Display::fmt)],
                                                             })) {
                ::std::result::Result::Ok(val) => val,
                ::std::result::Result::Err(err) => {
                    return ::std::result::Result::Err(::std::convert::From::from(err))
                }
            };
            for i in 0..kvs.len() - 1 {
                match f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                    static __STATIC_FMTSTR:
                                                                           &'static [&'static str]
                                                                           =
                                                                        &["",
                                                                          ","];
                                                                    __STATIC_FMTSTR
                                                                },
                                                                &match (&kvs[i],)
                                                                     {
                                                                     (__arg0,)
                                                                     =>
                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                  ::std::fmt::Display::fmt)],
                                                                 })) {
                    ::std::result::Result::Ok(val) => val,
                    ::std::result::Result::Err(err) => {
                        return ::std::result::Result::Err(::std::convert::From::from(err))
                    }
                };
            }
            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                          static __STATIC_FMTSTR:
                                                                 &'static [&'static str]
                                                                 =
                                                              &["", "", "}"];
                                                          __STATIC_FMTSTR
                                                      },
                                                      &match (&kvs[kvs.len() -
                                                                       1],
                                                              &ws.ws2) {
                                                           (__arg0, __arg1) =>
                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                        ::std::fmt::Display::fmt),
                                                            ::std::fmt::ArgumentV1::new(__arg1,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
        impl <'a> Display for InlineTable<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match &self.keyvals {
                    &Some(ref k) => write_table_vector(k, &self.ws, f),
                    &None =>
                    f.write_fmt(::std::fmt::Arguments::new_v1({
                                                                  static __STATIC_FMTSTR:
                                                                         &'static [&'static str]
                                                                         =
                                                                      &["{",
                                                                        "",
                                                                        "}"];
                                                                  __STATIC_FMTSTR
                                                              },
                                                              &match (&self.ws.ws1,
                                                                      &self.ws.ws2)
                                                                   {
                                                                   (__arg0,
                                                                    __arg1) =>
                                                                   [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                ::std::fmt::Display::fmt),
                                                                    ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                ::std::fmt::Display::fmt)],
                                                               })),
                }
            }
        }
    }
}
mod toml {
    #[prelude_import]
    use std::prelude::v1::*;
    use ast::structs::{Toml, NLExpression, Expression, WSSep};
    use nom::eof;
    use parser::Parser;
    impl <'a> Parser<'a> {
        pub fn toml(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Toml, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let (tmp, res) = self.expression(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let expr = o;
                             match {
                                       let (tmp, res) =
                                           self.nl_expressions(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                    +
                                                                                    ((i).input_len()
                                                                                         -
                                                                                         i.input_len())
                                                                                    +
                                                                                    i)),
                                 ::nom::IResult::Done(i, o) => {
                                     let nl_exprs = o;
                                     ::nom::IResult::Done(i,
                                                          (|| {
                                                              let mut tmp =
                                                                  <[_]>::into_vec(::std::boxed::Box::new([NLExpression{nl:
                                                                                                                           "",
                                                                                                                       expr:
                                                                                                                           expr,}]));
                                                              tmp.extend(nl_exprs);
                                                              Toml{exprs:
                                                                       tmp,}
                                                          })())
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn nl_expressions(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Vec<NLExpression>, u32>) {
            (self,
             {
                 use nom::InputLength;
                 if (i).input_len() == 0 {
                     ::nom::IResult::Done(i, ::std::vec::Vec::new())
                 } else {
                     match {
                               let (tmp, res) = self.nl_expression(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(_) => {
                             ::nom::IResult::Done(i, ::std::vec::Vec::new())
                         }
                         ::nom::IResult::Incomplete(i) =>
                         ::nom::IResult::Incomplete(i),
                         ::nom::IResult::Done(i1, o1) => {
                             if i1.input_len() == 0 {
                                 ::nom::IResult::Done(i1,
                                                      <[_]>::into_vec(::std::boxed::Box::new([o1])))
                             } else {
                                 let mut res =
                                     ::std::vec::Vec::with_capacity(4);
                                 res.push(o1);
                                 let mut input = i1;
                                 let mut incomplete:
                                         ::std::option::Option<::nom::Needed> =
                                     ::std::option::Option::None;
                                 loop  {
                                     match {
                                               let (tmp, res) =
                                                   self.nl_expression(input);
                                               self = tmp;
                                               res
                                           } {
                                         ::nom::IResult::Done(i, o) => {
                                             if i.input_len() ==
                                                    input.input_len() {
                                                 break ;
                                             }
                                             res.push(o);
                                             input = i;
                                         }
                                         ::nom::IResult::Error(_) => {
                                             break ;
                                         }
                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                         => {
                                             incomplete =
                                                 ::std::option::Option::Some(::nom::Needed::Unknown);
                                             break ;
                                         }
                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                         => {
                                             incomplete =
                                                 ::std::option::Option::Some(::nom::Needed::Size(i
                                                                                                     +
                                                                                                     (i).input_len()
                                                                                                     -
                                                                                                     input.input_len()));
                                             break ;
                                         }
                                     }
                                     if input.input_len() == 0 { break ; }
                                 }
                                 match incomplete {
                                     ::std::option::Option::Some(i) =>
                                     ::nom::IResult::Incomplete(i),
                                     ::std::option::Option::None =>
                                     ::nom::IResult::Done(input, res),
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn nl_expression(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, NLExpression, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let (tmp, res) = self.newline(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let nl = o;
                             match {
                                       let (tmp, res) = self.expression(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                    +
                                                                                    ((i).input_len()
                                                                                         -
                                                                                         i.input_len())
                                                                                    +
                                                                                    i)),
                                 ::nom::IResult::Done(i, o) => {
                                     let expr = o;
                                     ::nom::IResult::Done(i,
                                                          (|| {
                                                              NLExpression{nl:
                                                                               nl,
                                                                           expr:
                                                                               expr,}
                                                          })())
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn expression(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Expression, u32>) {
            (self,
             {
                 {
                     let res =
                         {
                             match {
                                       let (tmp, res) = self.table_comment(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                i))
                                 }
                             }
                         };
                     match res {
                         ::nom::IResult::Done(_, _) => res,
                         ::nom::IResult::Incomplete(_) => res,
                         _ => {
                             let res =
                                 {
                                     match {
                                               let (tmp, res) =
                                                   self.keyval_comment(i);
                                               self = tmp;
                                               res
                                           } {
                                         ::nom::IResult::Done(i, o) =>
                                         ::nom::IResult::Done(i, o),
                                         ::nom::IResult::Error(e) =>
                                         ::nom::IResult::Error(e),
                                         ::nom::IResult::Incomplete(_) => {
                                             ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                        i))
                                         }
                                     }
                                 };
                             match res {
                                 ::nom::IResult::Done(_, _) => res,
                                 ::nom::IResult::Incomplete(_) => res,
                                 _ => {
                                     let res =
                                         {
                                             match {
                                                       let (tmp, res) =
                                                           self.ws_comment(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Done(i, o) =>
                                                 ::nom::IResult::Done(i, o),
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(_)
                                                 => {
                                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                i))
                                                 }
                                             }
                                         };
                                     match res {
                                         ::nom::IResult::Done(_, _) => res,
                                         ::nom::IResult::Incomplete(_) => res,
                                         _ => {
                                             match {
                                                       match {
                                                                 let (tmp,
                                                                      res) =
                                                                     self.ws_expr(i);
                                                                 self = tmp;
                                                                 res
                                                             } {
                                                           ::nom::IResult::Done(i,
                                                                                o)
                                                           =>
                                                           ::nom::IResult::Done(i,
                                                                                o),
                                                           ::nom::IResult::Error(e)
                                                           =>
                                                           ::nom::IResult::Error(e),
                                                           ::nom::IResult::Incomplete(_)
                                                           => {
                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                          i))
                                                           }
                                                       }
                                                   } {
                                                 ::nom::IResult::Done(i, o) =>
                                                 ::nom::IResult::Done(i, o),
                                                 ::nom::IResult::Incomplete(x)
                                                 =>
                                                 ::nom::IResult::Incomplete(x),
                                                 ::nom::IResult::Error(_) => {
                                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                                i))
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn ws_expr(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Expression, u32>) {
            (self,
             {
                 match { let (tmp, res) = self.ws(i); self = tmp; res } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let ws = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  Expression{ws:
                                                                 WSSep{ws1:
                                                                           ws,
                                                                       ws2:
                                                                           "",},
                                                             keyval: None,
                                                             table: None,
                                                             comment: None,}
                                              })())
                     }
                 }
             })
        }
        fn table_comment(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Expression, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.table(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let table = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.ws(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let ws2 = o;
                                                     {
                                                         let res =
                                                             {
                                                                 match {
                                                                           let (tmp,
                                                                                res) =
                                                                               self.comment(i);
                                                                           self
                                                                               =
                                                                               tmp;
                                                                           res
                                                                       } {
                                                                     ::nom::IResult::Done(i,
                                                                                          o)
                                                                     =>
                                                                     ::nom::IResult::Done(i,
                                                                                          o),
                                                                     ::nom::IResult::Error(e)
                                                                     =>
                                                                     ::nom::IResult::Error(e),
                                                                     ::nom::IResult::Incomplete(_)
                                                                     => {
                                                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                    i))
                                                                     }
                                                                 }
                                                             };
                                                         if let ::nom::IResult::Incomplete(inc)
                                                                = res {
                                                             match inc {
                                                                 ::nom::Needed::Unknown
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                 ::nom::Needed::Size(i)
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    i)),
                                                             }
                                                         } else {
                                                             let (comment,
                                                                  input) =
                                                                 if let ::nom::IResult::Done(i,
                                                                                             o)
                                                                        = res
                                                                        {
                                                                     (::std::option::Option::Some(o),
                                                                      i)
                                                                 } else {
                                                                     (::std::option::Option::None,
                                                                      i)
                                                                 };
                                                             ::nom::IResult::Done(input,
                                                                                  (||
                                                                                       {
                                                                                      Expression{ws:
                                                                                                     WSSep{ws1:
                                                                                                               ws1,
                                                                                                           ws2:
                                                                                                               ws2,},
                                                                                                 keyval:
                                                                                                     None,
                                                                                                 table:
                                                                                                     Some(table),
                                                                                                 comment:
                                                                                                     comment,}
                                                                                  })())
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn keyval_comment(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Expression, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.keyval(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let keyval = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.ws(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let ws2 = o;
                                                     {
                                                         let res =
                                                             {
                                                                 match {
                                                                           let (tmp,
                                                                                res) =
                                                                               self.comment(i);
                                                                           self
                                                                               =
                                                                               tmp;
                                                                           res
                                                                       } {
                                                                     ::nom::IResult::Done(i,
                                                                                          o)
                                                                     =>
                                                                     ::nom::IResult::Done(i,
                                                                                          o),
                                                                     ::nom::IResult::Error(e)
                                                                     =>
                                                                     ::nom::IResult::Error(e),
                                                                     ::nom::IResult::Incomplete(_)
                                                                     => {
                                                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                    i))
                                                                     }
                                                                 }
                                                             };
                                                         if let ::nom::IResult::Incomplete(inc)
                                                                = res {
                                                             match inc {
                                                                 ::nom::Needed::Unknown
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                 ::nom::Needed::Size(i)
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    i)),
                                                             }
                                                         } else {
                                                             let (comment,
                                                                  input) =
                                                                 if let ::nom::IResult::Done(i,
                                                                                             o)
                                                                        = res
                                                                        {
                                                                     (::std::option::Option::Some(o),
                                                                      i)
                                                                 } else {
                                                                     (::std::option::Option::None,
                                                                      i)
                                                                 };
                                                             ::nom::IResult::Done(input,
                                                                                  (||
                                                                                       {
                                                                                      Expression{ws:
                                                                                                     WSSep{ws1:
                                                                                                               ws1,
                                                                                                           ws2:
                                                                                                               ws2,},
                                                                                                 keyval:
                                                                                                     Some(keyval),
                                                                                                 table:
                                                                                                     None,
                                                                                                 comment:
                                                                                                     comment,}
                                                                                  })())
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn ws_comment(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Expression, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws = o;
                             match {
                                       let (tmp, res) = self.comment(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                    +
                                                                                    ((i).input_len()
                                                                                         -
                                                                                         i.input_len())
                                                                                    +
                                                                                    i)),
                                 ::nom::IResult::Done(i, o) => {
                                     let comment = o;
                                     ::nom::IResult::Done(i,
                                                          (|| {
                                                              Expression{ws:
                                                                             WSSep{ws1:
                                                                                       ws,
                                                                                   ws2:
                                                                                       "",},
                                                                         keyval:
                                                                             None,
                                                                         table:
                                                                             None,
                                                                         comment:
                                                                             Some(comment),}
                                                          })())
                                 }
                             }
                         }
                     }
                 }
             })
        }
    }
}
mod util {
    #[prelude_import]
    use std::prelude::v1::*;
    use ast::structs::Comment;
    use parser::Parser;
    fn not_eol(chr: char) -> bool {
        (chr as u32) == 9 || ((chr as u32) >= 32 && (chr as u32) <= 69631)
    }
    impl <'a> Parser<'a> {
        pub fn newline(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 match {
                           {
                               let res =
                                   {
                                       match {
                                                 let res:
                                                         ::nom::IResult<_,
                                                                        _> =
                                                     if "\r\n".len() > i.len()
                                                        {
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size("\r\n".len()))
                                                     } else if (i).starts_with("\r\n")
                                                      {
                                                         ::nom::IResult::Done(&i["\r\n".len()..],
                                                                              &i[0.."\r\n".len()])
                                                     } else {
                                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                    i))
                                                     };
                                                 res
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Error(e) =>
                                           ::nom::IResult::Error(e),
                                           ::nom::IResult::Incomplete(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                          i))
                                           }
                                       }
                                   };
                               match res {
                                   ::nom::IResult::Done(_, _) => res,
                                   ::nom::IResult::Incomplete(_) => res,
                                   _ => {
                                       match {
                                                 match {
                                                           let res:
                                                                   ::nom::IResult<_,
                                                                                  _> =
                                                               if "\n".len() >
                                                                      i.len()
                                                                  {
                                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("\n".len()))
                                                               } else if (i).starts_with("\n")
                                                                {
                                                                   ::nom::IResult::Done(&i["\n".len()..],
                                                                                        &i[0.."\n".len()])
                                                               } else {
                                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                              i))
                                                               };
                                                           res
                                                       } {
                                                     ::nom::IResult::Done(i,
                                                                          o)
                                                     =>
                                                     ::nom::IResult::Done(i,
                                                                          o),
                                                     ::nom::IResult::Error(e)
                                                     =>
                                                     ::nom::IResult::Error(e),
                                                     ::nom::IResult::Incomplete(_)
                                                     => {
                                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                    i))
                                                     }
                                                 }
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Incomplete(x) =>
                                           ::nom::IResult::Incomplete(x),
                                           ::nom::IResult::Error(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                          i))
                                           }
                                       }
                                   }
                               }
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let string = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  self.line_count.set(self.line_count.get()
                                                                          +
                                                                          1);
                                                  string })())
                     }
                 }
             })
        }
        pub fn ws(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re = ::regex::Regex::new("^( |\t)*").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        pub fn comment(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Comment, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let res: ::nom::IResult<_, _> =
                                   if "#".len() > i.len() {
                                       ::nom::IResult::Incomplete(::nom::Needed::Size("#".len()))
                                   } else if (i).starts_with("#") {
                                       ::nom::IResult::Done(&i["#".len()..],
                                                            &i[0.."#".len()])
                                   } else {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                  i))
                                   };
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, _) => {
                             match {
                                       let mut offset = i.len();
                                       for (o, c) in i.char_indices() {
                                           if !not_eol(c) {
                                               offset = o;
                                               break ;
                                           }
                                       }
                                       if offset < i.len() {
                                           ::nom::IResult::Done(&i[offset..],
                                                                &i[..offset])
                                       } else { ::nom::IResult::Done("", i) }
                                   } {
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                 =>
                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                    +
                                                                                    ((i).input_len()
                                                                                         -
                                                                                         i.input_len())
                                                                                    +
                                                                                    i)),
                                 ::nom::IResult::Done(i, o) => {
                                     let comment_txt = o;
                                     ::nom::IResult::Done(i,
                                                          (|| {
                                                              Comment{text:
                                                                          comment_txt,}
                                                          })())
                                 }
                             }
                         }
                     }
                 }
             })
        }
    }
}
mod objects {
    #[prelude_import]
    use std::prelude::v1::*;
    use std::slice::SliceConcatExt;
    use ast::structs::{TableType, WSKeySep, Table, CommentNewLines,
                       CommentOrNewLines, ArrayValue, Array, InlineTable,
                       WSSep, TableKeyVal};
    use parser::{Parser, count_lines};
    impl <'a> Parser<'a> {
        pub fn table(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TableType, u32>) {
            (self,
             {
                 {
                     let res =
                         {
                             match {
                                       let (tmp, res) = self.array_table(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                i))
                                 }
                             }
                         };
                     match res {
                         ::nom::IResult::Done(_, _) => res,
                         ::nom::IResult::Incomplete(_) => res,
                         _ => {
                             match {
                                       match {
                                                 let (tmp, res) =
                                                     self.std_table(i);
                                                 self = tmp;
                                                 res
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Error(e) =>
                                           ::nom::IResult::Error(e),
                                           ::nom::IResult::Incomplete(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                          i))
                                           }
                                       }
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Incomplete(x) =>
                                 ::nom::IResult::Incomplete(x),
                                 ::nom::IResult::Error(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                i))
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn table_subkeys(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Vec<WSKeySep>, u32>) {
            (self,
             {
                 use nom::InputLength;
                 if (i).input_len() == 0 {
                     ::nom::IResult::Done(i, ::std::vec::Vec::new())
                 } else {
                     match {
                               let (tmp, res) = self.table_subkey(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(_) => {
                             ::nom::IResult::Done(i, ::std::vec::Vec::new())
                         }
                         ::nom::IResult::Incomplete(i) =>
                         ::nom::IResult::Incomplete(i),
                         ::nom::IResult::Done(i1, o1) => {
                             if i1.input_len() == 0 {
                                 ::nom::IResult::Done(i1,
                                                      <[_]>::into_vec(::std::boxed::Box::new([o1])))
                             } else {
                                 let mut res =
                                     ::std::vec::Vec::with_capacity(4);
                                 res.push(o1);
                                 let mut input = i1;
                                 let mut incomplete:
                                         ::std::option::Option<::nom::Needed> =
                                     ::std::option::Option::None;
                                 loop  {
                                     match {
                                               let (tmp, res) =
                                                   self.table_subkey(input);
                                               self = tmp;
                                               res
                                           } {
                                         ::nom::IResult::Done(i, o) => {
                                             if i.input_len() ==
                                                    input.input_len() {
                                                 break ;
                                             }
                                             res.push(o);
                                             input = i;
                                         }
                                         ::nom::IResult::Error(_) => {
                                             break ;
                                         }
                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                         => {
                                             incomplete =
                                                 ::std::option::Option::Some(::nom::Needed::Unknown);
                                             break ;
                                         }
                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                         => {
                                             incomplete =
                                                 ::std::option::Option::Some(::nom::Needed::Size(i
                                                                                                     +
                                                                                                     (i).input_len()
                                                                                                     -
                                                                                                     input.input_len()));
                                             break ;
                                         }
                                     }
                                     if input.input_len() == 0 { break ; }
                                 }
                                 match incomplete {
                                     ::std::option::Option::Some(i) =>
                                     ::nom::IResult::Incomplete(i),
                                     ::std::option::Option::None =>
                                     ::nom::IResult::Done(input, res),
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn table_subkey(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, WSKeySep, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if ".".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(".".len()))
                                               } else if (i).starts_with(".")
                                                {
                                                   ::nom::IResult::Done(&i[".".len()..],
                                                                        &i[0..".".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.ws(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let ws2 = o;
                                                     match {
                                                               let (tmp,
                                                                    res) =
                                                                   self.key(i);
                                                               self = tmp;
                                                               res
                                                           } {
                                                         ::nom::IResult::Error(e)
                                                         =>
                                                         ::nom::IResult::Error(e),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            i)),
                                                         ::nom::IResult::Done(i,
                                                                              o)
                                                         => {
                                                             let key = o;
                                                             ::nom::IResult::Done(i,
                                                                                  (||
                                                                                       {
                                                                                      WSKeySep{ws:
                                                                                                   WSSep{ws1:
                                                                                                             ws1,
                                                                                                         ws2:
                                                                                                             ws2,},
                                                                                               key:
                                                                                                   key,}
                                                                                  })())
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn std_table(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TableType, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let res: ::nom::IResult<_, _> =
                                   if "[".len() > i.len() {
                                       ::nom::IResult::Incomplete(::nom::Needed::Size("[".len()))
                                   } else if (i).starts_with("[") {
                                       ::nom::IResult::Done(&i["[".len()..],
                                                            &i[0.."[".len()])
                                   } else {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                  i))
                                   };
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, _) => {
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.ws(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let ws1 = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.key(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let key = o;
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let (tmp,
                                                                        res) =
                                                                       self.table_subkeys(i);
                                                                   self = tmp;
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             => {
                                                                 let subkeys =
                                                                     o;
                                                                 {
                                                                     use nom::InputLength;
                                                                     match {
                                                                               let (tmp,
                                                                                    res) =
                                                                                   self.ws(i);
                                                                               self
                                                                                   =
                                                                                   tmp;
                                                                               res
                                                                           } {
                                                                         ::nom::IResult::Error(e)
                                                                         =>
                                                                         ::nom::IResult::Error(e),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            i)),
                                                                         ::nom::IResult::Done(i,
                                                                                              o)
                                                                         => {
                                                                             let ws2 =
                                                                                 o;
                                                                             match {
                                                                                       let res:
                                                                                               ::nom::IResult<_,
                                                                                                              _> =
                                                                                           if "]".len()
                                                                                                  >
                                                                                                  i.len()
                                                                                              {
                                                                                               ::nom::IResult::Incomplete(::nom::Needed::Size("]".len()))
                                                                                           } else if (i).starts_with("]")
                                                                                            {
                                                                                               ::nom::IResult::Done(&i["]".len()..],
                                                                                                                    &i[0.."]".len()])
                                                                                           } else {
                                                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                                          i))
                                                                                           };
                                                                                       res
                                                                                   }
                                                                                 {
                                                                                 ::nom::IResult::Error(e)
                                                                                 =>
                                                                                 ::nom::IResult::Error(e),
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                                 =>
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                                 =>
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    i)),
                                                                                 ::nom::IResult::Done(i,
                                                                                                      _)
                                                                                 =>
                                                                                 {
                                                                                     ::nom::IResult::Done(i,
                                                                                                          (||
                                                                                                               {
                                                                                                              TableType::Standard(Table{ws:
                                                                                                                                            WSSep{ws1:
                                                                                                                                                      ws1,
                                                                                                                                                  ws2:
                                                                                                                                                      ws2,},
                                                                                                                                        key:
                                                                                                                                            key,
                                                                                                                                        subkeys:
                                                                                                                                            subkeys,})
                                                                                                          })())
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn array_table(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TableType, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let res: ::nom::IResult<_, _> =
                                   if "[[".len() > i.len() {
                                       ::nom::IResult::Incomplete(::nom::Needed::Size("[[".len()))
                                   } else if (i).starts_with("[[") {
                                       ::nom::IResult::Done(&i["[[".len()..],
                                                            &i[0.."[[".len()])
                                   } else {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                  i))
                                   };
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, _) => {
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.ws(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let ws1 = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.key(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let key = o;
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let (tmp,
                                                                        res) =
                                                                       self.table_subkeys(i);
                                                                   self = tmp;
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             => {
                                                                 let subkeys =
                                                                     o;
                                                                 {
                                                                     use nom::InputLength;
                                                                     match {
                                                                               let (tmp,
                                                                                    res) =
                                                                                   self.ws(i);
                                                                               self
                                                                                   =
                                                                                   tmp;
                                                                               res
                                                                           } {
                                                                         ::nom::IResult::Error(e)
                                                                         =>
                                                                         ::nom::IResult::Error(e),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            i)),
                                                                         ::nom::IResult::Done(i,
                                                                                              o)
                                                                         => {
                                                                             let ws2 =
                                                                                 o;
                                                                             match {
                                                                                       let res:
                                                                                               ::nom::IResult<_,
                                                                                                              _> =
                                                                                           if "]]".len()
                                                                                                  >
                                                                                                  i.len()
                                                                                              {
                                                                                               ::nom::IResult::Incomplete(::nom::Needed::Size("]]".len()))
                                                                                           } else if (i).starts_with("]]")
                                                                                            {
                                                                                               ::nom::IResult::Done(&i["]]".len()..],
                                                                                                                    &i[0.."]]".len()])
                                                                                           } else {
                                                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                                          i))
                                                                                           };
                                                                                       res
                                                                                   }
                                                                                 {
                                                                                 ::nom::IResult::Error(e)
                                                                                 =>
                                                                                 ::nom::IResult::Error(e),
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                                 =>
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                                 =>
                                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    ((i).input_len()
                                                                                                                                         -
                                                                                                                                         i.input_len())
                                                                                                                                    +
                                                                                                                                    i)),
                                                                                 ::nom::IResult::Done(i,
                                                                                                      _)
                                                                                 =>
                                                                                 {
                                                                                     ::nom::IResult::Done(i,
                                                                                                          (||
                                                                                                               {
                                                                                                              *self.last_table.borrow_mut()
                                                                                                                  =
                                                                                                                  key;
                                                                                                              TableType::Array(Table{ws:
                                                                                                                                         WSSep{ws1:
                                                                                                                                                   ws1,
                                                                                                                                               ws2:
                                                                                                                                                   ws2,},
                                                                                                                                     key:
                                                                                                                                         key,
                                                                                                                                     subkeys:
                                                                                                                                         subkeys,})
                                                                                                          })())
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn array_sep(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, WSSep, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if ",".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(",".len()))
                                               } else if (i).starts_with(",")
                                                {
                                                   ::nom::IResult::Done(&i[",".len()..],
                                                                        &i[0..",".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         match {
                                                   let (tmp, res) =
                                                       self.ws(i);
                                                   self = tmp;
                                                   res
                                               } {
                                             ::nom::IResult::Error(e) =>
                                             ::nom::IResult::Error(e),
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                i)),
                                             ::nom::IResult::Done(i, o) => {
                                                 let ws2 = o;
                                                 ::nom::IResult::Done(i,
                                                                      (|| {
                                                                          WSSep{ws1:
                                                                                    ws1,
                                                                                ws2:
                                                                                    ws2,}
                                                                      })())
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn ws_newline(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 match {
                           let re =
                               ::regex::Regex::new("^( |\t|\n|(\r\n))*").unwrap();
                           if let Some((begin, end)) = re.find(i) {
                               ::nom::IResult::Done(&i[end..], &i[begin..end])
                           } else {
                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let string = o;
                         ::nom::IResult::Done(i, (|| { string })())
                     }
                 }
             })
        }
        fn ws_newlines(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 match {
                           let re =
                               ::regex::Regex::new("^(\n|(\r\n))( |\t|\n|(\r\n))*").unwrap();
                           if let Some((begin, end)) = re.find(i) {
                               ::nom::IResult::Done(&i[end..], &i[begin..end])
                           } else {
                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let string = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  self.line_count.set(self.line_count.get()
                                                                          +
                                                                          count_lines(string));
                                                  string })())
                     }
                 }
             })
        }
        fn comment_nl(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, CommentNewLines, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let (tmp, res) = self.ws_newline(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let prewsnl = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.comment(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let comment = o;
                                         match {
                                                   let (tmp, res) =
                                                       self.ws_newlines(i);
                                                   self = tmp;
                                                   res
                                               } {
                                             ::nom::IResult::Error(e) =>
                                             ::nom::IResult::Error(e),
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                i)),
                                             ::nom::IResult::Done(i, o) => {
                                                 let newlines = o;
                                                 ::nom::IResult::Done(i,
                                                                      (|| {
                                                                          CommentNewLines{pre_ws_nl:
                                                                                              prewsnl,
                                                                                          comment:
                                                                                              comment,
                                                                                          newlines:
                                                                                              newlines,}
                                                                      })())
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn comment_or_nl(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, CommentOrNewLines, u32>) {
            (self,
             {
                 {
                     match {
                               match {
                                         let (tmp, res) = self.comment_nl(i);
                                         self = tmp;
                                         res
                                     } {
                                   ::nom::IResult::Done(i, o) =>
                                   ::nom::IResult::Done(i, o),
                                   ::nom::IResult::Error(e) =>
                                   ::nom::IResult::Error(e),
                                   ::nom::IResult::Incomplete(_) => {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                  i))
                                   }
                               }
                           } {
                         ::nom::IResult::Done(i, o) =>
                         ::nom::IResult::Done(i,
                                              (|com|
                                                   CommentOrNewLines::Comment(com))(o)),
                         ::nom::IResult::Incomplete(x) =>
                         ::nom::IResult::Incomplete(x),
                         ::nom::IResult::Error(_) => {
                             {
                                 match {
                                           match {
                                                     let (tmp, res) =
                                                         self.ws_newlines(i);
                                                     self = tmp;
                                                     res
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(_)
                                               => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                              i))
                                               }
                                           }
                                       } {
                                     ::nom::IResult::Done(i, o) =>
                                     ::nom::IResult::Done(i,
                                                          (|nl|
                                                               CommentOrNewLines::NewLines(nl))(o)),
                                     ::nom::IResult::Incomplete(x) =>
                                     ::nom::IResult::Incomplete(x),
                                     ::nom::IResult::Error(_) => {
                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                    i))
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn array_value(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, ArrayValue, u32>) {
            (self,
             {
                 {
                     let res =
                         {
                             match {
                                       {
                                           use nom::InputLength;
                                           match {
                                                     let (tmp, res) =
                                                         self.val(i);
                                                     self = tmp;
                                                     res
                                                 } {
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                               =>
                                               ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                               ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                               =>
                                               ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                  +
                                                                                                  i)),
                                               ::nom::IResult::Done(i, o) => {
                                                   let val = o;
                                                   {
                                                       use nom::InputLength;
                                                       match {
                                                                 let (tmp,
                                                                      res) =
                                                                     self.array_sep(i);
                                                                 self = tmp;
                                                                 res
                                                             } {
                                                           ::nom::IResult::Error(e)
                                                           =>
                                                           ::nom::IResult::Error(e),
                                                           ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                           =>
                                                           ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                           ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                           =>
                                                           ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                              +
                                                                                                              ((i).input_len()
                                                                                                                   -
                                                                                                                   i.input_len())
                                                                                                              +
                                                                                                              i)),
                                                           ::nom::IResult::Done(i,
                                                                                o)
                                                           => {
                                                               let array_sep =
                                                                   o;
                                                               match {
                                                                         match {
                                                                                   let (tmp,
                                                                                        res) =
                                                                                       self.comment_or_nl(i);
                                                                                   self
                                                                                       =
                                                                                       tmp;
                                                                                   res
                                                                               }
                                                                             {
                                                                             ::nom::IResult::Done(i,
                                                                                                  o)
                                                                             =>
                                                                             ::nom::IResult::Done(i,
                                                                                                  o),
                                                                             ::nom::IResult::Error(e)
                                                                             =>
                                                                             ::nom::IResult::Error(e),
                                                                             ::nom::IResult::Incomplete(_)
                                                                             =>
                                                                             {
                                                                                 ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                            i))
                                                                             }
                                                                         }
                                                                     } {
                                                                   ::nom::IResult::Error(e)
                                                                   =>
                                                                   ::nom::IResult::Error(e),
                                                                   ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                   =>
                                                                   ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                   =>
                                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                      +
                                                                                                                      ((i).input_len()
                                                                                                                           -
                                                                                                                           i.input_len())
                                                                                                                      +
                                                                                                                      ((i).input_len()
                                                                                                                           -
                                                                                                                           i.input_len())
                                                                                                                      +
                                                                                                                      i)),
                                                                   ::nom::IResult::Done(i,
                                                                                        o)
                                                                   => {
                                                                       let comment_nl =
                                                                           o;
                                                                       ::nom::IResult::Done(i,
                                                                                            (||
                                                                                                 {
                                                                                                ArrayValue{val:
                                                                                                               val,
                                                                                                           array_sep:
                                                                                                               Some(array_sep),
                                                                                                           comment_nl:
                                                                                                               Some(comment_nl),}
                                                                                            })())
                                                                   }
                                                               }
                                                           }
                                                       }
                                                   }
                                               }
                                           }
                                       }
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                i))
                                 }
                             }
                         };
                     match res {
                         ::nom::IResult::Done(_, _) => res,
                         ::nom::IResult::Incomplete(_) => res,
                         _ => {
                             match {
                                       match {
                                                 {
                                                     use nom::InputLength;
                                                     match {
                                                               let (tmp,
                                                                    res) =
                                                                   self.val(i);
                                                               self = tmp;
                                                               res
                                                           } {
                                                         ::nom::IResult::Error(e)
                                                         =>
                                                         ::nom::IResult::Error(e),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                            +
                                                                                                            i)),
                                                         ::nom::IResult::Done(i,
                                                                              o)
                                                         => {
                                                             let val = o;
                                                             match {
                                                                       match {
                                                                                 let (tmp,
                                                                                      res) =
                                                                                     self.comment_or_nl(i);
                                                                                 self
                                                                                     =
                                                                                     tmp;
                                                                                 res
                                                                             }
                                                                           {
                                                                           ::nom::IResult::Done(i,
                                                                                                o)
                                                                           =>
                                                                           ::nom::IResult::Done(i,
                                                                                                o),
                                                                           ::nom::IResult::Error(e)
                                                                           =>
                                                                           ::nom::IResult::Error(e),
                                                                           ::nom::IResult::Incomplete(_)
                                                                           =>
                                                                           {
                                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                          i))
                                                                           }
                                                                       }
                                                                   } {
                                                                 ::nom::IResult::Error(e)
                                                                 =>
                                                                 ::nom::IResult::Error(e),
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                 =>
                                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                    +
                                                                                                                    ((i).input_len()
                                                                                                                         -
                                                                                                                         i.input_len())
                                                                                                                    +
                                                                                                                    i)),
                                                                 ::nom::IResult::Done(i,
                                                                                      o)
                                                                 => {
                                                                     let comment_nl =
                                                                         o;
                                                                     ::nom::IResult::Done(i,
                                                                                          (||
                                                                                               {
                                                                                              ArrayValue{val:
                                                                                                             val,
                                                                                                         array_sep:
                                                                                                             None,
                                                                                                         comment_nl:
                                                                                                             Some(comment_nl),}
                                                                                          })())
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Error(e) =>
                                           ::nom::IResult::Error(e),
                                           ::nom::IResult::Incomplete(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                          i))
                                           }
                                       }
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Incomplete(x) =>
                                 ::nom::IResult::Incomplete(x),
                                 ::nom::IResult::Error(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                i))
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn array_values(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Vec<ArrayValue>, u32>) {
            (self,
             {
                 match {
                           use nom::InputLength;
                           if (i).input_len() == 0 {
                               ::nom::IResult::Done(i, ::std::vec::Vec::new())
                           } else {
                               match {
                                         let (tmp, res) = self.array_value(i);
                                         self = tmp;
                                         res
                                     } {
                                   ::nom::IResult::Error(_) => {
                                       ::nom::IResult::Done(i,
                                                            ::std::vec::Vec::new())
                                   }
                                   ::nom::IResult::Incomplete(i) =>
                                   ::nom::IResult::Incomplete(i),
                                   ::nom::IResult::Done(i1, o1) => {
                                       if i1.input_len() == 0 {
                                           ::nom::IResult::Done(i1,
                                                                <[_]>::into_vec(::std::boxed::Box::new([o1])))
                                       } else {
                                           let mut res =
                                               ::std::vec::Vec::with_capacity(4);
                                           res.push(o1);
                                           let mut input = i1;
                                           let mut incomplete:
                                                   ::std::option::Option<::nom::Needed> =
                                               ::std::option::Option::None;
                                           loop  {
                                               match {
                                                         let (tmp, res) =
                                                             self.array_value(input);
                                                         self = tmp;
                                                         res
                                                     } {
                                                   ::nom::IResult::Done(i, o)
                                                   => {
                                                       if i.input_len() ==
                                                              input.input_len()
                                                          {
                                                           break ;
                                                       }
                                                       res.push(o);
                                                       input = i;
                                                   }
                                                   ::nom::IResult::Error(_) =>
                                                   {
                                                       break ;
                                                   }
                                                   ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                   => {
                                                       incomplete =
                                                           ::std::option::Option::Some(::nom::Needed::Unknown);
                                                       break ;
                                                   }
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                   => {
                                                       incomplete =
                                                           ::std::option::Option::Some(::nom::Needed::Size(i
                                                                                                               +
                                                                                                               (i).input_len()
                                                                                                               -
                                                                                                               input.input_len()));
                                                       break ;
                                                   }
                                               }
                                               if input.input_len() == 0 {
                                                   break ;
                                               }
                                           }
                                           match incomplete {
                                               ::std::option::Option::Some(i)
                                               =>
                                               ::nom::IResult::Incomplete(i),
                                               ::std::option::Option::None =>
                                               ::nom::IResult::Done(input,
                                                                    res),
                                           }
                                       }
                                   }
                               }
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let vals = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  let mut tmp =
                                                      <[_]>::into_vec(::std::boxed::Box::new([]));
                                                  tmp.extend(vals); tmp })())
                     }
                 }
             })
        }
        pub fn array(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Array, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let res: ::nom::IResult<_, _> =
                                   if "[".len() > i.len() {
                                       ::nom::IResult::Incomplete(::nom::Needed::Size("[".len()))
                                   } else if (i).starts_with("[") {
                                       ::nom::IResult::Done(&i["[".len()..],
                                                            &i[0.."[".len()])
                                   } else {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                  i))
                                   };
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, _) => {
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) =
                                               self.ws_newline(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let ws1 = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.array_values(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let array_vals = o;
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let (tmp,
                                                                        res) =
                                                                       self.ws(i);
                                                                   self = tmp;
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             => {
                                                                 let ws2 = o;
                                                                 match {
                                                                           let res:
                                                                                   ::nom::IResult<_,
                                                                                                  _> =
                                                                               if "]".len()
                                                                                      >
                                                                                      i.len()
                                                                                  {
                                                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("]".len()))
                                                                               } else if (i).starts_with("]")
                                                                                {
                                                                                   ::nom::IResult::Done(&i["]".len()..],
                                                                                                        &i[0.."]".len()])
                                                                               } else {
                                                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                              i))
                                                                               };
                                                                           res
                                                                       } {
                                                                     ::nom::IResult::Error(e)
                                                                     =>
                                                                     ::nom::IResult::Error(e),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        i)),
                                                                     ::nom::IResult::Done(i,
                                                                                          _)
                                                                     => {
                                                                         ::nom::IResult::Done(i,
                                                                                              (||
                                                                                                   {
                                                                                                  Array{values:
                                                                                                            array_vals,
                                                                                                        ws:
                                                                                                            WSSep{ws1:
                                                                                                                      ws1,
                                                                                                                  ws2:
                                                                                                                      ws2,},}
                                                                                              })())
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn table_keyval(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TableKeyVal, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.keyval(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let keyval = o;
                                         match {
                                                   let (tmp, res) =
                                                       self.ws(i);
                                                   self = tmp;
                                                   res
                                               } {
                                             ::nom::IResult::Error(e) =>
                                             ::nom::IResult::Error(e),
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                i)),
                                             ::nom::IResult::Done(i, o) => {
                                                 let ws2 = o;
                                                 ::nom::IResult::Done(i,
                                                                      (|| {
                                                                          TableKeyVal{keyval:
                                                                                          keyval,
                                                                                      kv_sep:
                                                                                          WSSep{ws1:
                                                                                                    ws1,
                                                                                                ws2:
                                                                                                    ws2,},}
                                                                      })())
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn inline_table_keyvals_non_empty(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Vec<TableKeyVal>, u32>) {
            (self,
             {
                 let mut res = ::std::vec::Vec::new();
                 let mut input = i;
                 match {
                           let (tmp, res) = self.table_keyval(input);
                           self = tmp;
                           res
                       } {
                     ::nom::IResult::Error(_) =>
                     ::nom::IResult::Done(input, ::std::vec::Vec::new()),
                     ::nom::IResult::Incomplete(i) =>
                     ::nom::IResult::Incomplete(i),
                     ::nom::IResult::Done(i, o) => {
                         if i.len() == input.len() {
                             ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::SeparatedList,
                                                                        input))
                         } else {
                             res.push(o);
                             input = i;
                             loop  {
                                 if let ::nom::IResult::Done(i2, _) =
                                        {
                                            let res: ::nom::IResult<_, _> =
                                                if ",".len() > input.len() {
                                                    ::nom::IResult::Incomplete(::nom::Needed::Size(",".len()))
                                                } else if (input).starts_with(",")
                                                 {
                                                    ::nom::IResult::Done(&input[",".len()..],
                                                                         &input[0..",".len()])
                                                } else {
                                                    ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                               input))
                                                };
                                            res
                                        } {
                                     if i2.len() == input.len() { break ; }
                                     input = i2;
                                     if let ::nom::IResult::Done(i3, o3) =
                                            {
                                                let (tmp, res) =
                                                    self.table_keyval(input);
                                                self = tmp;
                                                res
                                            } {
                                         if i3.len() == input.len() {
                                             break ;
                                         }
                                         res.push(o3);
                                         input = i3;
                                     } else { break ; }
                                 } else { break ; }
                             }
                             ::nom::IResult::Done(input, res)
                         }
                     }
                 }
             })
        }
        pub fn inline_table(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, InlineTable, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let res: ::nom::IResult<_, _> =
                                   if "{".len() > i.len() {
                                       ::nom::IResult::Incomplete(::nom::Needed::Size("{".len()))
                                   } else if (i).starts_with("{") {
                                       ::nom::IResult::Done(&i["{".len()..],
                                                            &i[0.."{".len()])
                                   } else {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                  i))
                                   };
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, _) => {
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) = self.ws(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let ws1 = o;
                                         {
                                             {
                                                 use nom::InputLength;
                                                 let res =
                                                     {
                                                         match {
                                                                   let (tmp,
                                                                        res) =
                                                                       self.inline_table_keyvals_non_empty(i);
                                                                   self = tmp;
                                                                   res
                                                               } {
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             =>
                                                             ::nom::IResult::Done(i,
                                                                                  o),
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(_)
                                                             => {
                                                                 ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                            i))
                                                             }
                                                         }
                                                     };
                                                 if let ::nom::IResult::Incomplete(inc)
                                                        = res {
                                                     match inc {
                                                         ::nom::Needed::Unknown
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                         ::nom::Needed::Size(i)
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            i)),
                                                     }
                                                 } else {
                                                     let (keyvals, input) =
                                                         if let ::nom::IResult::Done(i,
                                                                                     o)
                                                                = res {
                                                             (::std::option::Option::Some(o),
                                                              i)
                                                         } else {
                                                             (::std::option::Option::None,
                                                              i)
                                                         };
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let (tmp,
                                                                        res) =
                                                                       self.ws(input);
                                                                   self = tmp;
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                (i).input_len()
                                                                                                                -
                                                                                                                input.input_len()
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             => {
                                                                 let ws2 = o;
                                                                 match {
                                                                           let res:
                                                                                   ::nom::IResult<_,
                                                                                                  _> =
                                                                               if "}".len()
                                                                                      >
                                                                                      i.len()
                                                                                  {
                                                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("}".len()))
                                                                               } else if (i).starts_with("}")
                                                                                {
                                                                                   ::nom::IResult::Done(&i["}".len()..],
                                                                                                        &i[0.."}".len()])
                                                                               } else {
                                                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                              i))
                                                                               };
                                                                           res
                                                                       } {
                                                                     ::nom::IResult::Error(e)
                                                                     =>
                                                                     ::nom::IResult::Error(e),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        (i).input_len()
                                                                                                                        -
                                                                                                                        input.input_len()
                                                                                                                        +
                                                                                                                        ((input).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        i)),
                                                                     ::nom::IResult::Done(i,
                                                                                          _)
                                                                     => {
                                                                         ::nom::IResult::Done(i,
                                                                                              (||
                                                                                                   {
                                                                                                  InlineTable{keyvals:
                                                                                                                  keyvals,
                                                                                                              ws:
                                                                                                                  WSSep{ws1:
                                                                                                                            ws1,
                                                                                                                        ws2:
                                                                                                                            ws2,},}
                                                                                              })())
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
    }
}
mod primitives {
    #[prelude_import]
    use std::prelude::v1::*;
    use std::cell::RefCell;
    use ast::structs::{Time, FullDate, KeyVal, WSSep, Value, StrType,
                       ErrorCode};
    use types::{DateTime, TimeOffset, TimeOffsetAmount};
    use parser::{Parser, count_lines};
    use nom;
    use nom::{IResult, InputLength};
    fn is_keychar(chr: char) -> bool {
        let uchr = chr as u32;
        uchr >= 65 && uchr <= 90 || uchr >= 97 && uchr <= 122 ||
            uchr >= 48 && uchr <= 57 || uchr == 45 || uchr == 95
    }
    impl <'a> Parser<'a> {
        fn integer(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re =
                     ::regex::Regex::new("^((\\+|-)?(([1-9](\\d|(_\\d))+)|\\d))").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        fn float(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re =
                     ::regex::Regex::new("^(\\+|-)?([1-9](\\d|(_\\d))+|\\d)((\\.\\d(\\d|(_\\d))*)((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d))|(\\.\\d(\\d|(_\\d))*)|((e|E)(\\+|-)?([1-9](\\d|(_\\d))+|\\d)))").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        fn raw_basic_string(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re =
                     ::regex::Regex::new("^\"( |!|[#-\\[]|[\\]-\u{10ffff}]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))*?\"").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        fn raw_ml_basic_string(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 match {
                           let re =
                               ::regex::Regex::new("^\"\"\"([ -\\[]|[\\]-\u{10ffff}]|(\\\\\")|(\\\\)|(\\\\/)|(\\b)|(\\f)|(\\n)|(\\r)|(\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8})|\n|(\r\n)|(\\\\(\n|(\r\n))))*?\"\"\"").unwrap();
                           if let Some((begin, end)) = re.find(i) {
                               ::nom::IResult::Done(&i[end..], &i[begin..end])
                           } else {
                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let string = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  self.line_count.set(self.line_count.get()
                                                                          +
                                                                          count_lines(string));
                                                  string })())
                     }
                 }
             })
        }
        fn raw_literal_string(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re =
                     ::regex::Regex::new("^\'(\t|[ -&]|[\\(-\u{10ffff}])*?\'").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        fn raw_ml_literal_string(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 match {
                           let re =
                               ::regex::Regex::new("^\'\'\'(\t|[ -\u{10ffff}]|\n|(\r\n))*?\'\'\'").unwrap();
                           if let Some((begin, end)) = re.find(i) {
                               ::nom::IResult::Done(&i[end..], &i[begin..end])
                           } else {
                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                           }
                       } {
                     ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                     ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize +
                                                                        i)),
                     ::nom::IResult::Done(i, o) => {
                         let string = o;
                         ::nom::IResult::Done(i,
                                              (|| {
                                                  self.line_count.set(self.line_count.get()
                                                                          +
                                                                          count_lines(string));
                                                  string })())
                     }
                 }
             })
        }
        fn ml_basic_string(self: Parser<'a>, input: &'a str)
         -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
            let (tmp, raw) = self.raw_ml_basic_string(input);
            self = tmp;
            let r =
                match raw {
                    IResult::Done(i, o) =>
                    IResult::Done(i,
                                  &o["\"\"\"".input_len()..o.input_len() -
                                                               "\"\"\"".input_len()]),
                    IResult::Error(_) =>
                    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString
                                                                             as
                                                                             u32))),
                    IResult::Incomplete(i) => IResult::Incomplete(i),
                };
            (self, r)
        }
        fn basic_string(self: Parser<'a>, input: &'a str)
         -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
            let (tmp, raw) = self.raw_basic_string(input);
            self = tmp;
            let r =
                match raw {
                    IResult::Done(i, o) =>
                    IResult::Done(i,
                                  &o["\"".input_len()..o.input_len() -
                                                           "\"".input_len()]),
                    IResult::Error(_) =>
                    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString
                                                                             as
                                                                             u32))),
                    IResult::Incomplete(i) => IResult::Incomplete(i),
                };
            (self, r)
        }
        fn ml_literal_string(self: Parser<'a>, input: &'a str)
         -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
            let (tmp, raw) = self.raw_ml_literal_string(input);
            self = tmp;
            let r =
                match raw {
                    IResult::Done(i, o) =>
                    IResult::Done(i,
                                  &o["\'\'\'".input_len()..o.input_len() -
                                                               "\'\'\'".input_len()]),
                    IResult::Error(_) =>
                    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString
                                                                             as
                                                                             u32))),
                    IResult::Incomplete(i) => IResult::Incomplete(i),
                };
            (self, r)
        }
        fn literal_string(self: Parser<'a>, input: &'a str)
         -> (Parser<'a>, nom::IResult<&'a str, &'a str>) {
            let (tmp, raw) = self.raw_literal_string(input);
            self = tmp;
            let r =
                match raw {
                    IResult::Done(i, o) =>
                    IResult::Done(i,
                                  &o["\'".input_len()..o.input_len() -
                                                           "\'".input_len()]),
                    IResult::Error(_) =>
                    IResult::Error(nom::Err::Code(nom::ErrorKind::Custom(ErrorCode::MLLiteralString
                                                                             as
                                                                             u32))),
                    IResult::Incomplete(i) => IResult::Incomplete(i),
                };
            (self, r)
        }
        fn string(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Value, u32>) {
            (self,
             {
                 {
                     match {
                               match {
                                         let (tmp, res) =
                                             self.ml_literal_string(i);
                                         self = tmp;
                                         res
                                     } {
                                   ::nom::IResult::Done(i, o) =>
                                   ::nom::IResult::Done(i, o),
                                   ::nom::IResult::Error(e) =>
                                   ::nom::IResult::Error(e),
                                   ::nom::IResult::Incomplete(_) => {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                  i))
                                   }
                               }
                           } {
                         ::nom::IResult::Done(i, o) =>
                         ::nom::IResult::Done(i,
                                              (|ml|
                                                   Value::String(ml,
                                                                 StrType::MLLiteral))(o)),
                         ::nom::IResult::Incomplete(x) =>
                         ::nom::IResult::Incomplete(x),
                         ::nom::IResult::Error(_) => {
                             {
                                 match {
                                           match {
                                                     let (tmp, res) =
                                                         self.ml_basic_string(i);
                                                     self = tmp;
                                                     res
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(_)
                                               => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                              i))
                                               }
                                           }
                                       } {
                                     ::nom::IResult::Done(i, o) =>
                                     ::nom::IResult::Done(i,
                                                          (|mb|
                                                               Value::String(mb,
                                                                             StrType::MLBasic))(o)),
                                     ::nom::IResult::Incomplete(x) =>
                                     ::nom::IResult::Incomplete(x),
                                     ::nom::IResult::Error(_) => {
                                         {
                                             match {
                                                       match {
                                                                 let (tmp,
                                                                      res) =
                                                                     self.basic_string(i);
                                                                 self = tmp;
                                                                 res
                                                             } {
                                                           ::nom::IResult::Done(i,
                                                                                o)
                                                           =>
                                                           ::nom::IResult::Done(i,
                                                                                o),
                                                           ::nom::IResult::Error(e)
                                                           =>
                                                           ::nom::IResult::Error(e),
                                                           ::nom::IResult::Incomplete(_)
                                                           => {
                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                          i))
                                                           }
                                                       }
                                                   } {
                                                 ::nom::IResult::Done(i, o) =>
                                                 ::nom::IResult::Done(i,
                                                                      (|b|
                                                                           Value::String(b,
                                                                                         StrType::Basic))(o)),
                                                 ::nom::IResult::Incomplete(x)
                                                 =>
                                                 ::nom::IResult::Incomplete(x),
                                                 ::nom::IResult::Error(_) => {
                                                     {
                                                         match {
                                                                   match {
                                                                             let (tmp,
                                                                                  res) =
                                                                                 self.literal_string(i);
                                                                             self
                                                                                 =
                                                                                 tmp;
                                                                             res
                                                                         } {
                                                                       ::nom::IResult::Done(i,
                                                                                            o)
                                                                       =>
                                                                       ::nom::IResult::Done(i,
                                                                                            o),
                                                                       ::nom::IResult::Error(e)
                                                                       =>
                                                                       ::nom::IResult::Error(e),
                                                                       ::nom::IResult::Incomplete(_)
                                                                       => {
                                                                           ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                      i))
                                                                       }
                                                                   }
                                                               } {
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             =>
                                                             ::nom::IResult::Done(i,
                                                                                  (|l|
                                                                                       Value::String(l,
                                                                                                     StrType::Literal))(o)),
                                                             ::nom::IResult::Incomplete(x)
                                                             =>
                                                             ::nom::IResult::Incomplete(x),
                                                             ::nom::IResult::Error(_)
                                                             => {
                                                                 ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                                            i))
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn boolean(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 {
                     let res =
                         {
                             match {
                                       let res: ::nom::IResult<_, _> =
                                           if "false".len() > i.len() {
                                               ::nom::IResult::Incomplete(::nom::Needed::Size("false".len()))
                                           } else if (i).starts_with("false")
                                            {
                                               ::nom::IResult::Done(&i["false".len()..],
                                                                    &i[0.."false".len()])
                                           } else {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                          i))
                                           };
                                       res
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                i))
                                 }
                             }
                         };
                     match res {
                         ::nom::IResult::Done(_, _) => res,
                         ::nom::IResult::Incomplete(_) => res,
                         _ => {
                             match {
                                       match {
                                                 let res:
                                                         ::nom::IResult<_,
                                                                        _> =
                                                     if "true".len() > i.len()
                                                        {
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size("true".len()))
                                                     } else if (i).starts_with("true")
                                                      {
                                                         ::nom::IResult::Done(&i["true".len()..],
                                                                              &i[0.."true".len()])
                                                     } else {
                                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                    i))
                                                     };
                                                 res
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Error(e) =>
                                           ::nom::IResult::Error(e),
                                           ::nom::IResult::Incomplete(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                          i))
                                           }
                                       }
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Incomplete(x) =>
                                 ::nom::IResult::Incomplete(x),
                                 ::nom::IResult::Error(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                i))
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn fractional(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Vec<&'a str>, u32>) {
            (self,
             {
                 let re = ::regex::Regex::new("^\\.([0-9]+)").unwrap();
                 if let Some(c) = re.captures(i) {
                     let v: Vec<&str> =
                         c.iter_pos().filter(|el|
                                                 el.is_some()).map(|el|
                                                                       el.unwrap()).map(|(begin,
                                                                                          end)|
                                                                                            &i[begin..end]).collect();
                     let offset =
                         {
                             let end = v.last().unwrap();
                             (end.as_ptr() as usize) + end.len() -
                                 (i.as_ptr() as usize)
                         };
                     ::nom::IResult::Done(&i[offset..], v)
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpCapture))
                 }
             })
        }
        fn time(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Time, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let re =
                                   ::regex::Regex::new("^[0-9]{2}").unwrap();
                               if let Some((begin, end)) = re.find(i) {
                                   ::nom::IResult::Done(&i[end..],
                                                        &i[begin..end])
                               } else {
                                   ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                               }
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let hour = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if ":".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size(":".len()))
                                               } else if (i).starts_with(":")
                                                {
                                                   ::nom::IResult::Done(&i[":".len()..],
                                                                        &i[0..":".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let re =
                                                           ::regex::Regex::new("^[0-9]{2}").unwrap();
                                                       if let Some((begin,
                                                                    end)) =
                                                              re.find(i) {
                                                           ::nom::IResult::Done(&i[end..],
                                                                                &i[begin..end])
                                                       } else {
                                                           ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                                       }
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let minute = o;
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let res:
                                                                           ::nom::IResult<_,
                                                                                          _> =
                                                                       if ":".len()
                                                                              >
                                                                              i.len()
                                                                          {
                                                                           ::nom::IResult::Incomplete(::nom::Needed::Size(":".len()))
                                                                       } else if (i).starts_with(":")
                                                                        {
                                                                           ::nom::IResult::Done(&i[":".len()..],
                                                                                                &i[0..":".len()])
                                                                       } else {
                                                                           ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                      i))
                                                                       };
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  _)
                                                             => {
                                                                 {
                                                                     use nom::InputLength;
                                                                     match {
                                                                               let re =
                                                                                   ::regex::Regex::new("^[0-9]{2}").unwrap();
                                                                               if let Some((begin,
                                                                                            end))
                                                                                      =
                                                                                      re.find(i)
                                                                                      {
                                                                                   ::nom::IResult::Done(&i[end..],
                                                                                                        &i[begin..end])
                                                                               } else {
                                                                                   ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                                                               }
                                                                           } {
                                                                         ::nom::IResult::Error(e)
                                                                         =>
                                                                         ::nom::IResult::Error(e),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                         =>
                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            ((i).input_len()
                                                                                                                                 -
                                                                                                                                 i.input_len())
                                                                                                                            +
                                                                                                                            i)),
                                                                         ::nom::IResult::Done(i,
                                                                                              o)
                                                                         => {
                                                                             let second =
                                                                                 o;
                                                                             {
                                                                                 let res =
                                                                                     {
                                                                                         match {
                                                                                                   let (tmp,
                                                                                                        res) =
                                                                                                       self.fractional(i);
                                                                                                   self
                                                                                                       =
                                                                                                       tmp;
                                                                                                   res
                                                                                               }
                                                                                             {
                                                                                             ::nom::IResult::Done(i,
                                                                                                                  o)
                                                                                             =>
                                                                                             ::nom::IResult::Done(i,
                                                                                                                  o),
                                                                                             ::nom::IResult::Error(e)
                                                                                             =>
                                                                                             ::nom::IResult::Error(e),
                                                                                             ::nom::IResult::Incomplete(_)
                                                                                             =>
                                                                                             {
                                                                                                 ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                                            i))
                                                                                             }
                                                                                         }
                                                                                     };
                                                                                 if let ::nom::IResult::Incomplete(inc)
                                                                                        =
                                                                                        res
                                                                                        {
                                                                                     match inc
                                                                                         {
                                                                                         ::nom::Needed::Unknown
                                                                                         =>
                                                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                                         ::nom::Needed::Size(i)
                                                                                         =>
                                                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                                            +
                                                                                                                                            ((i).input_len()
                                                                                                                                                 -
                                                                                                                                                 i.input_len())
                                                                                                                                            +
                                                                                                                                            ((i).input_len()
                                                                                                                                                 -
                                                                                                                                                 i.input_len())
                                                                                                                                            +
                                                                                                                                            ((i).input_len()
                                                                                                                                                 -
                                                                                                                                                 i.input_len())
                                                                                                                                            +
                                                                                                                                            ((i).input_len()
                                                                                                                                                 -
                                                                                                                                                 i.input_len())
                                                                                                                                            +
                                                                                                                                            ((i).input_len()
                                                                                                                                                 -
                                                                                                                                                 i.input_len())
                                                                                                                                            +
                                                                                                                                            i)),
                                                                                     }
                                                                                 } else {
                                                                                     let (fraction,
                                                                                          input) =
                                                                                         if let ::nom::IResult::Done(i,
                                                                                                                     o)
                                                                                                =
                                                                                                res
                                                                                                {
                                                                                             (::std::option::Option::Some(o),
                                                                                              i)
                                                                                         } else {
                                                                                             (::std::option::Option::None,
                                                                                              i)
                                                                                         };
                                                                                     ::nom::IResult::Done(input,
                                                                                                          (||
                                                                                                               {
                                                                                                              Time{hour:
                                                                                                                       hour,
                                                                                                                   minute:
                                                                                                                       minute,
                                                                                                                   second:
                                                                                                                       second,
                                                                                                                   fraction:
                                                                                                                       match fraction
                                                                                                                           {
                                                                                                                           Some(ref x)
                                                                                                                           =>
                                                                                                                           x[1],
                                                                                                                           None
                                                                                                                           =>
                                                                                                                           "",
                                                                                                                       },}
                                                                                                          })())
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn time_offset_amount(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TimeOffsetAmount, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               {
                                   let res =
                                       {
                                           match {
                                                     let res:
                                                             ::nom::IResult<_,
                                                                            _> =
                                                         if "+".len() >
                                                                i.len() {
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size("+".len()))
                                                         } else if (i).starts_with("+")
                                                          {
                                                             ::nom::IResult::Done(&i["+".len()..],
                                                                                  &i[0.."+".len()])
                                                         } else {
                                                             ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                        i))
                                                         };
                                                     res
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(_)
                                               => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                              i))
                                               }
                                           }
                                       };
                                   match res {
                                       ::nom::IResult::Done(_, _) => res,
                                       ::nom::IResult::Incomplete(_) => res,
                                       _ => {
                                           match {
                                                     match {
                                                               let res:
                                                                       ::nom::IResult<_,
                                                                                      _> =
                                                                   if "-".len()
                                                                          >
                                                                          i.len()
                                                                      {
                                                                       ::nom::IResult::Incomplete(::nom::Needed::Size("-".len()))
                                                                   } else if (i).starts_with("-")
                                                                    {
                                                                       ::nom::IResult::Done(&i["-".len()..],
                                                                                            &i[0.."-".len()])
                                                                   } else {
                                                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                  i))
                                                                   };
                                                               res
                                                           } {
                                                         ::nom::IResult::Done(i,
                                                                              o)
                                                         =>
                                                         ::nom::IResult::Done(i,
                                                                              o),
                                                         ::nom::IResult::Error(e)
                                                         =>
                                                         ::nom::IResult::Error(e),
                                                         ::nom::IResult::Incomplete(_)
                                                         => {
                                                             ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                        i))
                                                         }
                                                     }
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Incomplete(x)
                                               =>
                                               ::nom::IResult::Incomplete(x),
                                               ::nom::IResult::Error(_) => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                              i))
                                               }
                                           }
                                       }
                                   }
                               }
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let pos_neg = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let re =
                                               ::regex::Regex::new("^[0-9]{2}").unwrap();
                                           if let Some((begin, end)) =
                                                  re.find(i) {
                                               ::nom::IResult::Done(&i[end..],
                                                                    &i[begin..end])
                                           } else {
                                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                           }
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let hour = o;
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let res:
                                                               ::nom::IResult<_,
                                                                              _> =
                                                           if ":".len() >
                                                                  i.len() {
                                                               ::nom::IResult::Incomplete(::nom::Needed::Size(":".len()))
                                                           } else if (i).starts_with(":")
                                                            {
                                                               ::nom::IResult::Done(&i[":".len()..],
                                                                                    &i[0..":".len()])
                                                           } else {
                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                          i))
                                                           };
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, _) =>
                                                 {
                                                     match {
                                                               let re =
                                                                   ::regex::Regex::new("^[0-9]{2}").unwrap();
                                                               if let Some((begin,
                                                                            end))
                                                                      =
                                                                      re.find(i)
                                                                      {
                                                                   ::nom::IResult::Done(&i[end..],
                                                                                        &i[begin..end])
                                                               } else {
                                                                   ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                                               }
                                                           } {
                                                         ::nom::IResult::Error(e)
                                                         =>
                                                         ::nom::IResult::Error(e),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            i)),
                                                         ::nom::IResult::Done(i,
                                                                              o)
                                                         => {
                                                             let minute = o;
                                                             ::nom::IResult::Done(i,
                                                                                  (||
                                                                                       {
                                                                                      TimeOffsetAmount{pos_neg:
                                                                                                           pos_neg,
                                                                                                       hour:
                                                                                                           hour,
                                                                                                       minute:
                                                                                                           minute,}
                                                                                  })())
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn time_offset(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, TimeOffset, u32>) {
            (self,
             {
                 {
                     match {
                               match {
                                         let res: ::nom::IResult<_, _> =
                                             if "Z".len() > i.len() {
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size("Z".len()))
                                             } else if (i).starts_with("Z") {
                                                 ::nom::IResult::Done(&i["Z".len()..],
                                                                      &i[0.."Z".len()])
                                             } else {
                                                 ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                            i))
                                             };
                                         res
                                     } {
                                   ::nom::IResult::Done(i, o) =>
                                   ::nom::IResult::Done(i, o),
                                   ::nom::IResult::Error(e) =>
                                   ::nom::IResult::Error(e),
                                   ::nom::IResult::Incomplete(_) => {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                  i))
                                   }
                               }
                           } {
                         ::nom::IResult::Done(i, o) =>
                         ::nom::IResult::Done(i, (|_| TimeOffset::Z)(o)),
                         ::nom::IResult::Incomplete(x) =>
                         ::nom::IResult::Incomplete(x),
                         ::nom::IResult::Error(_) => {
                             {
                                 match {
                                           match {
                                                     let (tmp, res) =
                                                         self.time_offset_amount(i);
                                                     self = tmp;
                                                     res
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(_)
                                               => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                              i))
                                               }
                                           }
                                       } {
                                     ::nom::IResult::Done(i, o) =>
                                     ::nom::IResult::Done(i,
                                                          (|offset|
                                                               TimeOffset::Time(offset))(o)),
                                     ::nom::IResult::Incomplete(x) =>
                                     ::nom::IResult::Incomplete(x),
                                     ::nom::IResult::Error(_) => {
                                         ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                    i))
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn full_date(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, FullDate, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let re =
                                   ::regex::Regex::new("^([0-9]{4})").unwrap();
                               if let Some((begin, end)) = re.find(i) {
                                   ::nom::IResult::Done(&i[end..],
                                                        &i[begin..end])
                               } else {
                                   ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                               }
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let year = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if "-".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("-".len()))
                                               } else if (i).starts_with("-")
                                                {
                                                   ::nom::IResult::Done(&i["-".len()..],
                                                                        &i[0.."-".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let re =
                                                           ::regex::Regex::new("^([0-9]{2})").unwrap();
                                                       if let Some((begin,
                                                                    end)) =
                                                              re.find(i) {
                                                           ::nom::IResult::Done(&i[end..],
                                                                                &i[begin..end])
                                                       } else {
                                                           ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                                       }
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let month = o;
                                                     {
                                                         use nom::InputLength;
                                                         match {
                                                                   let res:
                                                                           ::nom::IResult<_,
                                                                                          _> =
                                                                       if "-".len()
                                                                              >
                                                                              i.len()
                                                                          {
                                                                           ::nom::IResult::Incomplete(::nom::Needed::Size("-".len()))
                                                                       } else if (i).starts_with("-")
                                                                        {
                                                                           ::nom::IResult::Done(&i["-".len()..],
                                                                                                &i[0.."-".len()])
                                                                       } else {
                                                                           ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                                                      i))
                                                                       };
                                                                   res
                                                               } {
                                                             ::nom::IResult::Error(e)
                                                             =>
                                                             ::nom::IResult::Error(e),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                             =>
                                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                ((i).input_len()
                                                                                                                     -
                                                                                                                     i.input_len())
                                                                                                                +
                                                                                                                i)),
                                                             ::nom::IResult::Done(i,
                                                                                  _)
                                                             => {
                                                                 match {
                                                                           let re =
                                                                               ::regex::Regex::new("^([0-9]{2})").unwrap();
                                                                           if let Some((begin,
                                                                                        end))
                                                                                  =
                                                                                  re.find(i)
                                                                                  {
                                                                               ::nom::IResult::Done(&i[end..],
                                                                                                    &i[begin..end])
                                                                           } else {
                                                                               ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                                                                           }
                                                                       } {
                                                                     ::nom::IResult::Error(e)
                                                                     =>
                                                                     ::nom::IResult::Error(e),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                                     =>
                                                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        ((i).input_len()
                                                                                                                             -
                                                                                                                             i.input_len())
                                                                                                                        +
                                                                                                                        i)),
                                                                     ::nom::IResult::Done(i,
                                                                                          o)
                                                                     => {
                                                                         let day =
                                                                             o;
                                                                         ::nom::IResult::Done(i,
                                                                                              (||
                                                                                                   {
                                                                                                  FullDate{year:
                                                                                                               year,
                                                                                                           month:
                                                                                                               month,
                                                                                                           day:
                                                                                                               day,}
                                                                                              })())
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn date_time(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, DateTime, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match {
                               let (tmp, res) = self.full_date(i);
                               self = tmp;
                               res
                           } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let date = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if "T".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("T".len()))
                                               } else if (i).starts_with("T")
                                                {
                                                   ::nom::IResult::Done(&i["T".len()..],
                                                                        &i[0.."T".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         {
                                             use nom::InputLength;
                                             match {
                                                       let (tmp, res) =
                                                           self.time(i);
                                                       self = tmp;
                                                       res
                                                   } {
                                                 ::nom::IResult::Error(e) =>
                                                 ::nom::IResult::Error(e),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                 =>
                                                 ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    ((i).input_len()
                                                                                                         -
                                                                                                         i.input_len())
                                                                                                    +
                                                                                                    i)),
                                                 ::nom::IResult::Done(i, o) =>
                                                 {
                                                     let time = o;
                                                     match {
                                                               let (tmp,
                                                                    res) =
                                                                   self.time_offset(i);
                                                               self = tmp;
                                                               res
                                                           } {
                                                         ::nom::IResult::Error(e)
                                                         =>
                                                         ::nom::IResult::Error(e),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                                         =>
                                                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            ((i).input_len()
                                                                                                                 -
                                                                                                                 i.input_len())
                                                                                                            +
                                                                                                            i)),
                                                         ::nom::IResult::Done(i,
                                                                              o)
                                                         => {
                                                             let offset = o;
                                                             ::nom::IResult::Done(i,
                                                                                  (||
                                                                                       {
                                                                                      DateTime{year:
                                                                                                   date.year,
                                                                                               month:
                                                                                                   date.month,
                                                                                               day:
                                                                                                   date.day,
                                                                                               hour:
                                                                                                   time.hour,
                                                                                               minute:
                                                                                                   time.minute,
                                                                                               second:
                                                                                                   time.second,
                                                                                               fraction:
                                                                                                   time.fraction,
                                                                                               offset:
                                                                                                   offset,}
                                                                                  })())
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn unquoted_key(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let mut offset = i.len();
                 for (o, c) in i.char_indices() {
                     if !is_keychar(c) { offset = o; break ; }
                 }
                 if offset == 0 {
                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TakeWhile1Str,
                                                                i))
                 } else if offset < i.len() {
                     ::nom::IResult::Done(&i[offset..], &i[..offset])
                 } else { ::nom::IResult::Done("", i) }
             })
        }
        fn quoted_key(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 let re =
                     ::regex::Regex::new("^\"( |!|[#-\\[]|[\\]-\u{10ffff}]|(\\\\\")|(\\\\\\\\)|(\\\\/)|(\\\\b)|(\\\\f)|(\\\\n)|(\\\\r)|(\\\\t)|(\\\\u[0-9A-Z]{4})|(\\\\U[0-9A-Z]{8}))+\"").unwrap();
                 if let Some((begin, end)) = re.find(i) {
                     ::nom::IResult::Done(&i[end..], &i[begin..end])
                 } else {
                     ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
                 }
             })
        }
        pub fn key(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, &'a str, u32>) {
            (self,
             {
                 {
                     let res =
                         {
                             match {
                                       let (tmp, res) = self.quoted_key(i);
                                       self = tmp;
                                       res
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Error(e) =>
                                 ::nom::IResult::Error(e),
                                 ::nom::IResult::Incomplete(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                i))
                                 }
                             }
                         };
                     match res {
                         ::nom::IResult::Done(_, _) => res,
                         ::nom::IResult::Incomplete(_) => res,
                         _ => {
                             match {
                                       match {
                                                 let (tmp, res) =
                                                     self.unquoted_key(i);
                                                 self = tmp;
                                                 res
                                             } {
                                           ::nom::IResult::Done(i, o) =>
                                           ::nom::IResult::Done(i, o),
                                           ::nom::IResult::Error(e) =>
                                           ::nom::IResult::Error(e),
                                           ::nom::IResult::Incomplete(_) => {
                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                          i))
                                           }
                                       }
                                   } {
                                 ::nom::IResult::Done(i, o) =>
                                 ::nom::IResult::Done(i, o),
                                 ::nom::IResult::Incomplete(x) =>
                                 ::nom::IResult::Incomplete(x),
                                 ::nom::IResult::Error(_) => {
                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                i))
                                 }
                             }
                         }
                     }
                 }
             })
        }
        fn keyval_sep(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, WSSep, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.ws(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let ws1 = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let res: ::nom::IResult<_, _> =
                                               if "=".len() > i.len() {
                                                   ::nom::IResult::Incomplete(::nom::Needed::Size("=".len()))
                                               } else if (i).starts_with("=")
                                                {
                                                   ::nom::IResult::Done(&i["=".len()..],
                                                                        &i[0.."=".len()])
                                               } else {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::TagStr,
                                                                                              i))
                                               };
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, _) => {
                                         match {
                                                   let (tmp, res) =
                                                       self.ws(i);
                                                   self = tmp;
                                                   res
                                               } {
                                             ::nom::IResult::Error(e) =>
                                             ::nom::IResult::Error(e),
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                i)),
                                             ::nom::IResult::Done(i, o) => {
                                                 let ws2 = o;
                                                 ::nom::IResult::Done(i,
                                                                      (|| {
                                                                          WSSep{ws1:
                                                                                    ws1,
                                                                                ws2:
                                                                                    ws2,}
                                                                      })())
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        pub fn val(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, Value, u32>) {
            (self,
             {
                 {
                     match {
                               match {
                                         let (tmp, res) = self.array(i);
                                         self = tmp;
                                         res
                                     } {
                                   ::nom::IResult::Done(i, o) =>
                                   ::nom::IResult::Done(i, o),
                                   ::nom::IResult::Error(e) =>
                                   ::nom::IResult::Error(e),
                                   ::nom::IResult::Incomplete(_) => {
                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                  i))
                                   }
                               }
                           } {
                         ::nom::IResult::Done(i, o) =>
                         ::nom::IResult::Done(i,
                                              (|arr|
                                                   Value::Array(Box::new(arr)))(o)),
                         ::nom::IResult::Incomplete(x) =>
                         ::nom::IResult::Incomplete(x),
                         ::nom::IResult::Error(_) => {
                             {
                                 match {
                                           match {
                                                     let (tmp, res) =
                                                         self.inline_table(i);
                                                     self = tmp;
                                                     res
                                                 } {
                                               ::nom::IResult::Done(i, o) =>
                                               ::nom::IResult::Done(i, o),
                                               ::nom::IResult::Error(e) =>
                                               ::nom::IResult::Error(e),
                                               ::nom::IResult::Incomplete(_)
                                               => {
                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                              i))
                                               }
                                           }
                                       } {
                                     ::nom::IResult::Done(i, o) =>
                                     ::nom::IResult::Done(i,
                                                          (|it|
                                                               Value::InlineTable(Box::new(it)))(o)),
                                     ::nom::IResult::Incomplete(x) =>
                                     ::nom::IResult::Incomplete(x),
                                     ::nom::IResult::Error(_) => {
                                         {
                                             match {
                                                       match {
                                                                 let (tmp,
                                                                      res) =
                                                                     self.date_time(i);
                                                                 self = tmp;
                                                                 res
                                                             } {
                                                           ::nom::IResult::Done(i,
                                                                                o)
                                                           =>
                                                           ::nom::IResult::Done(i,
                                                                                o),
                                                           ::nom::IResult::Error(e)
                                                           =>
                                                           ::nom::IResult::Error(e),
                                                           ::nom::IResult::Incomplete(_)
                                                           => {
                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                          i))
                                                           }
                                                       }
                                                   } {
                                                 ::nom::IResult::Done(i, o) =>
                                                 ::nom::IResult::Done(i,
                                                                      (|dt|
                                                                           Value::DateTime(dt))(o)),
                                                 ::nom::IResult::Incomplete(x)
                                                 =>
                                                 ::nom::IResult::Incomplete(x),
                                                 ::nom::IResult::Error(_) => {
                                                     {
                                                         match {
                                                                   match {
                                                                             let (tmp,
                                                                                  res) =
                                                                                 self.float(i);
                                                                             self
                                                                                 =
                                                                                 tmp;
                                                                             res
                                                                         } {
                                                                       ::nom::IResult::Done(i,
                                                                                            o)
                                                                       =>
                                                                       ::nom::IResult::Done(i,
                                                                                            o),
                                                                       ::nom::IResult::Error(e)
                                                                       =>
                                                                       ::nom::IResult::Error(e),
                                                                       ::nom::IResult::Incomplete(_)
                                                                       => {
                                                                           ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                      i))
                                                                       }
                                                                   }
                                                               } {
                                                             ::nom::IResult::Done(i,
                                                                                  o)
                                                             =>
                                                             ::nom::IResult::Done(i,
                                                                                  (|flt|
                                                                                       Value::Float(flt))(o)),
                                                             ::nom::IResult::Incomplete(x)
                                                             =>
                                                             ::nom::IResult::Incomplete(x),
                                                             ::nom::IResult::Error(_)
                                                             => {
                                                                 {
                                                                     match {
                                                                               match {
                                                                                         let (tmp,
                                                                                              res) =
                                                                                             self.integer(i);
                                                                                         self
                                                                                             =
                                                                                             tmp;
                                                                                         res
                                                                                     }
                                                                                   {
                                                                                   ::nom::IResult::Done(i,
                                                                                                        o)
                                                                                   =>
                                                                                   ::nom::IResult::Done(i,
                                                                                                        o),
                                                                                   ::nom::IResult::Error(e)
                                                                                   =>
                                                                                   ::nom::IResult::Error(e),
                                                                                   ::nom::IResult::Incomplete(_)
                                                                                   =>
                                                                                   {
                                                                                       ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                                  i))
                                                                                   }
                                                                               }
                                                                           } {
                                                                         ::nom::IResult::Done(i,
                                                                                              o)
                                                                         =>
                                                                         ::nom::IResult::Done(i,
                                                                                              (|int|
                                                                                                   Value::Integer(int))(o)),
                                                                         ::nom::IResult::Incomplete(x)
                                                                         =>
                                                                         ::nom::IResult::Incomplete(x),
                                                                         ::nom::IResult::Error(_)
                                                                         => {
                                                                             {
                                                                                 match {
                                                                                           match {
                                                                                                     let (tmp,
                                                                                                          res) =
                                                                                                         self.boolean(i);
                                                                                                     self
                                                                                                         =
                                                                                                         tmp;
                                                                                                     res
                                                                                                 }
                                                                                               {
                                                                                               ::nom::IResult::Done(i,
                                                                                                                    o)
                                                                                               =>
                                                                                               ::nom::IResult::Done(i,
                                                                                                                    o),
                                                                                               ::nom::IResult::Error(e)
                                                                                               =>
                                                                                               ::nom::IResult::Error(e),
                                                                                               ::nom::IResult::Incomplete(_)
                                                                                               =>
                                                                                               {
                                                                                                   ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                                              i))
                                                                                               }
                                                                                           }
                                                                                       }
                                                                                     {
                                                                                     ::nom::IResult::Done(i,
                                                                                                          o)
                                                                                     =>
                                                                                     ::nom::IResult::Done(i,
                                                                                                          (|b|
                                                                                                               Value::Boolean(b))(o)),
                                                                                     ::nom::IResult::Incomplete(x)
                                                                                     =>
                                                                                     ::nom::IResult::Incomplete(x),
                                                                                     ::nom::IResult::Error(_)
                                                                                     =>
                                                                                     {
                                                                                         {
                                                                                             match {
                                                                                                       match {
                                                                                                                 let (tmp,
                                                                                                                      res) =
                                                                                                                     self.string(i);
                                                                                                                 self
                                                                                                                     =
                                                                                                                     tmp;
                                                                                                                 res
                                                                                                             }
                                                                                                           {
                                                                                                           ::nom::IResult::Done(i,
                                                                                                                                o)
                                                                                                           =>
                                                                                                           ::nom::IResult::Done(i,
                                                                                                                                o),
                                                                                                           ::nom::IResult::Error(e)
                                                                                                           =>
                                                                                                           ::nom::IResult::Error(e),
                                                                                                           ::nom::IResult::Incomplete(_)
                                                                                                           =>
                                                                                                           {
                                                                                                               ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Complete,
                                                                                                                                                          i))
                                                                                                           }
                                                                                                       }
                                                                                                   }
                                                                                                 {
                                                                                                 ::nom::IResult::Done(i,
                                                                                                                      o)
                                                                                                 =>
                                                                                                 ::nom::IResult::Done(i,
                                                                                                                      (|s|
                                                                                                                           s)(o)),
                                                                                                 ::nom::IResult::Incomplete(x)
                                                                                                 =>
                                                                                                 ::nom::IResult::Incomplete(x),
                                                                                                 ::nom::IResult::Error(_)
                                                                                                 =>
                                                                                                 {
                                                                                                     ::nom::IResult::Error(::nom::Err::Position(::nom::ErrorKind::Alt,
                                                                                                                                                i))
                                                                                                 }
                                                                                             }
                                                                                         }
                                                                                     }
                                                                                 }
                                                                             }
                                                                         }
                                                                     }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
        pub fn keyval(mut self: Parser<'a>, i: &'a str)
         -> (Parser<'a>, ::nom::IResult<&'a str, KeyVal, u32>) {
            (self,
             {
                 {
                     use nom::InputLength;
                     match { let (tmp, res) = self.key(i); self = tmp; res } {
                         ::nom::IResult::Error(e) => ::nom::IResult::Error(e),
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                         ::nom::IResult::Incomplete(::nom::Needed::Size(i)) =>
                         ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                            +
                                                                            i)),
                         ::nom::IResult::Done(i, o) => {
                             let key = o;
                             {
                                 use nom::InputLength;
                                 match {
                                           let (tmp, res) =
                                               self.keyval_sep(i);
                                           self = tmp;
                                           res
                                       } {
                                     ::nom::IResult::Error(e) =>
                                     ::nom::IResult::Error(e),
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                     =>
                                     ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                        +
                                                                                        ((i).input_len()
                                                                                             -
                                                                                             i.input_len())
                                                                                        +
                                                                                        i)),
                                     ::nom::IResult::Done(i, o) => {
                                         let ws = o;
                                         match {
                                                   let (tmp, res) =
                                                       self.val(i);
                                                   self = tmp;
                                                   res
                                               } {
                                             ::nom::IResult::Error(e) =>
                                             ::nom::IResult::Error(e),
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Unknown),
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                             =>
                                             ::nom::IResult::Incomplete(::nom::Needed::Size(0usize
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                ((i).input_len()
                                                                                                     -
                                                                                                     i.input_len())
                                                                                                +
                                                                                                i)),
                                             ::nom::IResult::Done(i, o) => {
                                                 let val = o;
                                                 ::nom::IResult::Done(i,
                                                                      (|| {
                                                                          KeyVal{key:
                                                                                     key,
                                                                                 keyval_sep:
                                                                                     ws,
                                                                                 val:
                                                                                     val,}
                                                                      })())
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                 }
             })
        }
    }
}
mod types {
    #[prelude_import]
    use std::prelude::v1::*;
    use std::collections::LinkedList;
    use ast::structs::{Array, InlineTable};
    use ast::structs::Value;
    use std::hash::{Hash, Hasher};
    pub struct HashValue<'a> {
        value: Option<Value<'a>>,
        subkeys: LinkedList<&'a str>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::cmp::Eq for HashValue<'a> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match *self {
                HashValue { value: ref __self_0_0, subkeys: ref __self_0_1 }
                => {
                    (*__self_0_0).assert_receiver_is_total_eq();
                    (*__self_0_1).assert_receiver_is_total_eq();
                }
            }
        }
    }
    impl <'a> HashValue<'a> {
        pub fn new(value: Value<'a>) -> HashValue<'a> {
            HashValue{value: Some(value), subkeys: LinkedList::new(),}
        }
    }
    impl <'a> PartialEq for HashValue<'a> {
        fn eq(&self, other: &HashValue<'a>) -> bool {
            self.value == other.value
        }
    }
    pub enum ParseResult<'a> {
        Success(&'a str),
        SuccessComplete(&'a str, &'a str),
        Error(Vec<ParseError<'a>>),
        Failure,
    }
    pub enum TimeOffset<'a> { Z, Time(TimeOffsetAmount<'a>), }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::cmp::Eq for TimeOffset<'a> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match (&*self,) {
                (&TimeOffset::Z,) => { }
                (&TimeOffset::Time(ref __self_0),) => {
                    (*__self_0).assert_receiver_is_total_eq();
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::fmt::Debug for TimeOffset<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match (&*self,) {
                (&TimeOffset::Z,) => {
                    let mut builder = __arg_0.debug_tuple("Z");
                    builder.finish()
                }
                (&TimeOffset::Time(ref __self_0),) => {
                    let mut builder = __arg_0.debug_tuple("Time");
                    let _ = builder.field(&&(*__self_0));
                    builder.finish()
                }
            }
        }
    }
    pub struct TimeOffsetAmount<'a> {
        pub pos_neg: &'a str,
        pub hour: &'a str,
        pub minute: &'a str,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::cmp::Eq for TimeOffsetAmount<'a> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match *self {
                TimeOffsetAmount {
                pos_neg: ref __self_0_0,
                hour: ref __self_0_1,
                minute: ref __self_0_2 } => {
                    (*__self_0_0).assert_receiver_is_total_eq();
                    (*__self_0_1).assert_receiver_is_total_eq();
                    (*__self_0_2).assert_receiver_is_total_eq();
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::fmt::Debug for TimeOffsetAmount<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                TimeOffsetAmount {
                pos_neg: ref __self_0_0,
                hour: ref __self_0_1,
                minute: ref __self_0_2 } => {
                    let mut builder =
                        __arg_0.debug_struct("TimeOffsetAmount");
                    let _ = builder.field("pos_neg", &&(*__self_0_0));
                    let _ = builder.field("hour", &&(*__self_0_1));
                    let _ = builder.field("minute", &&(*__self_0_2));
                    builder.finish()
                }
            }
        }
    }
    pub struct DateTime<'a> {
        pub year: &'a str,
        pub month: &'a str,
        pub day: &'a str,
        pub hour: &'a str,
        pub minute: &'a str,
        pub second: &'a str,
        pub fraction: &'a str,
        pub offset: TimeOffset<'a>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::cmp::Eq for DateTime<'a> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            match *self {
                DateTime {
                year: ref __self_0_0,
                month: ref __self_0_1,
                day: ref __self_0_2,
                hour: ref __self_0_3,
                minute: ref __self_0_4,
                second: ref __self_0_5,
                fraction: ref __self_0_6,
                offset: ref __self_0_7 } => {
                    (*__self_0_0).assert_receiver_is_total_eq();
                    (*__self_0_1).assert_receiver_is_total_eq();
                    (*__self_0_2).assert_receiver_is_total_eq();
                    (*__self_0_3).assert_receiver_is_total_eq();
                    (*__self_0_4).assert_receiver_is_total_eq();
                    (*__self_0_5).assert_receiver_is_total_eq();
                    (*__self_0_6).assert_receiver_is_total_eq();
                    (*__self_0_7).assert_receiver_is_total_eq();
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::fmt::Debug for DateTime<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter)
         -> ::std::fmt::Result {
            match *self {
                DateTime {
                year: ref __self_0_0,
                month: ref __self_0_1,
                day: ref __self_0_2,
                hour: ref __self_0_3,
                minute: ref __self_0_4,
                second: ref __self_0_5,
                fraction: ref __self_0_6,
                offset: ref __self_0_7 } => {
                    let mut builder = __arg_0.debug_struct("DateTime");
                    let _ = builder.field("year", &&(*__self_0_0));
                    let _ = builder.field("month", &&(*__self_0_1));
                    let _ = builder.field("day", &&(*__self_0_2));
                    let _ = builder.field("hour", &&(*__self_0_3));
                    let _ = builder.field("minute", &&(*__self_0_4));
                    let _ = builder.field("second", &&(*__self_0_5));
                    let _ = builder.field("fraction", &&(*__self_0_6));
                    let _ = builder.field("offset", &&(*__self_0_7));
                    builder.finish()
                }
            }
        }
    }
    pub enum ParseError<'a> {
        MixedArray(Array<'a>),
        DuplicateTableName(String, Box<Fn(&String)>),
    }
}
pub mod parser {
    #[prelude_import]
    use std::prelude::v1::*;
    use std::fmt;
    use std::fmt::Display;
    use std::collections::HashMap;
    use std::cell::{RefCell, Cell, RefMut};
    use std::thread;
    use nom::IResult;
    use ast::structs::{Toml};
    use types::HashValue;
    use nom;
    use std::any::Any;
    use ast::structs::{Expression, NLExpression, WSSep, Comment};
    fn full_line(i: &str) -> ::nom::IResult<&str, &str, u32> {
        {
            let re = ::regex::Regex::new("^(.*?)(\n|(\r\n))").unwrap();
            if let Some((begin, end)) = re.find(i) {
                ::nom::IResult::Done(&i[end..], &i[begin..end])
            } else {
                ::nom::IResult::Error(::nom::Err::Code(::nom::ErrorKind::RegexpFind))
            }
        }
    }
    fn all_lines(i: &str) -> ::nom::IResult<&str, Vec<&str>, u32> {
        {
            use nom::InputLength;
            if (i).input_len() == 0 {
                ::nom::IResult::Done(i, ::std::vec::Vec::new())
            } else {
                match full_line(i) {
                    ::nom::IResult::Error(_) => {
                        ::nom::IResult::Done(i, ::std::vec::Vec::new())
                    }
                    ::nom::IResult::Incomplete(i) =>
                    ::nom::IResult::Incomplete(i),
                    ::nom::IResult::Done(i1, o1) => {
                        if i1.input_len() == 0 {
                            ::nom::IResult::Done(i1,
                                                 <[_]>::into_vec(::std::boxed::Box::new([o1])))
                        } else {
                            let mut res = ::std::vec::Vec::with_capacity(4);
                            res.push(o1);
                            let mut input = i1;
                            let mut incomplete:
                                    ::std::option::Option<::nom::Needed> =
                                ::std::option::Option::None;
                            loop  {
                                match full_line(input) {
                                    ::nom::IResult::Done(i, o) => {
                                        if i.input_len() == input.input_len()
                                           {
                                            break ;
                                        }
                                        res.push(o);
                                        input = i;
                                    }
                                    ::nom::IResult::Error(_) => { break ; }
                                    ::nom::IResult::Incomplete(::nom::Needed::Unknown)
                                    => {
                                        incomplete =
                                            ::std::option::Option::Some(::nom::Needed::Unknown);
                                        break ;
                                    }
                                    ::nom::IResult::Incomplete(::nom::Needed::Size(i))
                                    => {
                                        incomplete =
                                            ::std::option::Option::Some(::nom::Needed::Size(i
                                                                                                +
                                                                                                (i).input_len()
                                                                                                -
                                                                                                input.input_len()));
                                        break ;
                                    }
                                }
                                if input.input_len() == 0 { break ; }
                            }
                            match incomplete {
                                ::std::option::Option::Some(i) =>
                                ::nom::IResult::Incomplete(i),
                                ::std::option::Option::None =>
                                ::nom::IResult::Done(input, res),
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn count_lines(s: &str) -> u64 {
        let r = all_lines(s);
        match &r {
            &IResult::Done(_, ref o) => o.len() as u64,
            _ => 0 as u64,
        }
    }
    pub struct Parser<'a> {
        pub root: RefCell<Toml<'a>>,
        pub map: RefCell<HashMap<&'a str, HashValue<'a>>>,
        pub leftover: RefCell<&'a str>,
        pub line_count: Cell<u64>,
        pub last_table: RefCell<&'a str>,
        pub input: &'a str,
        pub vec_nl_expr: Vec<NLExpression<'a>>,
        pub expression: Expression<'a>,
    }
    impl <'a> Default for Parser<'a> {
        fn default() -> Parser<'a> {
            Parser{root:
                       RefCell::new(Toml{exprs:
                                             <[_]>::into_vec(::std::boxed::Box::new([])),}),
                   map: RefCell::new(HashMap::new()),
                   leftover: RefCell::new(""),
                   line_count: Cell::new(0u64),
                   last_table: RefCell::new(""),
                   input: "",
                   vec_nl_expr: <[_]>::into_vec(::std::boxed::Box::new([])),
                   expression:
                       Expression{ws: WSSep{ws1: "", ws2: "",},
                                  keyval: None,
                                  table: None,
                                  comment:
                                      Some(Comment{text:
                                                       " T\u{3bb}\u{ef}\u{1a8} \u{ef}\u{1a8} \u{e1} T\u{d3}M\u{a3} \u{3b4}\u{f4}\u{e7}\u{fa}\u{20a5}\u{e8}\u{f1}\u{1ad}.",}),},}
        }
    }
    impl <'a> Parser<'a> {
        pub fn new<'b>() -> Parser<'a> {
            Parser{root:
                       RefCell::new(Toml{exprs:
                                             <[_]>::into_vec(::std::boxed::Box::new([])),}),
                   map: RefCell::new(HashMap::new()),
                   leftover: RefCell::new(""),
                   line_count: Cell::new(0),
                   last_table: RefCell::new(""),
                   input: "",
                   vec_nl_expr: <[_]>::into_vec(::std::boxed::Box::new([])),
                   expression:
                       Expression{ws: WSSep{ws1: "", ws2: "",},
                                  keyval: None,
                                  table: None,
                                  comment:
                                      Some(Comment{text:
                                                       " T\u{3bb}\u{ef}\u{1a8} \u{ef}\u{1a8} \u{e1} T\u{d3}M\u{a3} \u{3b4}\u{f4}\u{e7}\u{fa}\u{20a5}\u{e8}\u{f1}\u{1ad}.",}),},}
        }
        pub fn parse(mut self: Parser<'a>, input: &'a str) {
            let (tmp, mut res) = self.toml(input);
            self = tmp;
            match res {
                IResult::Done(i, o) => {
                    *self.root.borrow_mut() = o;
                    *self.leftover.borrow_mut() = i;
                }
                _ => (),
            };
        }
    }
    impl <'a> Display for Parser<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_fmt(::std::fmt::Arguments::new_v1({
                                                          static __STATIC_FMTSTR:
                                                                 &'static [&'static str]
                                                                 =
                                                              &[""];
                                                          __STATIC_FMTSTR
                                                      },
                                                      &match (&*self.root.borrow(),)
                                                           {
                                                           (__arg0,) =>
                                                           [::std::fmt::ArgumentV1::new(__arg0,
                                                                                        ::std::fmt::Display::fmt)],
                                                       }))
        }
    }
}

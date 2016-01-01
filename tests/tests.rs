#[cfg(test)]
use nom::IResult::{Done};
use ::{literal_string, ml_literal_string, boolean, partial_time,
       time_offset_amount, time_offset, full_time, full_date,
       date_time, array, inline_table_keyvals_non_empty, inline_table};
use ast::{Val, PartialTime, TimeOffsetAmount, TimeOffset, FullTime, PosNeg, WSSep,
          FullDate, DateTime, ArrayValues, Array, TableKeyVals, InlineTable};

#[test]
fn test_literal_string() {
	assert_eq!(literal_string("'Abc џ'"), Done("", "'Abc џ'"));	
}

#[test]
fn test_ml_literal_string() {
  assert_eq!(ml_literal_string(r#"'''
                                  Abc џ
                                  '''"#),
    Done("", r#"'''
                                  Abc џ
                                  '''"#));
}

#[test]
fn test_boolean() {
  assert_eq!(boolean("true"), Done("", "true"));
  assert_eq!(boolean("false"), Done("", "false"));
}

#[test]
fn test_partial_time() {
  assert_eq!(partial_time("11:22:33.456"),
    Done("", PartialTime{
      hour: "11",
      minute: "22",
      second: "33",
      fraction: "456"
    })
  );
  assert_eq!(partial_time("04:05:06"),
    Done("", PartialTime{
      hour: "04",
      minute: "05",
      second: "06",
      fraction: ""
    })
  );
}

#[test]
fn test_time_offset_amount() {
  assert_eq!(time_offset_amount("+12:34"),
    Done("", TimeOffsetAmount{
      pos_neg: PosNeg::Pos,
      hour: "12",
      minute: "34"
    })
  );
}

#[test]
fn test_time_offset() {
  assert_eq!(time_offset("+12:34"),
    Done("", TimeOffset::Time(TimeOffsetAmount{
      pos_neg: PosNeg::Pos,
      hour: "12",
      minute: "34"
    }))
  );
  assert_eq!(time_offset("Z"), Done("", TimeOffset::Z));
}

#[test]
fn test_full_time() {
  assert_eq!(full_time("10:30:55.83+12:54"),
    Done("", FullTime{
      partial_time: PartialTime{
        hour: "10",
        minute: "30",
        second: "55",
        fraction: "83"
      },
      time_offset: TimeOffset::Time(TimeOffsetAmount{
        pos_neg: PosNeg::Pos,
        hour: "12",
        minute: "54"
      })
    })
  );
}

#[test]
fn test_full_date() {
  assert_eq!(full_date("1942-12-07"),
    Done("", FullDate{
      year: "1942", month: "12", day: "07"
    })
  );
}

#[test]
fn test_date_time() {
  assert_eq!(date_time("1999-03-21T20:15:44.5-07:00"),
    Done("", DateTime{
      date: FullDate{
        year: "1999", month: "03", day: "21"
      },
      time: FullTime{
        partial_time: PartialTime{
          hour: "20",
          minute: "15",
          second: "44",
          fraction: "5"
        },
        time_offset: TimeOffset::Time(TimeOffsetAmount{
          pos_neg: PosNeg::Neg,
          hour: "07",
          minute: "00"
        })
      }
    })
  );
}

#[test]
fn test_non_nested_array() {
  assert_eq!(array("[2010-10-10T10:10:10.33Z, 1950-03-30T21:04:14.123+05:00]"),
    Done("", Array {
      values: Some(ArrayValues {
        val: Val::DateTime(DateTime {
          date: FullDate {
            year: "2010", month: "10", day: "10"
          },
          time: FullTime {
            partial_time: PartialTime {
              hour: "10", minute: "10", second: "10", fraction: "33"
            },
            time_offset: TimeOffset::Z
          }
        }),
        array_sep: Some(WSSep{
          ws1: "", ws2: " "
        }),
        comment_nl: None, array_vals: Some(Box::new(ArrayValues{
          val: Val::DateTime(DateTime{
            date: FullDate {
              year: "1950", month: "03", day: "30"
            },
            time: FullTime{
              partial_time: PartialTime{
                hour: "21", minute: "04", second: "14", fraction: "123"
              },
              time_offset: TimeOffset::Time(TimeOffsetAmount{
                pos_neg: PosNeg::Pos, hour: "05", minute: "00"
              })
            }
          }),
          array_sep: None, comment_nl: None, array_vals: None
        }))
      }),
      ws: WSSep{
        ws1: "", ws2: ""
      }
    }));
}

#[test]
fn test_nested_array() {
  assert_eq!(array("[[3,4], [4,5], [6]]"),
    Done("", Array{
      values: Some(ArrayValues {
        val: Val::Array(Box::new(Array { values: Some(ArrayValues {
          val: Val::Integer("3"), array_sep: Some(WSSep {
            ws1: "", ws2: ""
          }), comment_nl: None, array_vals: Some(Box::new(ArrayValues {
            val: Val::Integer("4"), array_sep: None, comment_nl: None, array_vals: None
          }))
        }),
        ws: WSSep {
          ws1: "", ws2: ""
        }
      })),
        array_sep: Some(WSSep {
          ws1: "", ws2: " "
        }),
        comment_nl: None, array_vals: Some(Box::new(ArrayValues {
          val: Val::Array(Box::new(Array {
            values: Some(ArrayValues {
              val: Val::Integer("4"), array_sep: Some(WSSep {
                ws1: "", ws2: ""
              }),
              comment_nl: None, array_vals: Some(Box::new(ArrayValues {
                val: Val::Integer("5"), array_sep: None, comment_nl: None, array_vals: None
              }))
            }),
            ws: WSSep {
              ws1: "", ws2: ""
            }
          })),
          array_sep: Some(WSSep {
            ws1: "", ws2: " "
          }),
          comment_nl: None, array_vals: Some(Box::new(ArrayValues {
            val: Val::Array(Box::new(Array {
              values: Some(ArrayValues {
                val: Val::Integer("6"), array_sep: None, comment_nl: None, array_vals: None
              }),
              ws: WSSep {
                ws1: "", ws2: ""
              }
            })),
            array_sep: None, comment_nl: None, array_vals: None
          }))
        }))
      }),
      ws: WSSep {
        ws1: "", ws2: ""
      }
    })
  );
}

#[test]
fn test_inline_table_keyvals_non_empty() {
  assert_eq!(inline_table_keyvals_non_empty("Key = 54 , \"Key2\" = '34.99'"),
    Done("", TableKeyVals{
      key: "Key", keyval_sep: WSSep{
        ws1: " ", ws2: " "
      },
      val: Val::Integer("54"), table_sep: Some(WSSep{
        ws1: " ", ws2: " "
      }),
      keyvals: Some(Box::new(TableKeyVals{
        key: "\"Key2\"", keyval_sep: WSSep{
          ws1: " ", ws2: " "
        },
        val: Val::String("'34.99'"), table_sep: None, keyvals: None
      }))
    })
  );
}

#[test]
fn test_inline_table() {
  assert_eq!(inline_table("{\tKey = 3.14E+5 , \"Key2\" = '''New\nLine'''\t}"),
    Done("", InlineTable{
      keyvals: TableKeyVals{
        key: "Key", keyval_sep: WSSep{
          ws1: " ", ws2: " "
        },
        val: Val::Float("3.14E+5"), table_sep: Some(WSSep{
          ws1: " ", ws2: " "
        }),
        keyvals: Some(Box::new(TableKeyVals{
          key: "\"Key2\"", keyval_sep: WSSep{
            ws1: " ", ws2: " "
          },
          val: Val::String("\'\'\'New\nLine\'\'\'"), table_sep: None, keyvals: None
        }))
      },
      ws: WSSep{
        ws1: "\t", ws2: "\t"
      }
    })
  );
}
// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_exception::Result;
use common_functions::scalars::InetAtonFunction;
use common_functions::scalars::InetNtoaFunction;
use common_functions::scalars::RunningDifferenceFunction;
use common_functions::scalars::TryInetAtonFunction;
use common_functions::scalars::TryInetNtoaFunction;

use super::scalar_function2_test::test_scalar_functions2;
use super::scalar_function2_test::ScalarFunction2Test;
use crate::scalars::scalar_function2_test::test_scalar_functions2_with_type;
use crate::scalars::scalar_function2_test::ScalarFunction2WithFieldTest;

#[test]
fn test_running_difference_first_null() -> Result<()> {
    use common_datavalues2::prelude::*;

    let tests = vec![
        ScalarFunction2Test {
            name: "i8_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i8),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i16)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u8_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u8),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i16)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i16_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i16),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i32)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u16_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u16),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i32)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i32_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i32),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u32_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u32),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i64_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_i64),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u64_first_null",
            columns: vec![Series::from_data([
                None,
                Some(1_u64),
                None,
                Some(3),
                Some(7),
            ])],
            expect: Series::from_data([None, None, None, None, Some(4_i64)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i8_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i8),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i16), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u8_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u8),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i16), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i16_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i16),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i32), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u16_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u16),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i32), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i32_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i32),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u32_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u32),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i64_first_not_null",
            columns: vec![Series::from_data([
                Some(2_i64),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u64_first_not_null",
            columns: vec![Series::from_data([
                Some(2_u64),
                Some(3),
                None,
                Some(4),
                Some(10),
            ])],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
    ];

    test_scalar_functions2(RunningDifferenceFunction::try_create("a")?, &tests)
}

#[test]
fn test_running_difference_datetime32_first_null() -> Result<()> {
    use common_datavalues2::prelude::*;
    use common_datavalues2::type_datetime32::DateTime32Type;

    let tests = vec![
        ScalarFunction2WithFieldTest {
            name: "datetime32_first_null",
            columns: vec![ColumnWithField::new(
                Series::from_data([None, Some(3_u32), None, Some(4), Some(10)]),
                DataField::new(
                    "dummy_1",
                    Arc::new(NullableType::create(DateTime32Type::arc(None))),
                ),
            )],
            expect: Series::from_data([None, None, None, None, Some(6_i64)]),
            error: "",
        },
        ScalarFunction2WithFieldTest {
            name: "datetime32_first_not_null",
            columns: vec![ColumnWithField::new(
                Series::from_data([Some(2_u32), Some(3), None, Some(4), Some(10)]),
                DataField::new(
                    "dummy_1",
                    Arc::new(NullableType::create(DateTime32Type::arc(None))),
                ),
            )],
            expect: Series::from_data([Some(0_i64), Some(1), None, None, Some(6)]),
            error: "",
        },
    ];

    test_scalar_functions2_with_type(RunningDifferenceFunction::try_create("a")?, &tests)
}

#[test]
fn test_try_inet_aton_function() -> Result<()> {
    use common_datavalues2::prelude::*;

    let tests = vec![
        ScalarFunction2Test {
            name: "valid input",
            columns: vec![Series::from_data(vec!["127.0.0.1"])],
            expect: Series::from_data(vec![Option::<u32>::Some(2130706433_u32)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "invalid input",
            columns: vec![Series::from_data(vec![Some("invalid")])],
            expect: Series::from_data(vec![Option::<u32>::None]),
            error: "",
        },
        ScalarFunction2Test {
            name: "null input",
            columns: vec![Series::from_data(vec![Option::<Vec<u8>>::None])],
            expect: Series::from_data(vec![Option::<u32>::None]),
            error: "",
        },
    ];

    let test_func = TryInetAtonFunction::try_create("try_inet_aton")?;
    test_scalar_functions2(test_func, &tests)
}

#[test]
fn test_inet_aton_function() -> Result<()> {
    use common_datavalues2::prelude::*;

    let tests = vec![
        ScalarFunction2Test {
            name: "valid input",
            columns: vec![Series::from_data([Some("127.0.0.1")])],
            expect: Series::from_data(vec![Option::<u32>::Some(2130706433_u32)]),
            error: "",
        },
        ScalarFunction2Test {
            name: "null input",
            columns: vec![Series::from_data([Option::<Vec<u8>>::None])],
            expect: Series::from_data([Option::<u32>::None]),
            error: "",
        },
        ScalarFunction2Test {
            name: "invalid input",
            columns: vec![Series::from_data([Some("1.1.1.1"), Some("batman")])],
            expect: Series::from_data(vec![Option::<u32>::None]),
            error: "Failed to parse 'batman' into a IPV4 address, invalid IP address syntax",
        },
        ScalarFunction2Test {
            name: "empty string",
            columns: vec![Series::from_data([Some("1.1.1.1"), Some("")])],
            expect: Series::from_data(vec![Option::<u32>::None]),
            error: "Failed to parse '' into a IPV4 address, invalid IP address syntax",
        },
    ];

    let test_func = InetAtonFunction::try_create("inet_aton")?;
    test_scalar_functions2(test_func, &tests)
}

#[test]
fn test_try_inet_ntoa_function() -> Result<()> {
    use common_datavalues2::prelude::*;

    let tests = vec![
        // integer input test cases
        ScalarFunction2Test {
            name: "integer_input_i32_positive",
            columns: vec![Series::from_data(vec![2130706433_i32])],
            expect: Series::from_data(vec![Some("127.0.0.1")]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_i32_negative",
            columns: vec![Series::from_data(vec![-1])],
            expect: Series::from_data(vec![Some("255.255.255.255")]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_u8",
            columns: vec![Series::from_data(vec![Some(0_u8)])],
            expect: Series::from_data(vec![Some("0.0.0.0")]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_u32",
            columns: vec![Series::from_data(vec![Some(3232235777_u32)])],
            expect: Series::from_data(vec![Some("192.168.1.1")]),
            error: "",
        },
        // float input test cases
        ScalarFunction2Test {
            name: "float_input_f64",
            columns: vec![Series::from_data(vec![2130706433.3917_f64])],
            expect: Series::from_data(vec![Some("127.0.0.1")]),
            error: "",
        },
        // string input test cases
        ScalarFunction2Test {
            name: "string_input_u32",
            columns: vec![Series::from_data(vec!["3232235777"])],
            expect: Series::from_data(vec![Some("192.168.1.1")]),
            error: "Expected numeric or null type, but got String",
        },
    ];

    let test_func = TryInetNtoaFunction::try_create("try_inet_ntoa")?;
    test_scalar_functions2(test_func, &tests)
}

#[test]
fn test_inet_ntoa_function() -> Result<()> {
    use common_datavalues2::prelude::*;

    let tests = vec![
        // integer input test cases
        ScalarFunction2Test {
            name: "integer_input_i32_positive",
            columns: vec![Series::from_data([2130706433_i32])],
            expect: Series::from_data(["127.0.0.1"]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_i32_negative",
            columns: vec![Series::from_data([-1])],
            expect: Series::from_data(["255.255.255.255"]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_u8",
            columns: vec![Series::from_data([Some(0_u8)])],
            expect: Series::from_data([Some("0.0.0.0")]),
            error: "",
        },
        ScalarFunction2Test {
            name: "integer_input_u32",
            columns: vec![Series::from_data([Some(3232235777_u32)])],
            expect: Series::from_data([Some("192.168.1.1")]),
            error: "",
        },
        // float input test cases
        ScalarFunction2Test {
            name: "float_input_f64",
            columns: vec![Series::from_data([2130706433.3917_f64])],
            expect: Series::from_data(["127.0.0.1"]),
            error: "",
        },
        // string input test cases
        ScalarFunction2Test {
            name: "string_input_empty",
            columns: vec![Series::from_data([""])],
            expect: Series::from_data([""]),
            error: "Expected numeric or null type, but got String",
        },
    ];

    let test_func = InetNtoaFunction::try_create("inet_ntoa")?;
    test_scalar_functions2(test_func, &tests)
}

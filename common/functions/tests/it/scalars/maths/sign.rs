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

use common_datavalues2::prelude::*;
use common_exception::Result;
use common_functions::scalars::*;

use crate::scalars::scalar_function2_test::test_scalar_functions2;
use crate::scalars::scalar_function2_test::ScalarFunction2Test;

#[test]
fn test_sign_function() -> Result<()> {
    let tests = vec![
        ScalarFunction2Test {
            name: "positive int",
            columns: vec![Series::from_data([11_i8])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "negative int",
            columns: vec![Series::from_data([-11_i8])],
            expect: Series::from_data([-1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "zero int",
            columns: vec![Series::from_data([0_i8])],
            expect: Series::from_data([0_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "with null",
            columns: vec![Series::from_data([Some(0_i8), None])],
            expect: Series::from_data([Some(0_i8), None]),
            error: "",
        },
        ScalarFunction2Test {
            name: "int as string",
            columns: vec![Series::from_data(["22"])],
            expect: Series::from_data([1_i8]),
            error: "Expected a numeric type, but got String",
        },
        ScalarFunction2Test {
            name: "i16",
            columns: vec![Series::from_data([11_i16])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i32",
            columns: vec![Series::from_data([11_i32])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "i64",
            columns: vec![Series::from_data([11_i64])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u8",
            columns: vec![Series::from_data([11_u8])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u16",
            columns: vec![Series::from_data([11_u16])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u32",
            columns: vec![Series::from_data([11_u32])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "u64",
            columns: vec![Series::from_data([11_u64])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "f32",
            columns: vec![Series::from_data([11.11_f32])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
        ScalarFunction2Test {
            name: "f64",
            columns: vec![Series::from_data([11.11_f64])],
            expect: Series::from_data([1_i8]),
            error: "",
        },
    ];

    let sign_f = SignFunction::try_create("sign")?;
    let sign_f = Function2Adapter::create(sign_f);
    test_scalar_functions2(sign_f, &tests)
}

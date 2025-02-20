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

use std::ops::Sub;

use common_datavalues2::prelude::*;
use common_datavalues2::with_match_date_type_error;
use common_datavalues2::with_match_primitive_type_id;
use common_exception::ErrorCode;
use common_exception::Result;
use num::traits::AsPrimitive;
use num_traits::WrappingSub;

use crate::scalars::function_factory::FunctionFeatures;
use crate::scalars::ArithmeticDescription;
use crate::scalars::BinaryArithmeticFunction;
use crate::scalars::Function2;
use crate::scalars::Monotonicity2;

fn sub_scalar<L, R, O>(l: L::RefType<'_>, r: R::RefType<'_>) -> O
where
    L: PrimitiveType + AsPrimitive<O>,
    R: PrimitiveType + AsPrimitive<O>,
    O: PrimitiveType + Sub<Output = O>,
{
    l.to_owned_scalar().as_() - r.to_owned_scalar().as_()
}

fn wrapping_sub_scalar<L, R, O>(l: L::RefType<'_>, r: R::RefType<'_>) -> O
where
    L: PrimitiveType + AsPrimitive<O>,
    R: PrimitiveType + AsPrimitive<O>,
    O: IntegerType + WrappingSub<Output = O>,
{
    l.to_owned_scalar()
        .as_()
        .wrapping_sub(&r.to_owned_scalar().as_())
}

pub struct ArithmeticMinusFunction;

impl ArithmeticMinusFunction {
    pub fn try_create_func(
        _display_name: &str,
        args: &[&DataTypePtr],
    ) -> Result<Box<dyn Function2>> {
        let op = DataValueBinaryOperator::Minus;
        let left_type = remove_nullable(args[0]).data_type_id();
        let right_type = remove_nullable(args[1]).data_type_id();

        let error_fn = || -> Result<Box<dyn Function2>> {
            Err(ErrorCode::BadDataValueType(format!(
                "DataValue Error: Unsupported arithmetic ({:?}) {} ({:?})",
                left_type, op, right_type
            )))
        };

        if left_type.is_interval() || right_type.is_interval() {
            todo!()
        }

        if left_type.is_date_or_date_time() {
            return with_match_date_type_error!(left_type, |$T| {
                with_match_primitive_type_id!(right_type, |$D| {
                    BinaryArithmeticFunction::<$T, $D, $T, _>::try_create_func(
                        op,
                        args[0].clone(),
                        sub_scalar::<$T,$D, _>
                    )
                },{
                    with_match_date_type_error!(right_type, |$D| {
                        BinaryArithmeticFunction::<$T, $D, i32, _>::try_create_func(
                            op,
                            Int32Type::arc(),
                            sub_scalar::<$T, $D, _>
                        )
                    })
                })
            });
        }

        if right_type.is_date_or_date_time() {
            return with_match_primitive_type_id!(left_type, |$T| {
                with_match_date_type_error!(right_type, |$D| {
                    BinaryArithmeticFunction::<$T, $D, $D, _>::try_create_func(
                        op,
                        args[1].clone(),
                        sub_scalar::<$T, $D, _>
                    )
                })
            },{
                error_fn()
            });
        }

        with_match_primitive_type_id!(left_type, |$T| {
            with_match_primitive_type_id!(right_type, |$D| {
                let result_type = <($T, $D) as ResultTypeOfBinary>::Minus::to_data_type();
                match result_type.data_type_id() {
                    TypeID::Int64 => BinaryArithmeticFunction::<$T, $D, i64, _>::try_create_func(
                        op,
                        result_type,
                        wrapping_sub_scalar::<$T, $D, _>
                    ),
                    _ => BinaryArithmeticFunction::<$T, $D, <($T, $D) as ResultTypeOfBinary>::Minus, _>::try_create_func(
                        op,
                        result_type,
                        sub_scalar::<$T, $D, _>
                    ),
                }
            }, {
                error_fn()
            })
        }, {
            error_fn()
        })
    }

    pub fn desc() -> ArithmeticDescription {
        ArithmeticDescription::creator(Box::new(Self::try_create_func)).features(
            FunctionFeatures::default()
                .deterministic()
                .monotonicity()
                .num_arguments(2),
        )
    }

    pub fn get_monotonicity(args: &[Monotonicity2]) -> Result<Monotonicity2> {
        // For expression f(x) - g(x), only when both f(x) and g(x) are monotonic and have
        // opposite 'is_positive' can we get a monotonic expression.
        let f_x = &args[0];
        let g_x = &args[1];

        // case of 12 - g(x)
        if f_x.is_constant {
            return Ok(Monotonicity2::create(
                g_x.is_monotonic || g_x.is_constant,
                !g_x.is_positive,
                g_x.is_constant,
            ));
        }

        // case of f(x) - 12
        if g_x.is_constant {
            return Ok(Monotonicity2::create(
                f_x.is_monotonic,
                f_x.is_positive,
                f_x.is_constant,
            ));
        }

        // if either one is non-monotonic, return non-monotonic
        if !f_x.is_monotonic || !g_x.is_monotonic {
            return Ok(Monotonicity2::default());
        }

        // when both are monotonic, and have same 'is_positive', we can't determine the monotonicity
        if f_x.is_positive == g_x.is_positive {
            return Ok(Monotonicity2::default());
        }

        Ok(Monotonicity2::create(true, f_x.is_positive, false))
    }
}

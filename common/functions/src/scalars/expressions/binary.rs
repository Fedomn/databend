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

use std::marker::PhantomData;

use common_datavalues2::prelude::*;
use common_exception::Result;

pub trait ScalarBinaryFunction<L: Scalar, R: Scalar, O: Scalar> {
    fn eval(&self, l: L::RefType<'_>, r: R::RefType<'_>) -> O;
}

/// Blanket implementation for all binary expression functions
impl<L: Scalar, R: Scalar, O: Scalar, F> ScalarBinaryFunction<L, R, O> for F
where F: Fn(L::RefType<'_>, R::RefType<'_>) -> O
{
    fn eval(&self, i1: L::RefType<'_>, i2: R::RefType<'_>) -> O {
        self(i1, i2)
    }
}

/// A common struct to caculate binary expression scalar op.
#[derive(Clone)]
pub struct ScalarBinaryExpression<L: Scalar, R: Scalar, O: Scalar, F> {
    func: F,
    _phantom: PhantomData<(L, R, O)>,
}

impl<L: Scalar, R: Scalar, O: Scalar, F> ScalarBinaryExpression<L, R, O, F>
where F: ScalarBinaryFunction<L, R, O>
{
    /// Create a binary expression from generic columns  and a lambda function.
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }

    /// Evaluate the expression with the given array.
    pub fn eval(&self, l: &ColumnRef, r: &ColumnRef) -> Result<<O as Scalar>::ColumnType> {
        debug_assert!(
            l.len() == r.len(),
            "Size of columns must match to apply binary expression"
        );

        match (l.is_const(), r.is_const()) {
            (false, true) => {
                let left: &<L as Scalar>::ColumnType = unsafe { Series::static_cast(l) };
                let right = R::try_create_viewer(r)?;

                let b = right.value_at(0);
                let it = left.scalar_iter().map(|a| self.func.eval(a, b));
                Ok(<O as Scalar>::ColumnType::from_owned_iterator(it))
            }

            (false, false) => {
                let left: &<L as Scalar>::ColumnType = unsafe { Series::static_cast(l) };
                let right: &<R as Scalar>::ColumnType = unsafe { Series::static_cast(r) };

                let it = left
                    .scalar_iter()
                    .zip(right.scalar_iter())
                    .map(|(a, b)| self.func.eval(a, b));
                Ok(<O as Scalar>::ColumnType::from_owned_iterator(it))
            }

            (true, false) => {
                let left = L::try_create_viewer(l)?;
                let a = left.value_at(0);

                let right: &<R as Scalar>::ColumnType = unsafe { Series::static_cast(r) };
                let it = right.scalar_iter().map(|b| self.func.eval(a, b));
                Ok(<O as Scalar>::ColumnType::from_owned_iterator(it))
            }

            // True True ?
            (true, true) => {
                let left = L::try_create_viewer(l)?;
                let right = R::try_create_viewer(r)?;

                let it = left
                    .iter()
                    .zip(right.iter())
                    .map(|(a, b)| self.func.eval(a, b));
                Ok(<O as Scalar>::ColumnType::from_owned_iterator(it))
            }
        }
    }
}

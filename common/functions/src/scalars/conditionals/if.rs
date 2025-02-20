// Copyright 2022 Datafuse Labs.
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

use common_datavalues2::prelude::*;
use common_datavalues2::remove_nullable;
use common_datavalues2::type_coercion::aggregate_types;
use common_datavalues2::with_match_scalar_type;
use common_exception::Result;

use crate::scalars::cast_column_field;
use crate::scalars::function_factory::FunctionFeatures;
use crate::scalars::Function2;
use crate::scalars::Function2Description;

#[derive(Clone, Debug)]
pub struct IfFunction {
    display_name: String,
}

impl IfFunction {
    pub fn try_create(display_name: &str) -> Result<Box<dyn Function2>> {
        Ok(Box::new(IfFunction {
            display_name: display_name.to_string(),
        }))
    }

    pub fn desc() -> Function2Description {
        let mut features = FunctionFeatures::default().num_arguments(3);
        features = features.deterministic();
        Function2Description::creator(Box::new(Self::try_create)).features(features)
    }
}

impl Function2 for IfFunction {
    fn name(&self) -> &str {
        "IfFunction"
    }

    fn return_type(&self, args: &[&DataTypePtr]) -> Result<DataTypePtr> {
        let dts = vec![args[1].clone(), args[2].clone()];
        let least_supertype = aggregate_types(dts.as_slice())?;

        Ok(least_supertype)
    }

    fn eval(&self, columns: &ColumnsWithField, _input_rows: usize) -> Result<ColumnRef> {
        // if predicate / lhs / rhs is nullable, then it will return NullableColumn or Non-NullableColumn
        let mut nullable = false;

        let predicate: ColumnRef;
        if columns[0].data_type().is_nullable() {
            nullable = true;
            let boolean_dt: DataTypePtr = Arc::new(NullableType::create(BooleanType::arc()));
            predicate = cast_column_field(&columns[0], &boolean_dt)?;
        } else {
            let boolean_dt = BooleanType::arc();
            predicate = cast_column_field(&columns[0], &boolean_dt)?;
        }

        let lhs = &columns[1];
        let rhs = &columns[2];
        let dts = vec![lhs.data_type().clone(), rhs.data_type().clone()];

        let least_supertype = aggregate_types(dts.as_slice())?;
        if least_supertype.is_nullable() {
            nullable = true;
        }

        let lhs = cast_column_field(lhs, &least_supertype)?;
        let rhs = cast_column_field(rhs, &least_supertype)?;
        let type_id = remove_nullable(&lhs.data_type()).data_type_id();

        macro_rules! scalar_build {
            (
             $T:ident
        ) => {{
                let predicate_viewer = bool::try_create_viewer(&predicate)?;
                let lhs_viewer = $T::try_create_viewer(&lhs)?;
                let rhs_viewer = $T::try_create_viewer(&rhs)?;

                let size = lhs_viewer.size();

                if nullable {
                    let mut builder = NullableColumnBuilder::<$T>::with_capacity(size);

                    for ((predicate, l), (row, r)) in predicate_viewer
                        .iter()
                        .zip(lhs_viewer.iter())
                        .zip(rhs_viewer.iter().enumerate())
                    {
                        let valid = predicate_viewer.valid_at(row);
                        if predicate & valid {
                            builder.append(l, lhs_viewer.valid_at(row));
                        } else {
                            builder.append(r, rhs_viewer.valid_at(row));
                        };
                    }

                    Ok(builder.build(size))
                } else {
                    // let a = Series::check_get_scalar::<$T>(&lhs)?;
                    // let b = Series::check_get_scalar::<$T>(&rhs)?;
                    // let p = Series::check_get_scalar::<bool>(&predicate)?;

                    // let it = p
                    //     .scalar_iter()
                    //     .zip(a.scalar_iter())
                    //     .zip(b.scalar_iter())
                    //     .map(|((predicate, l), r)| if predicate { l } else { r });

                    let it = predicate_viewer
                        .iter()
                        .zip(lhs_viewer.iter())
                        .zip(rhs_viewer.iter())
                        .map(|((predicate, l), r)| if predicate { l } else { r });

                    let col = <$T as Scalar>::ColumnType::from_iterator(it);
                    Ok(col.arc())
                }
            }};
        }

        with_match_scalar_type!(type_id.to_physical_type(), |$T| {
            scalar_build!($T)
        }, {
            unimplemented!()
        })
    }

    fn passthrough_null(&self) -> bool {
        false
    }
}

impl std::fmt::Display for IfFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}()", self.display_name)
    }
}

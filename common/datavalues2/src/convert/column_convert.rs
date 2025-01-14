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
// limitations under the License.pub use data_type::*;

use std::sync::Arc;

use common_arrow::arrow::bitmap::MutableBitmap;
use common_datavalues::prelude::DataColumn as OldDataColumn;
use common_datavalues::prelude::DataColumnWithField as OldDataColumnWithField;

use crate::ColumnRef;
use crate::ColumnWithField;
use crate::ConstColumn;
use crate::DataField;
use crate::IntoColumn;
use crate::NullableColumn;
use crate::Series;

pub fn convert2_new_column(column: &OldDataColumnWithField) -> ColumnWithField {
    let result = convert2_new_column_nonull(column);
    if column.field().is_nullable() && result.column().data_type().can_inside_nullable() {
        let arrow_c = column.column().get_array_ref().unwrap();
        let bitmap = arrow_c.validity().cloned();

        let bitmap = if let Some(b) = bitmap {
            b
        } else {
            let mut b = MutableBitmap::with_capacity(arrow_c.len());
            b.extend_constant(arrow_c.len(), true);
            b.into()
        };

        let column = NullableColumn::new(result.column().clone(), bitmap);
        return ColumnWithField::new(Arc::new(column), result.field().clone());
    }

    result
}

fn convert2_new_column_nonull(column: &OldDataColumnWithField) -> ColumnWithField {
    let field = column.field().clone();
    let field: DataField = field.into();

    match column.column() {
        OldDataColumn::Array(array) => {
            let arrow_column = array.get_array_ref();
            ColumnWithField::new(arrow_column.into_column(), field)
        }
        OldDataColumn::Constant(value, size) => {
            let s = value.to_series_with_size(1).unwrap();
            let arrow_column = s.get_array_ref();
            let col = arrow_column.into_column();

            ColumnWithField::new(Arc::new(ConstColumn::new(col, *size)), field)
        }
    }
}

pub fn convert2_old_column(column: &ColumnRef) -> OldDataColumn {
    if column.is_const() {
        let c: &ConstColumn = unsafe { Series::static_cast(column) };
        let e = convert2_old_column(c.inner());
        let v = e.try_get(0).unwrap();
        return OldDataColumn::Constant(v, column.len());
    }

    let arrow_c = column.as_arrow_array();
    OldDataColumn::from(arrow_c)
}

pub fn convert2_old_column_with_field(column: &ColumnWithField) -> OldDataColumnWithField {
    let new_f = column.field().clone();
    let old_field = new_f.into();

    OldDataColumnWithField::new(convert2_old_column(column.column()), old_field)
}

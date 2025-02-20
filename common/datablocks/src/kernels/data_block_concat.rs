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
use common_exception::ErrorCode;
use common_exception::Result;

use crate::DataBlock;

impl DataBlock {
    pub fn concat_blocks(blocks: &[DataBlock]) -> Result<DataBlock> {
        if blocks.is_empty() {
            return Result::Err(ErrorCode::EmptyData("Can't concat empty blocks"));
        }

        let first_block = &blocks[0];
        for block in blocks.iter() {
            if block.schema().ne(first_block.schema()) {
                return Result::Err(ErrorCode::DataStructMissMatch("Schema not matched"));
            }
        }

        let mut concat_columns = Vec::with_capacity(first_block.num_columns());
        for (i, _f) in blocks[0].schema().fields().iter().enumerate() {
            let mut columns = Vec::with_capacity(blocks.len());
            for block in blocks.iter() {
                columns.push(block.column(i).clone());
            }

            concat_columns.push(Series::concat(&columns)?);
        }

        Ok(DataBlock::create(
            first_block.schema().clone(),
            concat_columns,
        ))
    }
}

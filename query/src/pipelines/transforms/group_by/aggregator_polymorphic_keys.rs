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

use bumpalo::Bump;
use common_datablocks::HashMethod;
use common_datablocks::HashMethodKeysU16;
use common_datablocks::HashMethodKeysU32;
use common_datablocks::HashMethodKeysU64;
use common_datablocks::HashMethodKeysU8;
use common_datablocks::HashMethodSerializer;
use common_datavalues2::prelude::*;

use crate::common::HashTable;
use crate::pipelines::transforms::group_by::aggregator_keys_builder::FixedKeysColumnBuilder;
use crate::pipelines::transforms::group_by::aggregator_keys_builder::KeysColumnBuilder;
use crate::pipelines::transforms::group_by::aggregator_keys_builder::SerializedKeysColumnBuilder;
use crate::pipelines::transforms::group_by::aggregator_state::LongerFixedKeysAggregatorState;
use crate::pipelines::transforms::group_by::aggregator_state::SerializedKeysAggregatorState;
use crate::pipelines::transforms::group_by::aggregator_state::ShortFixedKeysAggregatorState;
use crate::pipelines::transforms::group_by::AggregatorState;

// Provide functions for all HashMethod to help implement polymorphic group by key
//
// When we want to add new HashMethod, we need to add the following components
//     - HashMethod, more information in [HashMethod] trait
//     - AggregatorState, more information in [AggregatorState] trait
//     - KeysColumnBuilder, more information in [KeysColumnBuilder] trait
//     - PolymorphicKeysHelper, more information in following comments
//
// For example:
//
// use bumpalo::Bump;
// use databend_query::common::HashTable;
// use common_datablocks::HashMethodSerializer;
// use common_datavalues2::prelude::*;
// use databend_query::pipelines::transforms::group_by::PolymorphicKeysHelper;
// use databend_query::pipelines::transforms::group_by::aggregator_state::SerializedKeysAggregatorState;
// use databend_query::pipelines::transforms::group_by::aggregator_keys_builder::SerializedKeysColumnBuilder;
//
// impl PolymorphicKeysHelper<HashMethodSerializer> for HashMethodSerializer {
//     type State = SerializedKeysAggregatorState;
//     fn aggregate_state(&self) -> Self::State {
//         SerializedKeysAggregatorState {
//             keys_area: Bump::new(),
//             state_area: Bump::new(),
//             data_state_map: HashTable::create(),
//         }
//     }
//
//     type ColumnBuilder = SerializedKeysColumnBuilder;
//     fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
//         SerializedKeysColumnBuilder {
//             inner_builder: MutableStringColumn::with_capacity(capacity),
//         }
//     }
// }
//
pub trait PolymorphicKeysHelper<Method: HashMethod> {
    type State: AggregatorState<Method>;
    fn aggregate_state(&self) -> Self::State;

    type ColumnBuilder: KeysColumnBuilder<<Self::State as AggregatorState<Method>>::Key>;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder;
}

impl PolymorphicKeysHelper<HashMethodKeysU8> for HashMethodKeysU8 {
    type State = ShortFixedKeysAggregatorState<u8>;
    fn aggregate_state(&self) -> Self::State {
        Self::State::create(u8::MAX as usize)
    }

    type ColumnBuilder = FixedKeysColumnBuilder<u8>;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
        FixedKeysColumnBuilder::<u8> {
            inner_builder: MutablePrimitiveColumn::<u8>::with_capacity(capacity),
        }
    }
}

impl PolymorphicKeysHelper<HashMethodKeysU16> for HashMethodKeysU16 {
    type State = ShortFixedKeysAggregatorState<u16>;
    fn aggregate_state(&self) -> Self::State {
        Self::State::create(u16::MAX as usize)
    }

    type ColumnBuilder = FixedKeysColumnBuilder<u16>;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
        FixedKeysColumnBuilder::<u16> {
            inner_builder: MutablePrimitiveColumn::<u16>::with_capacity(capacity),
        }
    }
}

impl PolymorphicKeysHelper<HashMethodKeysU32> for HashMethodKeysU32 {
    type State = LongerFixedKeysAggregatorState<u32>;
    fn aggregate_state(&self) -> Self::State {
        LongerFixedKeysAggregatorState::<u32> {
            area: Bump::new(),
            data: HashTable::create(),
        }
    }

    type ColumnBuilder = FixedKeysColumnBuilder<u32>;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
        FixedKeysColumnBuilder::<u32> {
            inner_builder: MutablePrimitiveColumn::<u32>::with_capacity(capacity),
        }
    }
}

impl PolymorphicKeysHelper<HashMethodKeysU64> for HashMethodKeysU64 {
    type State = LongerFixedKeysAggregatorState<u64>;
    fn aggregate_state(&self) -> Self::State {
        LongerFixedKeysAggregatorState::<u64> {
            area: Bump::new(),
            data: HashTable::create(),
        }
    }

    type ColumnBuilder = FixedKeysColumnBuilder<u64>;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
        FixedKeysColumnBuilder::<u64> {
            inner_builder: MutablePrimitiveColumn::<u64>::with_capacity(capacity),
        }
    }
}

impl PolymorphicKeysHelper<HashMethodSerializer> for HashMethodSerializer {
    type State = SerializedKeysAggregatorState;
    fn aggregate_state(&self) -> Self::State {
        SerializedKeysAggregatorState {
            keys_area: Bump::new(),
            state_area: Bump::new(),
            data_state_map: HashTable::create(),
        }
    }

    type ColumnBuilder = SerializedKeysColumnBuilder;
    fn state_array_builder(&self, capacity: usize) -> Self::ColumnBuilder {
        SerializedKeysColumnBuilder {
            inner_builder: MutableStringColumn::with_capacity(capacity),
        }
    }
}

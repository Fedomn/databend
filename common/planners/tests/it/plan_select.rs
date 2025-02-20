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

use common_datavalues2::prelude::*;
use common_exception::Result;
use common_planners::*;

#[test]
fn test_select_wildcard_plan() -> Result<()> {
    use pretty_assertions::assert_eq;

    let schema = DataSchemaRefExt::create(vec![DataField::new("a", Vu8::to_data_type())]);
    let plan = PlanBuilder::create(schema).project(&[col("a")])?.build()?;
    let select = PlanNode::Select(SelectPlan {
        input: Arc::new(plan),
    });
    let expect = "Projection: a:String";

    let actual = format!("{:?}", select);
    assert_eq!(expect, actual);
    Ok(())
}

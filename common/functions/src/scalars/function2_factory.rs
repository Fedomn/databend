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

use std::collections::HashMap;
use std::sync::Arc;

use common_datavalues::prelude::DataField as OldDataField;
use common_datavalues::DataTypeAndNullable;
use common_datavalues2::DataField;
use common_datavalues2::DataTypePtr;
use common_exception::ErrorCode;
use common_exception::Result;
use once_cell::sync::Lazy;

use super::function2::Function2;
use super::function_factory::FunctionFeatures;
use super::ArithmeticFunction;
use super::ComparisonFunction;
use super::ConditionalFunction;
use super::Function1Convertor;
use super::Function2Adapter;
use super::FunctionFactory;
use super::HashesFunction;
use super::LogicFunction;
use super::MathsFunction;
use super::NullableFunction;
use super::OtherFunction;
use super::StringFunction;
use super::ToCastFunction;
use super::TupleClassFunction;
use super::UdfFunction;
use crate::scalars::DateFunction;
use crate::scalars::UUIDFunction;

pub type Factory2Creator = Box<dyn Fn(&str) -> Result<Box<dyn Function2>> + Send + Sync>;

// Temporary adaptation for arithmetic.
pub type ArithmeticCreator =
    Box<dyn Fn(&str, &[&DataTypePtr]) -> Result<Box<dyn Function2>> + Send + Sync>;

pub struct Function2Description {
    features: FunctionFeatures,
    function_creator: Factory2Creator,
}

impl Function2Description {
    pub fn creator(creator: Factory2Creator) -> Function2Description {
        Function2Description {
            function_creator: creator,
            features: FunctionFeatures::default(),
        }
    }

    #[must_use]
    pub fn features(mut self, features: FunctionFeatures) -> Function2Description {
        self.features = features;
        self
    }
}

pub struct ArithmeticDescription {
    pub features: FunctionFeatures,
    pub arithmetic_creator: ArithmeticCreator,
}

impl ArithmeticDescription {
    pub fn creator(creator: ArithmeticCreator) -> ArithmeticDescription {
        ArithmeticDescription {
            arithmetic_creator: creator,
            features: FunctionFeatures::default(),
        }
    }

    #[must_use]
    pub fn features(mut self, features: FunctionFeatures) -> ArithmeticDescription {
        self.features = features;
        self
    }
}

pub struct Function2Factory {
    case_insensitive_desc: HashMap<String, Function2Description>,
    case_insensitive_arithmetic_desc: HashMap<String, ArithmeticDescription>,
}

static FUNCTION2_FACTORY: Lazy<Arc<Function2Factory>> = Lazy::new(|| {
    let mut function_factory = Function2Factory::create();

    ArithmeticFunction::register(&mut function_factory);
    ToCastFunction::register(&mut function_factory);
    TupleClassFunction::register(&mut function_factory);
    ComparisonFunction::register(&mut function_factory);
    UdfFunction::register2(&mut function_factory);
    StringFunction::register2(&mut function_factory);
    HashesFunction::register2(&mut function_factory);
    ConditionalFunction::register(&mut function_factory);
    LogicFunction::register(&mut function_factory);
    NullableFunction::register(&mut function_factory);
    DateFunction::register2(&mut function_factory);
    OtherFunction::register(&mut function_factory);
    UUIDFunction::register2(&mut function_factory);
    MathsFunction::register2(&mut function_factory);

    Arc::new(function_factory)
});

impl Function2Factory {
    pub(in crate::scalars::function2_factory) fn create() -> Function2Factory {
        Function2Factory {
            case_insensitive_desc: Default::default(),
            case_insensitive_arithmetic_desc: Default::default(),
        }
    }

    pub fn instance() -> &'static Function2Factory {
        FUNCTION2_FACTORY.as_ref()
    }

    pub fn register(&mut self, name: &str, desc: Function2Description) {
        let case_insensitive_desc = &mut self.case_insensitive_desc;
        case_insensitive_desc.insert(name.to_lowercase(), desc);
    }

    pub fn register_arithmetic(&mut self, name: &str, desc: ArithmeticDescription) {
        let case_insensitive_arithmetic_desc = &mut self.case_insensitive_arithmetic_desc;
        case_insensitive_arithmetic_desc.insert(name.to_lowercase(), desc);
    }

    pub fn get(&self, name: impl AsRef<str>, args: &[&DataTypePtr]) -> Result<Box<dyn Function2>> {
        let origin_name = name.as_ref();
        let lowercase_name = origin_name.to_lowercase();

        // TODO: remove the codes
        {
            let fs = args
                .iter()
                .map(|c| DataField::new("xx", (*c).clone()))
                .collect::<Vec<_>>();

            let mut types = vec![];
            let fs: Vec<OldDataField> = fs.iter().map(|f| f.clone().into()).collect();
            for t in fs.iter() {
                types.push(DataTypeAndNullable::create(t.data_type(), t.is_nullable()));
            }

            let factory = FunctionFactory::instance();
            if let Ok(v) = factory.get(origin_name, &types) {
                let adapter = Function1Convertor::create(v);
                return Ok(adapter);
            }
        }

        let inner = match self.case_insensitive_desc.get(&lowercase_name) {
            // TODO(Winter): we should write similar function names into error message if function name is not found.
            None => match self.case_insensitive_arithmetic_desc.get(&lowercase_name) {
                None => Err(ErrorCode::UnknownFunction(format!(
                    "Unsupported Function: {}",
                    origin_name
                ))),
                Some(desc) => (desc.arithmetic_creator)(origin_name, args),
            },
            Some(desc) => (desc.function_creator)(origin_name),
        }?;

        Ok(Function2Adapter::create(inner))
    }

    pub fn get_features(&self, name: impl AsRef<str>) -> Result<FunctionFeatures> {
        let origin_name = name.as_ref();
        let lowercase_name = origin_name.to_lowercase();

        let factory = FunctionFactory::instance();
        if let Ok(v) = factory.get_features(origin_name) {
            return Ok(v);
        }

        match self.case_insensitive_desc.get(&lowercase_name) {
            // TODO(Winter): we should write similar function names into error message if function name is not found.
            None => match self.case_insensitive_arithmetic_desc.get(&lowercase_name) {
                None => Err(ErrorCode::UnknownFunction(format!(
                    "Unsupported Function: {}",
                    origin_name
                ))),
                Some(desc) => Ok(desc.features.clone()),
            },
            Some(desc) => Ok(desc.features.clone()),
        }
    }

    pub fn check(&self, name: impl AsRef<str>) -> bool {
        let origin_name = name.as_ref();
        let lowercase_name = origin_name.to_lowercase();

        let function_factory = FunctionFactory::instance();
        if function_factory.check(name) {
            return true;
        }

        if self.case_insensitive_desc.contains_key(&lowercase_name) {
            return true;
        }
        self.case_insensitive_arithmetic_desc
            .contains_key(&lowercase_name)
    }

    pub fn registered_names(&self) -> Vec<String> {
        let function_factory = FunctionFactory::instance();
        let func_names = function_factory.registered_names();

        self.case_insensitive_desc
            .keys()
            .chain(self.case_insensitive_arithmetic_desc.keys())
            .chain(func_names.iter())
            .cloned()
            .collect::<Vec<_>>()
    }
}

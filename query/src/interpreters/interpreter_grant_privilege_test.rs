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

use common_base::tokio;
use common_exception::Result;
use common_management::UserInfo;
use common_meta_types::AuthType;
use common_meta_types::UserPrivilege;
use common_planners::*;
use futures::stream::StreamExt;
use pretty_assertions::assert_eq;

use crate::interpreters::*;
use crate::sql::PlanParser;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_grant_privilege_interpreter() -> Result<()> {
    common_tracing::init_default_ut_tracing();

    let ctx = crate::tests::try_create_context()?;
    let name = "test";
    let hostname = "localhost";
    let password = "test";
    let user_info = UserInfo::new(
        name.to_string(),
        hostname.to_string(),
        Vec::from(password),
        AuthType::PlainText,
    );
    assert_eq!(user_info.privileges, UserPrivilege::empty());
    let user_mgr = ctx.get_sessions_manager().get_user_manager();
    user_mgr.add_user(user_info).await?;
    if let PlanNode::GrantPrivilege(plan) = PlanParser::create(ctx.clone())
        .build_from_sql(format!("GRANT ALL ON * TO '{}'@'{}'", name, hostname).as_str())?
    {
        let executor = GrantPrivilegeInterpreter::try_create(ctx, plan.clone())?;
        assert_eq!(executor.name(), "GrantPrivilegeInterpreter");
        let mut stream = executor.execute(None).await?;
        while let Some(_block) = stream.next().await {}
        let new_user = user_mgr.get_user(name, hostname).await?;
        assert_eq!(new_user.privileges, {
            let mut privileges = UserPrivilege::empty();
            privileges.set_all_privileges();
            privileges
        })
    } else {
        panic!()
    }

    Ok(())
}
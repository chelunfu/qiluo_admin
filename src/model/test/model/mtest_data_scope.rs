pub use super::args::atest_data_scope::*;
pub use super::entity::test_data_scope::{self, ActiveModel, Model as TestDataScopeModel};
use crate::model::prelude::*;
use crate::model::sys::entity::sys_dept;
use crate::service::data_scope::{check_record_scope, DataScopeContext, RecordOp, RecordScope};
impl TestDataScopeModel {
    pub async fn list(
        arg: PageParams,
        search: TestDataScopeSearch,
        userinfo: UserInfo,
    ) -> Result<ListData<TestDataScopeResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = test_data_scope::Entity::find();
        rmodel = rmodel.filter(test_data_scope::Column::DeletedAt.is_null());

        if let Some(title) = search.title {
            rmodel = rmodel.filter(test_data_scope::Column::Title.contains(title));
        }
        if let Some(content) = search.content {
            rmodel = rmodel.filter(test_data_scope::Column::Content.contains(content));
        }

        let scope = DataScopeContext::from_user_id(db, userinfo.uid).await?;

        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to(test_data_scope::Entity)
                .from(sys_dept::Column::DeptId)
                .to(test_data_scope::Column::DeptId)
                .into(),
        );
        if let Some(cond) = scope.to_scope_condition(
            Some(test_data_scope::Column::DeptId),  // 部门字段
            Some(test_data_scope::Column::OwnerId), // 归属人字段
            userinfo.uid,
        ) {
            rmodel = rmodel.filter(cond);
        }
        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(test_data_scope::Column::UpdatedAt)
            .into_model::<TestDataScopeResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        let res = ListData {
            list,
            total,
            total_pages,
            page_num,
        };
        Ok(res)
    }

    pub async fn add(arg: TestDataScopeAdd, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();

        let rec_scope = check_record_scope(
            db,
            &userinfo,
            RecordOp::Create {
                req_dept_id: arg.dept_id,
                req_owner_id: arg.owner_id,
            },
        )
        .await?;

        let model = test_data_scope::ActiveModel {
            id: Set(id),
            title: Set(arg.title),
            content: Set(arg.content),
            dept_id: Set(rec_scope.dept_id),
            owner_id: Set(rec_scope.owner_id),
            status: Set(arg.status),
            created_at: Set(now),
            updated_at: Set(now),
            deleted_at: Set(None),
        };
        model.insert(db).await?;
        Ok(format!("Successfully added record with id: {}", id))
    }

    pub async fn edit(arg: TestDataScopeEdit, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let model = test_data_scope::Entity::find()
            .filter(test_data_scope::Column::Id.eq(arg.id))
            .one(db)
            .await?;

        let Some(model) = model else {
            return Err("Record not found".into());
        };
        let old_scope = RecordScope {
            dept_id: model.dept_id,
            owner_id: model.owner_id,
        };

       
        let rec_scope = check_record_scope(
            db,
            &userinfo,
            RecordOp::Update {
                old: old_scope,
                new_dept_id: arg.dept_id, 
                new_owner_id: arg.owner_id,
            },
        )
        .await?;

        let mut active_model: test_data_scope::ActiveModel = model.into();
        active_model.title = Set(arg.title);
        active_model.content = Set(arg.content);
        active_model.status = Set(arg.status);
        active_model.dept_id = Set(rec_scope.dept_id);
        active_model.owner_id = Set(rec_scope.owner_id);
        active_model.updated_at = Set(Local::now().naive_local());
        let _ = active_model.update(db).await?;

        Ok("Successfully updated record".to_string())
    }

    pub async fn del(arg: TestDataScopeDel, userinfo: UserInfo) -> Result<String> {
        let db = DB().await;
        let id = arg.id;
        let model = test_data_scope::Entity::find_by_id(id).one(db).await?;

        let Some(model) = model else {
            return Err("Record not found".into());
        };

        let old_scope = RecordScope {
            dept_id: model.dept_id,
            owner_id: model.owner_id,
        };
 
        let _ = check_record_scope(db, &userinfo, RecordOp::Delete { old: old_scope }).await?;

        test_data_scope::Entity::delete_by_id(id).exec(db).await?;
        Ok("Successfully deleted record".to_string())
    }
}

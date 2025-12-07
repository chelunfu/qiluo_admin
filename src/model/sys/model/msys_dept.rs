use sea_orm::Statement;

pub use super::args::asys_dept::*;
pub use super::entity::sys_dept::{self, ActiveModel, Model as SysDeptModel};
use crate::model::prelude::*;
use crate::service::data_scope::DataScopeContext;
impl SysDeptModel {
    pub async fn list_tree(
        arg: PageParams,
        search: SysDeptSearch,
        userinfo: UserInfo,
    ) -> Result<ListData<DeptResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = sys_dept::Entity::find();
        rmodel = rmodel.filter(sys_dept::Column::DeletedAt.is_null());
        if let Some(dept_name) = search.dept_name {
            rmodel = rmodel.filter(sys_dept::Column::DeptName.contains(dept_name));
        }

        let scope = DataScopeContext::from_user_id(db, userinfo.uid).await?;

        if let Some(cond) = scope.to_dept_condition_for_list(userinfo.did) {
            rmodel = rmodel.filter(cond);
        }
        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(sys_dept::Column::Order)
            .into_model::<DeptResp>()
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

    pub async fn edit(arg: SysDeptEdit) -> Result<(String, bool)> {
        let db = DB().await;
        let rmodel = sys_dept::Entity::find_by_id(arg.dept_id).one(db).await?;
        let model = if let Some(model) = rmodel {
            model
        } else {
            return Err("Model Failed".into());
        };
        let update_tree = model.parent_id != arg.parent_id;
        let mut amodel: sys_dept::ActiveModel = model.into();
        amodel.parent_id = Set(arg.parent_id);
        amodel.dept_name = Set(arg.dept_name);
        amodel.dept_category = Set(arg.dept_category);
        amodel.order = Set(arg.order);
        amodel.leader = Set(arg.leader);
        amodel.phone = Set(arg.phone);
        amodel.email = Set(arg.email);
        amodel.status = Set(arg.status);
        amodel.remark = Set(arg.remark);
        amodel.update(db).await?;
        Ok(("Success".to_string(), update_tree))
    }

    pub async fn add(arg: SysDeptAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let amode = sys_dept::ActiveModel {
            dept_id: Set(id),
            parent_id: Set(arg.parent_id),
            dept_name: Set(arg.dept_name),
            dept_category: Set(arg.dept_category),
            order: Set(arg.order),
            leader: Set(arg.leader),
            phone: Set(arg.phone),
            email: Set(arg.email),
            status: Set(arg.status),
            remark: Set(arg.remark),
            ..Default::default()
        };
        amode.insert(db).await?;
        Ok("Success".to_string())
    }

    pub async fn delete(dept_id: i64) -> Result<String> {
        let db = DB().await;
        let rmodel = sys_dept::Entity::find()
            .filter(sys_dept::Column::ParentId.eq(dept_id))
            .count(db)
            .await?;
        if rmodel > 0 {
            return Err("Failed,There are sub-departments".into());
        }

        sys_dept::Entity::delete_by_id(dept_id).exec(db).await?;
        Ok("Success".to_string())
    }

    pub async fn get_all(userinfo: UserInfo) -> Result<Vec<DeptResp>> {
        let db = DB().await;
        let mut rmodel = sys_dept::Entity::find();
        rmodel = rmodel.filter(sys_dept::Column::DeletedAt.is_null());
        let scope = DataScopeContext::from_user_id(db, userinfo.uid).await?;
        if let Some(cond) = scope.to_dept_condition_for_list(userinfo.did) {
            rmodel = rmodel.filter(cond);
        }
        let modes = rmodel.into_model::<DeptResp>().all(db).await?;
        Ok(modes)
    }

    pub async fn tree_data_all() -> Result<Vec<DeptData>> {
        let db = DB().await;
        let mut rmodel = sys_dept::Entity::find();
        rmodel = rmodel.filter(sys_dept::Column::DeletedAt.is_null());
        let modes = rmodel.into_model::<DeptData>().all(db).await?;
        Ok(modes)
    }
    pub async fn update_tree(args: Vec<DeptTreeData>) -> Result<String> {
        let db = DB().await;

        let mut query = String::from("UPDATE sys_dept SET ");

        let mut where_clause = String::new();
        let mut lft_cases = String::new();
        let mut rgt_cases = String::new();
        let mut depth_cases = String::new();

        for dept in &args {
            lft_cases.push_str(&format!(" WHEN {} THEN {} ", dept.dept_id, dept.lft));
            rgt_cases.push_str(&format!(" WHEN {} THEN {} ", dept.dept_id, dept.rgt));
            depth_cases.push_str(&format!(" WHEN {} THEN {} ", dept.dept_id, dept.depth));
            where_clause.push_str(&format!("{},", dept.dept_id));
        }

        query.push_str(&format!("lft = CASE dept_id {lft_cases} END, "));
        query.push_str(&format!("rgt = CASE dept_id {rgt_cases} END, "));
        query.push_str(&format!("depth = CASE dept_id {depth_cases} END"));

        if where_clause.ends_with(',') {
            where_clause.pop();
        }

        query.push_str(&format!(" WHERE dept_id IN ({where_clause})"));

        let stmt: Statement = Statement::from_string(DbBackend::MySql, query.as_str());

        db.execute(stmt).await?;

        Ok("Success".to_string())
    }
}

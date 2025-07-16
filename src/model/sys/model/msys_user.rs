pub use super::args::asys_user::*;
pub use super::entity::{
    sys_role,
    sys_user::{self, ActiveModel, Model as SysUserModel},
    sys_user_role::{self, Model as SysUserRoleModel},
};
use crate::model::prelude::*;
use crate::model::sys::entity::sys_dept;

impl SysUserModel {
    pub async fn list(arg: PageParams, search: UserSearch) -> Result<ListData<SysUserRes>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_size = arg.page_size.unwrap_or(10);
        let db = DB().await;

        let dept = sys_dept::Entity::find()
            .filter(sys_dept::Column::DeptId.eq(search.dept_id))
            .one(db)
            .await?;

        let mut rmodel = sys_user::Entity::find();

        let mut cond = Condition::all(); //.add(sys_user::Column::DeptId.eq(search.dept_id));

        cond = cond.add(sys_user::Column::DeletedAt.is_null());

        if let Some(sa) = search.user_name {
            cond = cond.add(sys_user::Column::UserName.contains(sa));
        }
        if let Some(sa) = search.phonenumber {
            cond = cond.add(sys_user::Column::Phonenumber.contains(sa));
        }
        if let Some(sa) = search.email {
            cond = cond.add(sys_user::Column::Email.contains(sa));
        }
        if let Some(sa) = search.status {
            cond = cond.add(sys_user::Column::Status.eq(sa));
        }
        rmodel = rmodel.filter(cond);
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_dept::Entity::belongs_to(sys_user::Entity)
                .from(sys_dept::Column::DeptId)
                .to(sys_user::Column::DeptId)
                .into(),
        );
        rmodel = rmodel.filter(
            Condition::all()
                .add(sys_dept::Column::Lft.gte(dept.clone().unwrap().lft))
                .add(sys_dept::Column::Rgt.lte(dept.unwrap().rgt)),
        );
        rmodel = rmodel.join_rev(
            JoinType::LeftJoin,
            sys_role::Entity::belongs_to(sys_user::Entity)
                .from(sys_role::Column::RoleId)
                .to(sys_user::Column::RoleId)
                .into(),
        );
        rmodel = rmodel.columns([sys_role::Column::RoleName]);
        rmodel = rmodel.column_as(sys_role::Column::RoleId, "ARoleId");

        let total = rmodel.clone().count(db).await.unwrap();
        let paginator = rmodel
            .order_by_asc(sys_user::Column::CreatedAt)
            .into_model::<SysUserRes>()
            .paginate(db, page_size);
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

    pub async fn add(arg: SysUserAdd) -> Result<SysUserModel> {
        let db = DB().await;
        let id: i64 = GID().await;
        let amodel = sys_user::ActiveModel {
            id: Set(id),
            dept_id: Set(arg.dept_id),
            role_id: Set(arg.role_id),
            avatar: Set(arg.avatar),
            user_name: Set(arg.user_name),
            nick_name: Set(arg.nick_name),
            phonenumber: Set(arg.phonenumber),
            email: Set(arg.email),
            password: Set(arg.password),
            sex: Set(arg.sex),
            status: Set(arg.status),
            remark: Set(arg.remark),
            ..Default::default()
        };
        Ok(amodel.insert(db).await?)
    }

    pub async fn edit(arg: SysUserEdit) -> Result<SysUserModel> {
        let db = DB().await;
        let rmodel = sys_user::Entity::find_by_id(arg.id).one(db).await?;
        let model = match rmodel {
            Some(model) => model,
            None => {
                return Err("Model Failed".into());
            }
        };
        let mut amodel: sys_user::ActiveModel = model.into();
        amodel.dept_id = Set(arg.dept_id);
        amodel.role_id = Set(arg.role_id);
        amodel.user_name = Set(arg.user_name);
        amodel.nick_name = Set(arg.nick_name);
        amodel.phonenumber = Set(arg.phonenumber);
        amodel.email = Set(arg.email);
        amodel.sex = Set(arg.sex);
        amodel.status = Set(arg.status);
        amodel.remark = Set(arg.remark);
        let umodel = amodel.update(db).await?;
        Ok(umodel)
    }

    pub async fn delete(ids: Vec<i64>) -> Result<String> {
        let db = DB().await;
        for id in ids {
            let rmodel = sys_user::Entity::find_by_id(id).one(db).await?;
            if let Some(model) = rmodel {
                let mut amodel: sys_user::ActiveModel = model.into();
                amodel.deleted_at = Set(Some(Local::now().naive_local()));
                let _ = amodel.update(db).await?;
            }
        }

        Ok("Success".into())
    }

    pub async fn find_by_username_or_email(username: &str, email: &str) -> Option<SysUserModel> {
        let db = DB().await;
        sys_user::Entity::find()
            .filter(
                Condition::any()
                    .add(sys_user::Column::UserName.eq(username))
                    .add(sys_user::Column::Email.eq(email)),
            )
            .one(db)
            .await
            .unwrap_or_default()
    }

    pub async fn find_by_username(username: &str) -> Option<SysUserModel> {
        let db = DB().await;
        sys_user::Entity::find()
            .filter(sys_user::Column::UserName.eq(username))
            .one(db)
            .await
            .unwrap_or_default()
    }

    pub async fn find_by_email(email: &str) -> Option<SysUserModel> {
        let db = DB().await;
        sys_user::Entity::find()
            .filter(sys_user::Column::Email.eq(email))
            .one(db)
            .await
            .unwrap_or_default()
    }
    pub async fn find_by_id(id: i64) -> Option<SysUserModel> {
        let db = DB().await;
        sys_user::Entity::find_by_id(id)
            .one(db)
            .await
            .unwrap_or_default()
    }

    pub async fn update_role_or_dept(uid: i64, arg: DeptAndRole) -> Result<String> {
        let db = DB().await;
        let ruser = sys_user::Entity::find_by_id(uid).one(db).await?;

        let user = if let Some(user) = ruser {
            user
        } else {
            return Err("User Not Found".into());
        };
        let mut amodel: sys_user::ActiveModel = user.into();

        amodel.role_id = Set(arg.rid);

        amodel.dept_id = Set(arg.did);

        amodel.update(db).await?;
        Ok("Success".into())
    }

    pub async fn update_avatar(id: i64, avatar: String) -> Result<String> {
        let db = DB().await;
        sys_user::Entity::update_many()
            .col_expr(sys_user::Column::Avatar, Expr::value(avatar))
            .filter(sys_user::Column::Id.eq(id))
            .exec(db)
            .await?;
        Ok("Success".to_string())
    }

    pub async fn chagne_password(arg: ChangePasswordParams) -> Result<String> {
        let db = DB().await;
        let ouser = SysUserModel::find_by_id(arg.uid).await;
        let user = if let Some(user) = ouser {
            user
        } else {
            return Err("Old Password Error".into());
        };

        let mut user_active: sys_user::ActiveModel = user.into();
        let new_password = util::hash_password(&arg.new_password).unwrap();

        user_active.password = Set(new_password);
        user_active.update(db).await?;
        Ok("Success".into())
    }
    pub async fn reset_password(uid: i64, arg: ResetPasswordParams) -> Result<String> {
        let db = DB().await;
        let ouser = SysUserModel::find_by_id(uid).await;
        let user = if let Some(user) = ouser {
            user
        } else {
            return Err("Old Password Error".into());
        };
        if !util::verify_password(&arg.old_password, user.password.as_str()) {
            return Err("Password Error".into());
        }
        let mut user_active: sys_user::ActiveModel = user.into();
        let new_password = util::hash_password(&arg.new_password).unwrap();

        user_active.password = Set(new_password);
        user_active.update(db).await?;
        Ok("Success".into())
    }

    pub async fn role() -> Result<SysUserAndRole> {
        let db = DB().await;
        let mut rmode = sys_user::Entity::find();
        rmode = rmode.join_rev(
            JoinType::LeftJoin,
            sys_user_role::Entity::belongs_to(sys_user::Entity)
                .from(sys_user_role::Column::UserId)
                .to(sys_user::Column::Id)
                .into(),
        );
        rmode = rmode.join_rev(
            JoinType::LeftJoin,
            sys_role::Entity::belongs_to(sys_user_role::Entity)
                .from(sys_role::Column::RoleId)
                .to(sys_user_role::Column::RoleId)
                .into(),
        );
        rmode = rmode.columns([
            sys_user::Column::Id,
            sys_user::Column::TenantId,
            sys_user::Column::DeptId,
            sys_user::Column::UserName,
            sys_user::Column::NickName,
            sys_user::Column::UserType,
            sys_user::Column::Email,
            sys_user::Column::Phonenumber,
            sys_user::Column::Sex,
            sys_user::Column::Avatar,
            sys_user::Column::Status,
            sys_user::Column::Remark,
        ]);
        rmode = rmode.columns([
            sys_role::Column::RoleId,
            sys_role::Column::RoleKey,
            sys_role::Column::RoleName,
            sys_role::Column::Order,
            sys_role::Column::DataScope,
        ]);
        rmode = rmode.columns([sys_user_role::Column::UserId, sys_user_role::Column::RoleId]);
        rmode
            .into_model::<SysUserAndRole>()
            .one(db)
            .await?
            .ok_or("Not Found".into())
    }
}

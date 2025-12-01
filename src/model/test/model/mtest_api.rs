pub use super::args::atest_api::*;
pub use super::entity::test_api::{self, ActiveModel, Model as TestApiModel};
use crate::model::prelude::*;

impl TestApiModel {
    pub async fn list(arg: PageParams, search: TestApiSearch) -> Result<ListData<TestApiResp>> {
        let page_num = arg.page_num.unwrap_or(1);
        let page_per_size = arg.page_size.unwrap_or(10);
        let db = DB().await;
        let mut rmodel = test_api::Entity::find();
        if let Some(name) = search.name {
            rmodel = rmodel.filter(test_api::Column::Name.contains(name));
        }
        if let Some(email) = search.email {
            rmodel = rmodel.filter(test_api::Column::Email.eq(email));
        }
        let total = rmodel.clone().count(db).await?;
        let paginator = rmodel
            .select_only()
            .column(test_api::Column::Id)
            .column(test_api::Column::Age)
            .column(test_api::Column::Name)
            .column(test_api::Column::Email)
            .column(test_api::Column::CreatedAt)
            .column(test_api::Column::UpdatedAt)
            .into_model::<TestApiResp>()
            .paginate(db, page_per_size);
        let total_pages = paginator.num_pages().await?;
        let list = paginator.fetch_page(page_num - 1).await?;
        Ok(ListData {
            list,
            total,
            total_pages,
            page_num,
        })
    }

    pub async fn add(arg: TestApiAdd) -> Result<String> {
        let db = DB().await;
        let id = GID().await;
        let now = Local::now().naive_local();
        let model = test_api::ActiveModel {
            id: Set(id),
            age: Set(arg.age),
            name: Set(arg.name),
            email: Set(arg.email),
            created_at: Set(now),
            updated_at: Set(now),
        };
        test_api::Entity::insert(model).exec(db).await?;
        Ok(format!("Successfully added record with id: {}", id))
    }

    pub async fn edit(arg: TestApiEdit) -> Result<String> {
        let db = DB().await;
        let model = test_api::Entity::find()
            .filter(test_api::Column::Id.eq(arg.id))
            .one(db)
            .await?;
        if let Some(model) = model {
            let mut active_model: test_api::ActiveModel = model.into();
            active_model.age = Set(arg.age);
            active_model.name = Set(arg.name);
            active_model.email = Set(arg.email);
            active_model.updated_at = Set(Local::now().naive_local());
            let _ = active_model.update(db).await?;

            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        }
    }
    pub async fn del(arg: TestApiDel) -> Result<String> {
        let db = DB().await;
        let result = test_api::Entity::delete_by_id(arg.id).exec(db).await?;
        if result.rows_affected > 0 {
            Ok("Success".to_string())
        } else {
            Err("delete failed".into())
        }
    }

    pub async fn db_index_test(arg: TestApiEdit) -> Result<String> {
        let dbread = DB_BY_INDEX(1).await;
         let model = test_api::Entity::find()
            .filter(test_api::Column::Id.eq(arg.id))
            .one(dbread)
            .await?;
         if let Some(model) = model {
            let mut active_model: test_api::ActiveModel = model.into();
            active_model.age = Set(arg.age);
            active_model.name = Set(arg.name);
            active_model.email = Set(arg.email);
            active_model.updated_at = Set(Local::now().naive_local());
            let dbwrite = DB_BY_INDEX(0).await;
            let _ = active_model.update(dbwrite).await?;

            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        } 
    }

     pub async fn db_name_test(arg: TestApiEdit) -> Result<String> {
        let dbread = DB_BY_NAME("").await;
         let model = test_api::Entity::find()
            .filter(test_api::Column::Id.eq(arg.id))
            .one(dbread)
            .await?;
         if let Some(model) = model {
            let mut active_model: test_api::ActiveModel = model.into();
            active_model.age = Set(arg.age);
            active_model.name = Set(arg.name);
            active_model.email = Set(arg.email);
            active_model.updated_at = Set(Local::now().naive_local());
            let dbwrite = DB_BY_NAME("").await;
            let _ = active_model.update(dbwrite).await?;

            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        } 
    }

    pub async fn db_read_write_test(arg: TestApiEdit) -> Result<String> {
        let dbread = DB_READ().await;
         let model = test_api::Entity::find()
            .filter(test_api::Column::Id.eq(arg.id))
            .one(dbread)
            .await?;
         if let Some(model) = model {
            let mut active_model: test_api::ActiveModel = model.into();
            active_model.age = Set(arg.age);
            active_model.name = Set(arg.name);
            active_model.email = Set(arg.email);
            active_model.updated_at = Set(Local::now().naive_local());
            let dbwrite = DB_WRITE().await;
            let _ = active_model.update(dbwrite).await?;

            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        } 
    }

     pub async fn db_auto_test(arg: TestApiEdit) -> Result<String> {
        let dbauto = DB_AUTO();
         let model = test_api::Entity::find()
            .filter(test_api::Column::Id.eq(arg.id))
            .one(dbauto)
            .await?;
         if let Some(model) = model {
            let mut active_model: test_api::ActiveModel = model.into();
            active_model.age = Set(arg.age);
            active_model.name = Set(arg.name);
            active_model.email = Set(arg.email);
            active_model.updated_at = Set(Local::now().naive_local()); 
            let _ = active_model.update(dbauto).await?;

            Ok("Successfully updated record".to_string())
        } else {
            Err("Record not found".into())
        } 
    }
}

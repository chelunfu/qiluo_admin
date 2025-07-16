use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = table_auto(SysJob::Table)
            .col(string(SysJob::JobId).primary_key().unique_key().char_len(32))
            .col(string(SysJob::TaskID).char_len(32))
            .col(integer(SysJob::TaskCount))
            .col(integer(SysJob::RunCount))
            .col(string(SysJob::JobName).char_len(32))
            .col(string_null(SysJob::JobParams).char_len(32))
            .col(string(SysJob::JobGroup).char_len(32))
            .col(string(SysJob::InvokeFunction).char_len(32))
            .col(string(SysJob::CronExpression).char_len(32))
            .col(string(SysJob::MisfirePolicy).char_len(32))
            .col(string_null(SysJob::Concurrent).char_len(32))
            .col(string(SysJob::Status).char_len(32))
            .col(string(SysJob::Remark).string_len(512))
            .col(timestamp_null(SysJob::LastTime).default(Expr::current_timestamp()))
            .col(timestamp_null(SysJob::NextTime).default(Expr::current_timestamp()))
            .col(timestamp_null(SysJob::EndTime).default(Expr::current_timestamp()))
            .to_owned();
        manager.create_table(table).await?;

        let table = table_auto(SysJobLog::Table)
            .col(string(SysJobLog::Id).primary_key().unique_key().char_len(32))
            .col(string(SysJobLog::JobId).char_len(32))
            .col(string(SysJobLog::LotId).char_len(32))
            .col(integer(SysJobLog::LotOrder))
            .col(string(SysJobLog::JobName).char_len(32))
            .col(string(SysJobLog::JobGroup).char_len(32))
            .col(string(SysJobLog::InvokeFunction).char_len(32))
            .col(string_null(SysJobLog::JobParams).char_len(32))
            .col(string_null(SysJobLog::JobMessage).string_len(512))
            .col(string(SysJobLog::Status).char_len(32))
            .col(string_null(SysJobLog::ExceptionInfo).string_len(512))
            .col(string_null(SysJobLog::IsOnce).char_len(32))
            .col(timestamp_null(SysJobLog::ElapsedTime).default(Expr::current_timestamp()))
            .to_owned();
        manager.create_table(table).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_table(Table::drop().table(SysJob::Table).to_owned())
            .await;
        let _ = manager
            .drop_table(Table::drop().table(SysJobLog::Table).to_owned())
            .await;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum SysJob {
    Table,
    JobId,
    TaskID,
    TaskCount,
    RunCount,
    JobName,
    JobParams,
    JobGroup,
    InvokeFunction,
    CronExpression,
    MisfirePolicy,
    Concurrent,
    Status,
    Remark,
    LastTime,
    NextTime,
    EndTime,
}

#[derive(DeriveIden)]
enum SysJobLog {
    Table,
    Id,
    JobId,
    LotId,
    LotOrder,
    JobName,
    JobGroup,
    InvokeFunction,
    JobParams,
    JobMessage,
    Status,
    ExceptionInfo,
    IsOnce,
    ElapsedTime,
}

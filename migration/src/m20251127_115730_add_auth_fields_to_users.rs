use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(string(User::PasswordHash))
                    .add_column(
                        ColumnDef::new(User::Role)
                            .string()
                            .not_null()
                            .default("user")
                    )
                    .add_column(timestamp(User::CreatedAt).default(Expr::current_timestamp()))
                    .add_column(
                        timestamp(User::UpdatedAt)
                            .default(Expr::current_timestamp())
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::PasswordHash)
                    .drop_column(User::Role)
                    .drop_column(User::CreatedAt)
                    .drop_column(User::UpdatedAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    #[sea_orm(iden = "users")]
    Table,
    PasswordHash,
    Role,
    CreatedAt,
    UpdatedAt,
}

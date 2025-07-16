# Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```



# Running Migrator CLI

- Generate a new migration file
    ```sh
    cargo run -- generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```


sea-orm-cli generate entity -o src/model/sys/entity --with-serde=both

sea-orm-cli migrate up
sea-orm-cli migrate down
sea-orm-cli migrate status
sea-orm-cli migrate generate role
sea-orm-cli migrate up -n 1
sea-orm-cli migrate down -n 1


/// All column types
///
/// | ColumnType            | MySQL data type   | PostgreSQL data type        | SQLite data type             |
/// |-----------------------|-------------------|-----------------------------|------------------------------|
/// | Char                  | char              | char                        | char                         |
/// | String                | varchar           | varchar                     | varchar                      |
/// | Text                  | text              | text                        | text                         |
/// | TinyInteger           | tinyint           | smallint                    | tinyint                      |
/// | SmallInteger          | smallint          | smallint                    | smallint                     |
/// | Integer               | int               | integer                     | integer                      |
/// | BigInteger            | bigint            | bigint                      | integer                      |
/// | TinyUnsigned          | tinyint unsigned  | smallint                    | tinyint                      |
/// | SmallUnsigned         | smallint unsigned | smallint                    | smallint                     |
/// | Unsigned              | int unsigned      | integer                     | integer                      |
/// | BigUnsigned           | bigint unsigned   | bigint                      | integer                      |
/// | Float                 | float             | real                        | float                        |
/// | Double                | double            | double precision            | double                       |
/// | Decimal               | decimal           | decimal                     | real                         |
/// | DateTime              | datetime          | timestamp without time zone | datetime_text                |
/// | Timestamp             | timestamp         | timestamp                   | timestamp_text               |
/// | TimestampWithTimeZone | timestamp         | timestamp with time zone    | timestamp_with_timezone_text |
/// | Time                  | time              | time                        | time_text                    |
/// | Date                  | date              | date                        | date_text                    |
/// | Year                  | year              | N/A                         | N/A                          |
/// | Interval              | N/A               | interval                    | N/A                          |
/// | Blob                  | blob              | bytea                       | blob                         |
/// | Binary                | binary            | bytea                       | blob                         |
/// | VarBinary             | varbinary         | bytea                       | varbinary_blob               |
/// | Bit                   | bit               | bit                         | N/A                          |
/// | VarBit                | bit               | varbit                      | N/A                          |
/// | Boolean               | bool              | bool                        | boolean                      |
/// | Money                 | decimal           | money                       | real_money                   |
/// | Json                  | json              | json                        | json_text                    |
/// | JsonBinary            | json              | jsonb                       | jsonb_text                   |
/// | Uuid                  | binary(16)        | uuid                        | uuid_text                    |
/// | Enum                  | ENUM(...)         | ENUM_NAME                   | enum_text                    |
/// | Array                 | N/A               | DATA_TYPE[]                 | N/A                          |
/// | Cidr                  | N/A               | cidr                        | N/A                          |
/// | Inet                  | N/A               | inet                        | N/A                          |
/// | MacAddr               | N/A               | macaddr                     | N/A                          |
/// | LTree                 | N/A               | ltree                       | N/A    
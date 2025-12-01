mod id;
mod registry;
mod router;
pub use id::generator_id as GID;
pub use registry::{
    db as DB, db_by_index as DB_BY_INDEX, db_by_name as DB_BY_NAME, db_read as DB_READ,
    db_write as DB_WRITE, with_read,
};
pub use router::db_auto as DB_AUTO;

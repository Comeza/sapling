pub const SQL_LOGIN_USER: &str = include_str!("auth/fetch_user.sql");
pub const SQL_REGISTER_USER: &str = include_str!("auth/register_user.sql");
pub const SQL_INSERT_SESSION: &str = include_str!("auth/insert_user_session.sql");

pub const SQL_FETCH_PRODUCT: &str = include_str!("fetch_product.sql");
pub const SQL_INSERT_PRODUCT: &str = include_str!("insert_product.sql");

pub const SQL_CREATE_AUTH_TABLES: &str = include_str!("setup/create_auth_tables.sql");
pub const SQL_CREATE_TABLES: &str = include_str!("setup/create_tables.sql");

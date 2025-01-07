//luma_db/lumadb/src/config.rs
//I'm sure as the project gets bigger, it could do with a lot of reworking, but
//For now, any hardcoded variables can go here for simplification
//filePath for all Tables, maybe make it changeable later
pub const DEFAULT_DIR: &str = "dbms_rust_project/data";
//defualt server ip and port
pub const DEFAULT_CONNECTION: &str = "127.0.0.1:7777";
//table name when none is supplied
pub const DEFAULT_TABLE: &str = "default_table_name";

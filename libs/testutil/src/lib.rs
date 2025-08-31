use sqlx::{Executor, PgPool};
use testcontainers_modules::{postgres, testcontainers::runners::AsyncRunner};
use tokio::sync::OnceCell;
use uuid::Uuid;

/// Static holder for the test database connection pool.
///
/// The pool will live for the entire lifetime of the test binary once initialized.
static TEST_DB: OnceCell<PgPool> = OnceCell::const_new();

/// Returns a reference to a test PostgreSQL database connection pool.
///
/// This function will:
/// 1. Spawn a new PostgreSQL database using `testcontainers` (with a randomized database name).
/// 2. Initialize the database (e.g., run migrations if needed).
/// 3. Store the resulting `PgPool` in a static `OnceCell`, so subsequent calls
///    return the same pool.
///
/// # Notes
/// - The database will persist for the lifetime of the test binary.
/// - Each test should ideally use transactions or cleanup code to avoid interfering
///   with other tests.
///
/// ```rust,ignore
/// #[tokio::test]
/// async fn test_stuff() {
///     let db = get_test_db();
/// }
pub async fn get_test_db_static() -> &'static PgPool {
    TEST_DB.get_or_init(get_test_db_instance).await
}

/// Sets up a fresh PostgreSQL database for testing.
///
/// This function does the following:
/// 1. Starts a new Postgres container using `testcontainers`.
/// 2. Connects to the default "postgres" database.
/// 3. Creates a new database with a unique name (UUID).
/// 4. Connects to the new test database and returns a `PgPool`.
///
/// # Returns
///
/// - `PgPool`: a connection pool to the newly created test database.
///
/// # Notes
///
/// - The container is dropped when the returned `Container` goes out of scope.
///   To keep the database alive for the duration of the test, store the container alongside the pool.
///
/// # Example
///
/// ```rust,ignore
/// #[tokio::test]
/// async fn test_stuff() {
///     let db = get_test_db_instance();
/// }
pub async fn get_test_db_instance() -> PgPool {
    let container = postgres::Postgres::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let admin_conn = &format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");
    let admin_db = PgPool::connect(admin_conn).await.unwrap();
    let db_name = format!("{}", Uuid::new_v4());

    // Create the test database
    admin_db
        .execute(format!(r#"CREATE DATABASE "{}""#, db_name).as_str())
        .await
        .unwrap();

    let conn_str = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/{}",
        container.get_host_port_ipv4(5432).await.unwrap(),
        db_name
    );

    let db: PgPool = PgPool::connect(&conn_str).await.unwrap();

    db
}

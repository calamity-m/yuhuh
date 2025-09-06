pub mod asserts;

use std::pin::Pin;

use sqlx::{Executor, PgPool};
use testcontainers_modules::{
    postgres::{self, Postgres},
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};
use tokio::sync::OnceCell;
use uuid::Uuid;

struct TestContainer {
    container: ContainerAsync<Postgres>,
}

impl Drop for TestContainer {
    fn drop(&mut self) {
        println!("cleaning up container")
    }
}

// Global container instance that persists across multiple test database creations.
// Uses OnceCell to ensure the container is only started once per test run,
// improving performance when running multiple tests that need databases.
static POSTGRES_CONTAINER: OnceCell<TestContainer> = OnceCell::const_new();

/// Creates a fresh PostgreSQL database instance for testing purposes.
///
/// This function manages the lifecycle of a PostgreSQL test container and creates
/// isolated database instances for each test. It follows these steps:
///
/// 1. **Container Management**:
///    - On first call: Spins up a new PostgreSQL container using testcontainers
///    - On subsequent calls: Reuses the existing container for efficiency
///
/// 2. **Database Creation**:
///    - Connects to the container's default "postgres" administrative database
///    - Creates a new database with a UUID-based name for complete test isolation
///    - Returns a connection pool to the newly created test database
///
/// # Returns
///
/// A `PgPool` connection pool configured for the newly created test database.
/// Each call returns a pool connected to a completely separate database instance.
///
/// # Test Isolation
///
/// Each test gets its own database with a unique name (UUID-based), ensuring:
/// - No interference between concurrent tests
/// - Clean state for each test run
/// - No cleanup required between tests
///
/// # Performance Notes
///
/// - The PostgreSQL container is shared across all tests in a single test run
/// - Container startup cost is only paid once per test execution
/// - Database creation is lightweight compared to container startup
///
/// # Example
///
/// ```rust,ignore
/// #[tokio::test]
/// async fn test_user_creation() {
///     let db = get_test_db_instance().await;
///     // Test code here - db is isolated from other tests
/// }
///
/// #[tokio::test]
/// async fn test_user_deletion() {
///     let db = get_test_db_instance().await;
///     // This test gets a completely separate database instance
/// }
/// ```
///
/// # Error Handling
///
/// Currently panics on database connection or creation failures. In a production
/// test suite, you might want to return `Result<PgPool, Error>` for better error handling.
pub async fn get_test_db_instance() -> PgPool {
    // Get or initialize the shared PostgreSQL container
    // This async closure only runs once thanks to OnceCell
    let container = POSTGRES_CONTAINER
        .get_or_init(|| async {
            TestContainer {
                container: Postgres::default()
                    .with_tag("18rc1-alpine3.22")
                    .start()
                    .await
                    .unwrap(),
            }
        })
        .await;

    // Get the host port that Docker has mapped to the container's PostgreSQL port (5432)
    let host_port = container.container.get_host_port_ipv4(5432).await.unwrap();

    // Connect to the administrative "postgres" database to create our test database
    let admin_conn = &format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");
    let admin_db = PgPool::connect(admin_conn).await.unwrap();

    // Generate a unique database name using UUID to ensure test isolation
    let db_name = format!("{}", Uuid::new_v4());

    // Create the test database using the administrative connection
    // Quotes around database name handle any special characters in the UUID
    admin_db
        .execute(format!(r#"CREATE DATABASE "{}""#, db_name).as_str())
        .await
        .unwrap();

    // Build connection string for the newly created test database
    let conn_str = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/{}",
        container.container.get_host_port_ipv4(5432).await.unwrap(),
        db_name
    );

    // Create and return a connection pool to the test database
    // This pool is what the test will use for all database operations
    let db: PgPool = PgPool::connect(&conn_str).await.unwrap();

    db
}

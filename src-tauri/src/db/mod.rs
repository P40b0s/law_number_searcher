pub use repository::{IRepository, Repository};

mod connection;
mod repository;
pub use connection::new_connection;
pub use repository::NumberDBO;
pub struct AppRepository<R: IRepository>
{
    pub repository: R
}

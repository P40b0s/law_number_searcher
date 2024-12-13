pub use repository::{IRepository, Repository};

mod connection;
mod repository;
pub use connection::new_connection;
pub struct AppRepository<R: IRepository>
{
    pub repository: R
}

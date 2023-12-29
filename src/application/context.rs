use crate::application::database::Database;

#[derive(Clone, Debug)]
pub struct AppContext {
    pub(crate) database: Database
}
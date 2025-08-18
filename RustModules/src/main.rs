pub mod auth_utils {
    
    pub fn auth() -> models::Status {
        println!("Authenticating!");
        models::Status::Secured

    }

    pub mod models {
        pub enum Status {
            Connected,
            Disconnected,
            Secured
        }
    }
}

pub mod database {

    

    pub fn connect() -> crate::auth_utils::models::Status {
        println!("Connected!");
        crate::auth_utils::models::Status::Connected
    }

    pub fn disconnect() -> crate::auth_utils::models::Status {
        println!("Disconnected!");
        crate::auth_utils::models::Status::Disconnected
    }
}

fn main() {
    use database;
    use auth_utils;

    let mut connection_status: auth_utils::models::Status;

    connection_status = database::connect();
    

    connection_status = auth_utils::auth();
    connection_status = database::disconnect();

}
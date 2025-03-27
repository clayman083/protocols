pub use self::protos::*;

mod protos {
    pub mod passport {
        tonic::include_proto!("passport");
    }
}


//!
//! this is a pure rust implimentation to get google cloud access token for servers with service account credentials.
//! 
//! ```
//! use gcp_access_token;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     match gcp_access_token::generator::init("./credentials.json".to_string()).await{
//!         Ok(v)=>{
//!             println!("token successfull : {}",v);
//!         },
//!         Err(_e)=>{
//!             println!("!!! failed-token : {:?}",_e);
//!         }
//!     }
//!
//! }
//! ```


mod io;
mod token;
pub mod generator;

// #[tokio::main]
// async fn main() {

//     match generator::init("../secret/daachi_firestore_key.json".to_string()).await{
//         Ok(v)=>{
//             println!("token successfull : {}",v);
//         },
//         Err(_e)=>{
//             println!("!!! failed-token : {:?}",_e);
//         }
//     }

// }



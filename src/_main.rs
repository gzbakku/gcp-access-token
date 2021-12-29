
mod io;
mod token;
pub mod generator;

#[tokio::main]
async fn main() {

    match generator::init("../secret/daachi_firestore_key.json".to_string()).await{
        Ok(v)=>{
            println!("token successfull : {}",v);
        },
        Err(_e)=>{
            println!("!!! failed-token : {:?}",_e);
        }
    }

}



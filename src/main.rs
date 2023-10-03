
mod io;
mod token;
pub mod generator;

#[tokio::main]
async fn main() {

    match generator::init(
        "../secret/firestore.json".to_string(),
        "https://www.googleapis.com/auth/firestore".to_string()
    ).await{
        Ok(v)=>{
            println!("token successfull : {}",v);
        },
        Err(_e)=>{
            println!("!!! failed-token : {:?}",_e);
        }
    }

}



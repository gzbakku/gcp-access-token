use json::{JsonValue,object,stringify};
use crate::io::read_json;
use std::time::{SystemTime, UNIX_EPOCH};
use json::number::Number;
use openssl::pkey::{PKey,Private};
use openssl::sign::{Signer};
// use base64::encode as Base64Encode;
use openssl::hash::MessageDigest;
// use data_encoding::BASE64URL as BASE64ENCODER;
use data_encoding::BASE64URL_NOPAD as BASE64ENCODER;

pub async fn init(file_path:String,scope:String)->Result<String,&'static str>{

    //--------------------------------------
    //process creds
    //--------------------------------------

    let creds:JsonValue;
    match read_json(file_path).await{
        Ok(v)=>{creds = v;},
        Err(_e)=>{
            println!("!!! failed-read_credss => {:?}",_e);
            return Err("faile-read_creds");
        }
    }

    let private_key_string:String;
    if !creds.has_key(&"private_key"){return Err("not_found-creds-private_key");}
    match creds["private_key"].as_str(){
        Some(v)=>{private_key_string = v.to_string();},
        None=>{return Err("not_found-creds-private_key");}
    }

    let private_key:PKey<Private>;
    match PKey::private_key_from_pem(&private_key_string.into_bytes()){
        Ok(k)=>{
            private_key = k;
        },
        Err(_e)=>{
            return Err("failed-invalid_key");
        }
    }

    let email:String;
    if !creds.has_key(&"client_email"){return Err("not_found-creds-client_email");}
    match creds["client_email"].as_str(){
        Some(v)=>{email = v.to_string();},
        None=>{return Err("not_found-creds-client_email");}
    }

    //--------------------------------------
    //build jwt
    //--------------------------------------

    //--------------------------------------
    //build algo
    //--------------------------------------

    let build_algo = object!{
        "alg":JsonValue::String(String::from("RS256")),
        "typ":JsonValue::String(String::from("JWT"))
    };
    let build_algo_string = stringify(build_algo);
    let build_algo_base64 = base64_encode(build_algo_string.as_bytes().to_vec());

    //--------------------------------------
    //build token
    //--------------------------------------

    let current_time:u64;
    match get_current_time(){
        Ok(v)=>{current_time = v;},
        Err(_e)=>{
            println!("!!! failed-get_current_time => {:?}",_e);
            return Err("faile-get_current_time");
        }
    }

    let expire_at = current_time + (59 * 60);
    let expire_at_num:Number = expire_at.into();
    let current_time_num:Number = current_time.into();

    //https://www.googleapis.com/auth/devstorage.read_only
    //https://www.googleapis.com/auth/firestore

    let build_token = object!{
        "iss":JsonValue::String(email),
        // "scope":JsonValue::String(String::from("https://www.googleapis.com/auth/firestore")),
        // "scope":JsonValue::String(String::from("https://www.googleapis.com/auth/devstorage.read_only")),
        "scope":JsonValue::String(scope),
        "aud":JsonValue::String(String::from("https://oauth2.googleapis.com/token")),
        "exp":JsonValue::Number(expire_at_num),
        "iat":JsonValue::Number(current_time_num),
    };
    let build_token_string = stringify(build_token);
    // println!("\ntoken : {}\n",build_token_string);
    let build_token_base64 = base64_encode(build_token_string.as_bytes().to_vec());

    //--------------------------------------
    //generate signarture
    //--------------------------------------

    let signature_string = format!("{}.{}",build_algo_base64,build_token_base64);

    let signature:String;
    match sign_token(signature_string.as_bytes().to_vec(),private_key){
        Ok(v)=>{signature = v;},
        Err(_)=>{
            return Err("failed-sign_token");
        }
    }

    let jwt = format!("{}.{}.{}",build_algo_base64,build_token_base64,signature);

    //replace "=" with ""
    //replace "+" with "-"
    //replace "/" with "_"

    // while jwt.contains("="){jwt = jwt.replace("=","");}
    // while jwt.contains("="){jwt = jwt.replace("+","-");}
    // while jwt.contains("/"){jwt = jwt.replace("/","_");}

    // println!("{}",jwt);

    // return Err("no_error");

    return Ok(jwt);

}

fn sign_token(data:Vec<u8>,key:PKey<Private>)->Result<String,&'static str>{

    let mut signer:Signer;
    match Signer::new(MessageDigest::sha256(), &key){
        Ok(v)=>{signer = v;},
        Err(_)=>{return Err("failed-init-signer");}
    }

    match signer.update(&data){
        Ok(_)=>{},
        Err(_)=>{return Err("failed-update-signer");}
    }

    match signer.sign_to_vec(){
        Ok(v)=>{
            return Ok(base64_encode(v));
        },
        Err(_)=>{return Err("failed-sign-signer");}
    }

}

fn get_current_time()->Result<u64,()>{
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            return Ok(n.as_secs());
        },
        Err(_) => {
            return Err(());
        }
    }
}

fn base64_encode(v:Vec<u8>)->String{
    return BASE64ENCODER.encode(&v);
}
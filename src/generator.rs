use reqwest::Client;
use crate::token;
use reqwest::{Response,Request};
use urlencoding::encode as UrlEncodingEncode;
use json::parse as JsonParse;
use json::JsonValue;
use crate::io::read_json;

pub async fn init(file_path:String,scope:String)->Result<JsonValue,String>{
    let creds:JsonValue;
    match read_json(file_path).await{
        Ok(v)=>{creds = v;},
        Err(_e)=>{
            // println!("!!! failed-read_credss => {:?}",_e);
            // return Err("faile-read_creds");
            return Err(format!("failed-read_json => {:?}",_e));
        }
    }
    getter(&creds,scope).await
}

pub async fn init_json(creds:&JsonValue,scope:String)->Result<JsonValue,String>{
    getter(creds,scope).await
}

pub async fn getter(creds:&JsonValue,scope:String)->Result<JsonValue,String>{

    let jwt:String;
    match token::init(creds,scope).await{
        Ok(v)=>{jwt = v;},
        Err(_e)=>{
            return Err(format!("failed-token-init => {:?}",_e));
        }
    }

    // let form = Form::new()
    // .text("grant_type",UrlEncodingEncode("urn:ietf:params:oauth:grant-type:jwt-bearer"))
    // .text("assertion",jwt);

    let body = format!("grant_type={}&assertion={}",
        UrlEncodingEncode("urn:ietf:params:oauth:grant-type:jwt-bearer"),
        jwt
    );

    let request:Request;
    match Client::new()
    .post("https://oauth2.googleapis.com/token")
    // .multipart(form)
    .header("Content-Type","application/x-www-form-urlencoded")
    .body(body)
    .build(){
        Ok(v)=>{request = v;},
        Err(_e)=>{
            // return Err("failed-build_request".to_string());
            return Err(format!("failed-build_request => {:?}",_e));
        }
    }

    let response:Response;
    match Client::new().execute(request).await{
        Ok(v)=>{response = v;},
        Err(_e)=>{
            return Err(format!("failed-send_request-client => {:?}",_e));
        }
    }

    let response_code:u16 = response.status().as_u16();
    let response_string;
    match response.text().await{
        Ok(v)=>{response_string = v;},
        Err(_e)=>{
            return Err(format!("failed-get-response-string => {:?}",_e));
        }
    }

    // println!("code : {}",response_code);
    // println!("body : {}",response_string);

    if response_code != 200{
        return Err(format!("request-failed => {:?}",response_code));
    }

    match JsonParse(&response_string){
        Ok(token)=>{
            return Ok(token);
            // if !token.has_key("access_token"){return Err("invalid-response_json");}
            // match token["access_token"].as_str(){
            //     Some(v)=>{
            //         return Ok(v.to_string());
            //     },
            //     None=>{return Err("not_found-access_token-response_json");}
            // }
        },
        Err(_e)=>{
            // return Err("failed-parse_response_json".to_string());
            return Err(format!("failed-parse_response_json => {:?}",_e));
        }
    }

}
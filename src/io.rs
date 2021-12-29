
use tokio::io::{AsyncReadExt};
use tokio::fs::File;
use json::parse as JsonParse;
use json::JsonValue;

pub async fn read_json(path:String)->Result<JsonValue,&'static str>{

    let body:String;
    match read_string(path).await{
        Ok(v)=>{body = v;},
        Err(e)=>{return Err(e);}
    }

    match JsonParse(&body){
        Ok(v)=>{return Ok(v);},
        Err(_)=>{return Err("failed-parse_to_json");}
    }

}

async fn read_string(path:String)->Result<String,&'static str>{

    let body:Vec<u8>;
    match read_raw(path).await{
        Ok(v)=>{body = v;},
        Err(e)=>{return Err(e);}
    }

    match String::from_utf8(body){
        Ok(v)=>{return Ok(v);},
        Err(_)=>{return Err("failed-parse_to_string");}
    }

}

async fn read_raw(path:String)->Result<Vec<u8>,&'static str>{

    let mut file:File;
    match File::open(&path).await{
        Ok(v)=>{file = v;},
        Err(_)=>{
            return Err("failed-open_file");
        }
    }

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer).await{
        Ok(_)=>{},
        Err(_)=>{
            return Err("failed-read_file");
        }
    }

    return Ok(buffer);

}
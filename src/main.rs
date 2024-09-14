#![allow(unused_imports)]
use resp::Value;
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use std::{env,time::Duration};

mod resp;
mod store;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let args:Vec<String> = env::args().collect();
    loop {
        let stream = listener.accept().await;
        
        match stream {
            Ok((stream, _)) => {
                println!("accepted new connection");
                let mut args_to_pass = Vec::new();
                if args.len() > 1 {
                    args_to_pass = args[1..].to_vec();    
                }
                
                tokio::spawn(async move{
                    handle_conn(stream, args_to_pass).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}


// *2\r\n$4\r\nECHO\r\n$3\r\nhey\r\n
async fn handle_conn(stream: TcpStream, cmd_args: Vec<String>) {
    let mut handler = resp::RespHandler::new(stream);
    let mut store = store::Store::new();
    println!("Starting read loop");
    print!("Got args {:?}", cmd_args);
    if cmd_args.len() > 0 {
        store.set(remove_prefix(cmd_args[0].as_str()), cmd_args[1].clone(), None);
        store.set(remove_prefix(cmd_args[2].as_str()), cmd_args[3].clone(), None);   
    }
    loop {
        let value = handler.read_value().await.unwrap();
        println!("Got value {:?}", value);
        
        let response = if let Some(v) = value {
            let (command, args) = extract_command(v).unwrap();
            println!("Got command {} with args {:?}", command, args);
            match command.to_lowercase().as_str() {
                "ping" => Value::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                "get" => store.get(args.first().unwrap()),
                "set" => {
                    if args.len() > 2 {
                        let px = args.get(2).unwrap().to_string();
                        if px.to_lowercase().as_str() == "px" {
                            store.set(
                                args.first().unwrap().to_string(),
                                args.get(1).unwrap().to_string(),
                                Some(Duration::from_millis(args.get(3).unwrap().to_string().parse::<u64>().unwrap())),
                            );
                        } 
                    } else {
                        store.set(
                            args.first().unwrap().to_string(),
                            args.get(1).unwrap().to_string(),
                            None,
                        );
                    }   
                    Value::SimpleString("OK".to_string())
                },
                "config" => {
                    if args.first().unwrap().to_string().to_lowercase().as_str() == "get" {
                        let key = args.get(1).unwrap();
                        let value = store.get(key);
                        let response:Vec<Value> = vec![Value::BulkString(key.to_string()), value];
                        Value::Array(response)
                    }else  {
                        Value::SimpleString("Error".to_string())
                    }
                },
                c => panic!("Cannot handle command {}", c),
            }
        } else {
            break;
        };
        println!("Sending value {:?}", response);
        handler.write_value(response).await.unwrap();
    }
}
fn extract_command(value: Value) -> Result<(String, Vec<Value>)> {
    match value {
        Value::Array(a) => {
            Ok((
                unpack_bulk_str(a.first().unwrap().clone())?,
                a.into_iter().skip(1).collect(),
            ))
        },
        _ => Err(anyhow::anyhow!("Unexpected command format")),
    }
}
fn unpack_bulk_str(value: Value) -> Result<String> {
    match value {
        Value::BulkString(s) => Ok(s),
        _ => Err(anyhow::anyhow!("Expected command to be a bulk string"))
    }
}

fn remove_prefix(arg: &str) -> String {
    arg.strip_prefix("--")
        .unwrap_or(arg)
        .to_string()  
}
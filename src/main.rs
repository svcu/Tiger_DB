use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, collections::HashMap, hash::Hash, fs::{File, self}, fmt::Error};
use serde_derive::{Deserialize, Serialize};


use serde_json::{Value, from_str, to_string, Error};


#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Entry{
    entry_type: String,
    schema: String,
    vertices: Vec<String>,
    data: Value,
}

impl Entry {
    fn getType(&self) -> String{
        self.entry_type.clone()
    }

    fn getRef(&self, map: &HashMap<String, Entry>) -> Option<&Entry>{
        if self.entry_type != "ref"{
            return  None;
        }else{
            return Some(map.get(&self.schema).unwrap());
        }
    }

    fn getSchema(&self) -> String{
        self.schema
    }

    fn getData(&self) -> Value{
        self.data
    }

    fn getVertices(&self) -> Vec<String>{
        self.vertices
    }

    fn dfs(&self, map: &HashMap<String, Entry>) -> Vec<String>{
        
        let stack: Vec<String> = Vec::new();
        let visited: Vec<String> = Vec::new();

        for node in self.vertices.iter(){
            stack.push(node.to_string());
        }

        while !stack.is_empty() {
            let actual = stack.pop().unwrap();
            if visited.contains(&actual) {
                continue;
            }

            visited.push(actual);

            let curr_entry = map.get(&actual).unwrap().clone();

            for node in curr_entry.getVertices(){
                stack.push(node)
            }
        }

        visited
        
    }


}

fn handle_conn(msg: String, map: &mut HashMap<String, Entry>, stream: &mut TcpStream){

    let converted_data: Value = from_str(&msg).unwrap();

    let instruction = &converted_data["instruction"];

    if instruction == "insert" {
        let key = &converted_data["key"];
        let mut entry: Entry = from_str(&converted_data["entry"].to_string()).unwrap();

        let res = map.insert(key.to_string(),  entry);

        if res.is_some() {
            write(&map);

            _ = stream.write(String::from("OK").as_bytes());
        }else{
            _ = stream.write(String::from("Error").as_bytes());
        }

    } else if instruction == "get"{
        let key = &converted_data["key"];
        let entry = map.get(&key.to_string()).unwrap();

        let res = to_string(entry);

        if res.is_err(){
            _ = stream.write(String::from("Error").as_bytes());
        }else{
            _ = stream.write(res.unwrap().as_bytes());
        }

    } else if instruction == "dfs"{
        let key = &converted_data["key"].to_string();

        let entry = map.get(key);

        if entry.is_some(){
            let neighbors = to_string(&entry.unwrap().dfs(&map));

            if neighbors.is_err(){  
                _ = stream.write(String::from("Error").as_bytes());
            }else{
                _ = stream.write(neighbors.unwrap().as_bytes())
            }

            
        }else{
            _ = stream.write(String::from("Error").as_bytes());
        }


    }
     

}

fn write(map: &HashMap<String, Entry>){
    _ = fs::write("./db.json", to_string(map).unwrap())
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:2310").unwrap();
    let mut json_map: HashMap<String, Entry> = HashMap::new();

    let db = fs::read_to_string("./db.json");

    if db.is_err() {
        _ = File::create("./db.json").unwrap();
        let db = fs::read_to_string("./db.json").unwrap();
        json_map = from_str(&db).unwrap();
        _ = fs::write("./db.json", "{}")
    }else{
        json_map = from_str(&db.unwrap()).unwrap();
    }




    loop{
        for stream in server.incoming(){

     
            let mut stream = stream.unwrap();
       

            let mut reader = BufReader::new(&stream);
            let mut msg: String = String::new();

            _ = reader.read_line(&mut msg);

            if msg=="close" {
                break;
            }

            handle_conn(msg, &mut json_map, &mut  stream)
        }
    }

}

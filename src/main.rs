use std::{ io::{BufReader, BufRead, Write, Read}, collections::{HashMap, VecDeque}, fs::{File, self, read}, net::{TcpListener, TcpStream}};
use serde_derive::{Deserialize, Serialize};



use serde_json::{Value, from_str, to_string};


#[derive(Deserialize, Debug, Serialize)]
struct Entry{
    entry_type: String,
    schema: String,
    vertices: Vec<String>,
    data: Value,
}

impl Entry {

    fn get_type(&self) -> String{
        self.entry_type.clone()
    }

     /*fn getRef (&self, map: &HashMap<String, Entry>) -> Option<&Entry>{
        if self.entry_type != "ref"{
            return  None;
        }else{
            let entry = map.get(&self.schema).unwrap();
            return Some(entry);
        }
    }*/

    fn get_schema(&self) -> String{
        self.schema.clone()
    }

    fn get_data(&self) -> Value{
        self.data.clone()
    }

    fn get_vertices(&self) -> Vec<String>{
        self.vertices.clone()
    }


    //O(V+E)
    fn dfs(&self, map: &HashMap<String, Entry>) -> Vec<String>{
        
        let mut stack: Vec<String> = Vec::new();
        let mut visited: Vec<String> = Vec::new();

        //Populate stack with neighbors
        for node in self.vertices.iter(){
            stack.push(node.to_string());
        }

        //DFS
        while !stack.is_empty() {
            let actual = stack.pop().unwrap();
            if visited.contains(&actual) {
                continue;
            }

            visited.push(actual.clone());

            let curr_entry = map.get(&actual).unwrap().clone();

            for node in curr_entry.get_vertices(){
                stack.push(node)
            }
        }

        //Return visited nodes
        visited
        
    }

    fn bfs(&self, map: &HashMap<String, Entry>) -> Vec<String>{
        
        let mut stack: VecDeque<String> = VecDeque::new();
        let mut visited: Vec<String> = Vec::new();

        //Populate stack with neighbors
        for node in self.vertices.iter(){
            stack.push_back(node.to_string());
        }

        //DFS
        while !stack.is_empty() {
            let actual = stack.pop_front().unwrap();
            if visited.contains(&actual) {
                continue;
            }

            visited.push(actual.clone());

            let curr_entry = map.get(&actual).unwrap().clone();

            for node in curr_entry.get_vertices(){
                stack.push_back(node)
            }
        }

        //Return visited nodes
        visited
        
    }

    fn add_vertex(&mut self, vertex: &String){
        self.vertices.push(vertex.clone());
    }

    fn update(&mut self, property: &String, value: &Value){
        let mut h_map: serde_json::Map<String, Value> = serde_json::Map::new();
        h_map.insert(property.clone(), value.clone());
        self.data.get_mut(property).unwrap().as_object_mut().replace(&mut h_map);
    }


}

fn handle_conn(mut msg: String, map: &mut HashMap<String, Entry>, stream: &mut TcpStream){

   // msg = msg.trim().to_string();
    let converted_data: Value = from_str(&msg).unwrap();

    let instruction = &converted_data["instruction"];

    println!("{}", instruction);

    if instruction == "insert" {

        let key = &converted_data["key"];
        let mut entry: Entry = from_str(&converted_data["entry"].to_string()).unwrap();

        //Insert entry in map
        _ = map.insert(key.to_string(),  entry);


       //Write map into db.json
       write(&map); 


       _ = stream.write(String::from("OK").as_bytes());
       _ = stream.shutdown(std::net::Shutdown::Both);
 

    } else if instruction == "get"{
        let key = &converted_data["key"];
        let entry = map.get(&key.to_string()).unwrap();

        let res = to_string(entry);

        if res.is_err(){
            _ = stream.write(String::from("Error").as_bytes());
            _ = stream.shutdown(std::net::Shutdown::Both);
 
        }else{
            _ = stream.write(res.unwrap().as_bytes());
            _ = stream.shutdown(std::net::Shutdown::Both);

        }

    } else if instruction == "dfs"{
        let key = &converted_data["key"].to_string();

        let entry = map.get(key);

        if entry.is_some(){
            let neighbors = to_string(&entry.unwrap().dfs(&map));

            if neighbors.is_err(){  
                _ = stream.write(String::from("Error").as_bytes());
                _ = stream.shutdown(std::net::Shutdown::Both);
 
            }else{
                _ = stream.write(neighbors.unwrap().as_bytes());
                _ = stream.shutdown(std::net::Shutdown::Both);
 
            }

            
        }else{
            _ = stream.write(String::from("Error").as_bytes());
            _ = stream.shutdown(std::net::Shutdown::Both);
 
        }


    }else if instruction=="add_vertex"{
        let key = &converted_data["key"].to_string();
        let vertex = &converted_data["vertex"].to_string();

        let entry = map.get_mut(key).unwrap();

        entry.add_vertex(vertex);

        
       _ = stream.write(String::from("OK").as_bytes());
       _ = stream.shutdown(std::net::Shutdown::Both);
 
    }else if instruction=="update"{
        let key = &converted_data["key"].to_string();
        let property = &converted_data["property"].to_string();
        let mut new_value = &converted_data["new_value"];

        let mut entry = map.get_mut(key).unwrap();

        _ = entry.update(property, new_value);

        
       _ = stream.write(String::from("OK").as_bytes());
       _ = stream.shutdown(std::net::Shutdown::Both);
 
     }else if instruction == "delete"{
        let key = &converted_data["key"].to_string();

        _ = map.remove(key);

        
       _ = stream.write(String::from("OK").as_bytes());
       _ = stream.shutdown(std::net::Shutdown::Both);
 
     }else if instruction == "bfs"{
        let key = &converted_data["key"].to_string();
        let entry = map.get(key).unwrap();

        let bfs = entry.bfs(&map);

        _ = stream.write(to_string(&bfs).unwrap().as_bytes());
        _ = stream.shutdown(std::net::Shutdown::Both);
 
         
     }
     

}

fn write(map: &HashMap<String, Entry>){
    _ = fs::write("./db.json", to_string(map).unwrap())
}

fn handle_tcp(mut stream: TcpStream, map: &mut HashMap<String, Entry>){

    
    let second = stream.try_clone().unwrap();
    let mut reader = BufReader::new(second);
    let mut conn = String::new();

    reader.read_line(&mut conn);

    handle_conn(conn.trim().replace("\0", ""), map, &mut stream)
        
    
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:2310").unwrap();
    let mut json_map: HashMap<String, Entry> = HashMap::new();
    let db = fs::read_to_string("./db.json");
    if db.is_err() {
        _ = File::create("./db.json").unwrap();
        _ = fs::write("./db.json", "{}");
        let db = fs::read_to_string("./db.json").unwrap();
        
        json_map = from_str(&db).unwrap();
    }else{
        json_map = from_str(&db.unwrap()).unwrap();
    }




    loop{
        for stream in server.incoming(){


                let mut stream = stream.unwrap();

                println!("Connection from: {:?}", stream.peer_addr());

                handle_tcp(stream, &mut json_map);
            
        }
    }

}

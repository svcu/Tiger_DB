# Tiger_DB
Tiger_DB is a Graph-NoSql database for the purposes of scaling and giving the user absolute power over their data. For now it only has the basic CRUD actions and the DFS algorithm

# How to use

## Installation

- Download the latest release of tiger db
- Run the executable

## Building the code

Run:  `cargo build`

## Drivers

Tiger_DB has a driver for JavaScript: https://github.com/svcu/Tiger_DB-js

You can also use it with raw sockets:

- Open a connection to your database uri on port 2310
- Send a message as a string with the following structure
```
{
    instruction : "insert" | "update" | "get" | "delete" | "dfs" | "bfs" | "add_vertex"
    key : //THE KEY OF THE ENTRY YOU WANT TO MODIFY,
    // IF YOU WANT TO INSERT entry : {
        entry_type : "ref" | "normal" //REF POINTS TO AN ENTRY,
        schema: "false" | "KEY OF THE ENTRY YOU WANT TO POINT AT",
        vertices: [] //ARRAY WITH NEIGHBORS OF THE ENTRY,
        data: //JSON OBJECT WITH THE DATA YOU WISH
    },
    //IF YOU WANT TO UPDATE property :  "" //PROPERTY YOU WANT TO UPDATE,
    new_value : "" //VALUE YOU WANT TO PUT IN PLACE OF THE PREVIOUS VALUE,
    vertex: "" //IF YOU WANT TO ADD A VERTEX TO THE GRAPH 

}+'\n'
```

- Sends a message with the following text 'close\n' and closes the connection to the database

# Plans

- [] Adding CLI
- [] Adding Schemas
- [] Cryptography
- [x] Add BFS
- Sugest ideas

# Contribute

Feel free to contribute
use std::fs;
use std::io::Read;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Host {
    chain: String,
    #[serde(rename = "idKey")]
    id_key: String,
    color: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    color: String,
    piece: String,
    from: String,
    to: String,
    flags: String,
    san: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CreatedAt {
    #[serde(rename = "$date")]
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpdatedAt {
    #[serde(rename = "$date")]
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Guest {
    chain: String,
    #[serde(rename = "idKey")]
    id_key: String,
    color: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    host: Host,
    description: String,
    moves: Vec<Move>,
    fen: String,
    #[serde(rename = "createdAt")]
    created_at: CreatedAt,
    #[serde(rename = "updatedAt")]
    updated_at: UpdatedAt,
    #[serde(rename = "__v")]
    v: i32,
    guest: Guest,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = fs::File::open("src/gamedata.json")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let value = serde_json::from_str(&contents)?;
    let game: Root = value;

    /* yo Teke us the dot notation to target values 
       example if i want to read the game description, ill do this
       println!("{}", game.description);
       where game is the root struct, use the game variable 
       to target every other data in the json file
    */ 
    println!("{:#?}", game);

    Ok(())
}

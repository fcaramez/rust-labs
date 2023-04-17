// Import necessary libs

use serde::de::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

struct Movie {
    title: String,
    year: u32,
    director: String,
    genre: Vec<String>,
    score: f32,
}

fn get_movie_data(file_name: &str) -> Result<Vec<Movie>, String> {
    let mut file = File::open(file_name).map_err(|e| format!("Error opening file: {}", e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Error reading file: {}", e))?;

    let movies =
        serde_json::from_str(&contents).map_err(|e| format!("Error deserializing JSON: {}", e))?;

    Ok(movies)
}

fn main() {}

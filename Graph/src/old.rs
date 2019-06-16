use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;


pub struct Vertex<'r> {
  id: u32,
  adj_edges: Vec<&'r mut Edge<'r>>
}

pub struct Edge<'r> {
  v1: &'r Vertex<'r>,
  v2: &'r Vertex<'r>,
  weight: f32
}

pub struct Graph<'r> {
  num_vertices: u32,
  vertices: Vec<&'r mut Vertex<'r>>
}


impl<'r> Edge<'r> {
  fn new(v1: &'r Vertex<'r>, v2: &'r Vertex<'r>, weight: f32) -> Self {
    Self {
      v1: v1,
      v2: v2,
      weight: weight
    }
  }

  fn v1(&mut self)-> &Vertex<'r> {
    self.v1
  }

  fn v2(&mut self)-> &Vertex<'r> {
    self.v2
  }

  fn weight(&mut self)-> &mut f32 {
    &mut self.weight
  } 
}


impl<'r> Vertex<'r> {
  fn new(id: u32) -> Self {
    Self {
      id: id,
      adj_edges: Vec::new()
    }
  }

  fn id(&self) -> u32 {
    self.id
  }

  fn adj(&self) -> &Vec<&'r mut Edge<'r>> {
    &self.adj_edges
  }

  fn add_edge(&mut self, e: &'r mut Edge<'r>) {
    self.adj_edges.push(e);
  }
}


impl<'r> Graph<'r> {
  pub fn new(filename: String) {
    let file_content = {
      // Create a path to the desired file
      let path = Path::new(&filename);

      // Open the path in read-only mode, returns `io::Result<File>`
      let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Couldn't open file"),
        Ok(file) => file,
      };

      // Read the file contents into a string, returns `io::Result<usize>`
      let mut s = String::new();
      match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read file"),
        Ok(_) => s
      }
    };

    let mut split = file_content.split_whitespace().collect::<Vec<&str>>();
    let graph_size = split[0].parse::<u32>().unwrap();

    let mut vertices = Vec::new();

    for i in 0..graph_size {
      vertices.push(Vertices::new(i as u32));
    }

    let mut j = 1;
    loop {
      let v1 = split[j].parse::<u32>().unwrap();
      let v2 = split[j + 1].parse::<u32>().unwrap();
      let weight = split[j + 2].parse::<f32>().unwrap();
      let mut edge = Edge::<'r>::new(&mvertices[v1 as usize], vertices[v2 as usize], weight);

      vertices[v1 as usize].add_edge(&mut edge);

      j += 3;
    }
    

    println!("{}", file_content);
  }

  pub fn vertices(&self) -> &Vec<&'r mut Vertex<'r>> {
    &self.vertices
  }

  pub fn num_vertices(&self) -> u32 {
    self.num_vertices
  }
}


fn main() {
  let g = Graph::new(String::from("hello.txt"));
}

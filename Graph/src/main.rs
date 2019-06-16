use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io;


pub struct Graph {
  num_vertices: u32,
  adj_list: Vec<Vec<u32>>
}


impl Graph {
  pub fn new(filename: String) -> Self {
    let file_content = {
      let mut s = String::new();
      let mut content = match File::open(Path::new(&filename)) {
        Err(why) => panic!("Couldn't open file"),
        Ok(mut file) => {
          match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read file"),
            Ok(_) => s
          }
        },
      };
      content
    };

    let split = file_content.split_whitespace().collect::<Vec<&str>>();

    Self {
      num_vertices: split[0].parse::<u32>().unwrap(),

      adj_list: {
        let mut vertices = Vec::with_capacity(split[0].parse::<usize>().unwrap());
        
        for i in 0..vertices.capacity() {
          vertices.push(Vec::new());
        }

        let mut j = 1;
        loop {
          if (j >= split.len() - 2) {
            break;
          }
          let v1 = split[j].parse::<u32>().unwrap();
          let v2 = split[j + 1].parse::<u32>().unwrap();
          vertices[v1 as usize].push(v2);
          vertices[v2 as usize].push(v1);
          j += 2;
        }
        vertices
      }
    }

  }

  pub fn num_vertices(&self) -> u32 {
    self.num_vertices
  }

  pub fn adj(&self, vertex: u32) -> &Vec<u32> {
    &self.adj_list[vertex as usize]
  }
}


fn main() {
  let g = Graph::new(String::from("hello.txt"));

  for i in 0..g.num_vertices() {
    print!("{} : ", i);
    for j in g.adj(i) {
      print!("{}, ", j);
    }
    println!("");
  }
}

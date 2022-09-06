use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn intersection(a : &Vec<u32>,  b : &Vec<u32>) -> Vec<u32> {
  let mut result : Vec<u32> = vec!();
  // // If not sorted, you have to use this
  // for i in 0..a.len() {
  //   for j in i..b.len() {
  //     if a[i] == b[j] {
  //       result.push(a[i]);
  //       continue;
  //     }
  //   }
  // }

  let mut i = 0;
  let mut j = 0;

  // Cheeky set intersection assuming pre-sorted
  while i < a.len() && j < b.len() {
    if a[i] == b[j] {
      result.push(a[i]);
      i+=1;
      j+=1;
    }
    else if a[i] > b[j] {
      j+=1;
    }
    else {
      i+=1;
    }
  }
  return result;
}

fn print_alternatives(hash : u32, anagrams : &HashMap<u32, Vec<String>>) {
    print!("- ");
    for word in &anagrams[&hash] {
      print!("{} ", word);
    }
    println!("");
}

fn main() {
  // Sanity check intersection code
  // let a  = vec!(1,2,3,9,11,22,33,42);
  // let b = vec!(2,4,9,33,42);
  // println!("{:?}", intersection(&a, &b));

  let mut anagrams : HashMap<u32, Vec<String>> = HashMap::new();
  let mut neighbors : Vec<Vec<u32>> = vec!();
  let mut hashes : Vec<u32> = vec!();

  // Copy this to the current working path
  let file = File::open("words_alpha.txt").expect("file not found!");
  let buf_reader = BufReader::new(file);

  // Process the file, note anagrams, discard words with duplicate letters
  // and note their "hash". 26 letters in alpha, so one bit per letter fits
  // nicely into a u32.
  for line in buf_reader.lines() {
    let word = line.unwrap();
    if word.len() == 5 {
      let mut hash = 0u32;
      let mut duplicate_letters = false;
      for c in word.to_lowercase().chars() {
        let mask = 1 << ((c as u32) - ('a' as u32));
        if (hash & mask) != 0 {
          duplicate_letters = true;
          break;
        }
        hash |= mask;
      }
      if !duplicate_letters {    
        let curr_anagrams = anagrams.entry(hash).or_insert(vec!());
        curr_anagrams.push(word.clone());
        // Only worry about the first word for our algorithm
        if curr_anagrams.len() == 1 {
          neighbors.push(vec!());
          hashes.push(hash);
        }
      }
    }
  }

  // Build up neighbors
  for i in 0..hashes.len() {
    for j in i..hashes.len() {
      if hashes[i] & hashes[j] == 0 {
        neighbors[i].push(j as u32);
      }
    }
  }

  // Now find groups of 5
  let mut count = 0;
  for zero_order_neighbor in 0..neighbors.len() {
    let zero_order_neighbors = &neighbors[zero_order_neighbor];
    for first_order_neighbor in zero_order_neighbors {
      if *first_order_neighbor as usize > zero_order_neighbor {
        let first_order_neighbors = intersection(zero_order_neighbors, &neighbors[*first_order_neighbor as usize]);
        if first_order_neighbors.len() >= 3 {
          for second_order_neighbor in &first_order_neighbors {
            if *second_order_neighbor > *first_order_neighbor {
              let second_order_neighbors = intersection(&first_order_neighbors, &neighbors[*second_order_neighbor as usize]);
              if second_order_neighbors.len() >= 2 {
                for third_order_neighbor in &second_order_neighbors {
                  if *third_order_neighbor > *second_order_neighbor {
                    let third_order_neighbors = intersection(&second_order_neighbors, &neighbors[*third_order_neighbor as usize]);
                    if third_order_neighbors.len() >= 1 {           
                      for fourth_order_neighbor in &third_order_neighbors {
                        if *fourth_order_neighbor > *third_order_neighbor {
                            println!("Solution: {}", count + 1);
                            print_alternatives(hashes[zero_order_neighbor], &anagrams);
                            print_alternatives(hashes[*first_order_neighbor as usize], &anagrams);
                            print_alternatives(hashes[*second_order_neighbor as usize], &anagrams);
                            print_alternatives(hashes[*third_order_neighbor as usize], &anagrams);
                            print_alternatives(hashes[*fourth_order_neighbor as usize], &anagrams);
                            println!("");
                            // println!("{} {} {} {} {}", words[zero_order_neighbor],
                            //                            words[*first_order_neighbor as usize],
                            //                            words[*second_order_neighbor as usize],
                            //                            words[*third_order_neighbor as usize],
                            //                            words[*fourth_order_neighbor as usize]);
                            count += 1;
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}

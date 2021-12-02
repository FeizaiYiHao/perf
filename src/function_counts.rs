use hashbrown::HashMap;
use crate::*;
use std::fs::File;
use std::io::Read;

pub fn function_counts(fname:&String){

    let mut perf_data = File::open(fname).unwrap();
    let mut buf = String::new();
    perf_data.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
    let mut map:HashMap<String,u64> = hashbrown::HashMap::new();
  
    let mut total:u64 = 0;
  
    for line in lines{
      let ip = parse_line(line).0;
      if ip == ENDOFSTACK{
        total += 1;
      }else{
        match resolve(parse_line(line).2,parse_line(line).1,parse_line(line).0){
          Some(func_name) => {
            let count = map.entry(func_name).or_insert(0);
            *count += 1;
          },
          None => {},
        }
      }
    }
    let mut vec:Vec<(String,u64)> = Vec::new();
    for (k,v) in map{
      vec.push((k,v));
    }
    vec.sort_by_key(|f| - (f.1 as i64));
  
    for (k,v) in vec{
      println!("{:05.2}%  {} ",v as f64/total as f64 * 100 as f64,k);
    }
}

pub fn function_counts_top_only(fname:&String){
    let mut perf_data = File::open(fname).unwrap();
    let mut buf = String::new();
    perf_data.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
    let mut map:HashMap<String,u64> = hashbrown::HashMap::new();
  
    let mut last:u64 = 0;
    for line in lines{
      let ip = parse_line(line).0;
      if ip == ENDOFSTACK{
        match resolve(parse_line(line).2,parse_line(line).1,last){
          Some(func_name) => {
            let count = map.entry(func_name).or_insert(0);
            *count += 1;
          },
          None => {},
        }
      }else{
        last = ip;
      }
    }
    let mut vec:Vec<(String,u64)> = Vec::new();
  
    let mut total:u64 = 0;
    for (k,v) in map{
      vec.push((k,v));
      total += v;
    }
    vec.sort_by_key(|f| - (f.1 as i64));
  
    for (k,v) in vec{
      println!("{} {:.2}%",k,v as f64/total as f64 * 100 as f64);
    }
  }

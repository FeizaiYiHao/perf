use crate::*;
use std::fs::File;
use std::io::Read;

pub fn flame_graph(fname:&String){
    let mut perf_data = File::open(fname).unwrap();
    let mut buf = String::new();
    perf_data.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
  
    let mut s = String::from("");
    let mut map:HashMap<String,u64> = hashbrown::HashMap::new();
  
    for line in lines{
      let ip = parse_line(line).0;
      if ip == ENDOFSTACK{
        let count = map.entry(s).or_insert(0);
        *count += 1;
        s = String::from("");
      }else {
        //println!("{:x}",ip);
        match resolve(parse_line(line).2,parse_line(line).1,parse_line(line).0){
          Some(func_name) => {
            s.push_str(&func_name);
            s.push_str(";");
          },
          None => {},
        }
      }
    }
    for (k,v) in map{
      println!("{} {}",k,v);
    }
  }
use addr2line::gimli::{EndianReader, RunTimeEndian};
use std::fs;
use std::rc::Rc;
use addr2line;
use addr2line::Context;
use std::env;
use hashbrown::HashMap;

mod function_counts;
mod flame_graph;
mod instruction_counts;

const ENDOFSTACK:u64 = 0xFFFFFFFFFFFFFFFF;
static mut CTXTS:Vec<(String,Context<EndianReader<RunTimeEndian, Rc<[u8]>>>)> = Vec::new();

fn main() {
  let paths = fs::read_dir("./perfelf").unwrap();

  for path in paths {
    let bin_data = fs::read(path.as_ref().unwrap().path()).unwrap();
    let parsed = addr2line::object::read::File::parse(bin_data.as_slice());
    if parsed.is_err(){
      continue;
    }
    let parsed = parsed.unwrap();
    let redleaf_ctxt = addr2line::Context::new(&parsed).unwrap();
    unsafe{
      CTXTS.push((path.unwrap().file_name().to_str().unwrap().to_string(),redleaf_ctxt));
    }
  }

  let args: Vec<String> = env::args().collect();
  if args[1] == "flame_graph"{
    flame_graph::flame_graph(&args[2]);
  }else if args[1] == "function_counts" {
    function_counts::function_counts(&args[2]);
  }else if args[1] == "function_counts_top_only" {
    function_counts::function_counts_top_only(&args[2]);
  }else if args[1] == "instruction_counts" {
    instruction_counts::instruction_counts(&args[2]);
  }

}

pub fn parse_line(line:&str)->(u64,u64,String){
  return (
    u64::from_str_radix(line.split(" ").nth(0).unwrap(),16).unwrap(),
    u64::from_str_radix(line.split(" ").nth(1).unwrap(),16).unwrap(),
    line.split(" ").nth(2).unwrap().to_string()
  );
}

pub fn resolve(domain:String,offset:u64,ip:u64)->Option<std::string::String>{
  let ip = ip + offset; // don't need to think about overflow. 
  unsafe{
    for (dname,ctxt) in &CTXTS{
      if !(dname == &domain){
        continue;
      }
      let frame_iter = ctxt.find_frames(ip);
      if frame_iter.is_ok(){
        let frame = frame_iter.unwrap().next();
        if frame.is_ok(){
          let func = frame.unwrap();
          if func.is_some(){
            let function = func.unwrap().function;
            if function.is_some(){
              let str =  function.as_ref().map(|f| f.demangle().unwrap());
              match str{
                Some(fun_name) => {
                  return Some(fun_name.as_ref().to_string())
                },
                None => {
                  return None
                },
              }
            }
          }
        }
      }
    }
  }
  
  None
}



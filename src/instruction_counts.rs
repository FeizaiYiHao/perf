use std::rc::Rc;
use std::fs::File;
use std::io::Read;
use crate::*;

struct FuncAsm{
    pub name:String,
    pub start:u64,
    pub end:u64,
    pub lines:Vec<(u32,u64,u64,String)>,
    pub counts:u64,
  }
  
  impl FuncAsm {
      fn new(name: String) -> FuncAsm {
        FuncAsm { 
          name:name,
          start:0,
          end: 0,
          lines: Vec::new(),
          counts: 0,
  
        }
      }
    }
  
  pub(crate) fn instruction_counts(fname:&String){
    //build disasm vec
    let mut disasm:Vec<Rc<FuncAsm>> = Vec::new();
  
    let mut elf_data = File::open("/home/xiangdc/Desktop/perf/disassembly/example").unwrap();
    let mut buf = String::new();
    elf_data.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
  
    let mut in_section = false;
    let mut in_function = false;
    for line in lines{
      if !in_section{
        if line.contains("Disassembly of section"){
          //println!("find!");
          in_section = true;
        }
      }else if!in_function{
        if line.len()>0 && !line.contains("Disassembly of section") {
          //println!("find func!");
          //println!("{}",line);
          in_function = true;
          
          if disasm.last_mut().is_some(){
            //println!("disasm len {} lines len {}",disasm.len(),Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.len());
            Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.last_mut().unwrap().2 = 
              u64::from_str_radix(line.split(|c| c == '<' || c == '>').nth(0).unwrap().trim_end(),16).unwrap() - 1;
            
            Rc::get_mut(disasm.last_mut().unwrap()).unwrap().end = 
              u64::from_str_radix(line.split(|c| c == '<' || c == '>').nth(0).unwrap().trim_end(),16).unwrap() - 1;
          }
          
          disasm.push(Rc::new(FuncAsm::new(
            line.split(|c| c == '<' || c == '>').nth(1).unwrap().to_string()
          )));
          //println!("{}",line.split(|c| c == '<' || c == '>').nth(0).unwrap().trim_end());
          Rc::get_mut(disasm.last_mut().unwrap()).unwrap().start = u64::from_str_radix(line.split(|c| c == '<' || c == '>').nth(0).unwrap().trim_end(),16).unwrap();
        }
      }else if in_function{
        if line.len()==0{
          in_function = false;
        }else {
          //println!("{}",line.split(":").nth(0).unwrap().trim_start().trim_end());
          if u64::from_str_radix(line.split(":").nth(0).unwrap().trim_start().trim_end(),16).is_ok(){
            if Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.last_mut().is_some(){
              Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.last_mut().unwrap().2 = 
                u64::from_str_radix(line.split(":").nth(0).unwrap().trim_start().trim_end(),16).unwrap() - 1;
            }
            Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.push((
              0,
              u64::from_str_radix(line.split(":").nth(0).unwrap().trim_start().trim_end(),16).unwrap(),
              0,
              line.to_string()
            ));
          }
        }
      }
    }
  
    Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.last_mut().unwrap().2 = 
      Rc::get_mut(disasm.last_mut().unwrap()).unwrap().lines.last_mut().unwrap().1 + 4; 
  
  
  
    //let mut total:u64 = 0;
  
    let mut perf_data = File::open(fname).unwrap();
    let mut buf = String::new();
    perf_data.read_to_string(&mut buf).unwrap();
    let lines = buf.lines();
  
    for line in lines{
      let ip = parse_line(line).0;
      if ip == ENDOFSTACK{
      }else{
        //total += 1;
        match disasm.binary_search_by(|f| f.as_ref().end.cmp(&ip)){
          Ok(i) => {
            Rc::get_mut(disasm.get_mut(i).unwrap()).unwrap().counts+=1;
  
            let instrucs:&mut Vec<(u32,u64,u64,String)> = Rc::get_mut(disasm.get_mut(i).unwrap()).unwrap().lines.as_mut();
            match instrucs.binary_search_by(|f| f.2.cmp(&ip)){
              Ok(ii) => {
                instrucs.get_mut(ii).unwrap().0 += 1;
              },
              Err(ii) => {
                instrucs.get_mut(ii).unwrap().0 += 1;
              },
            }
          },
          Err(i) => {
            Rc::get_mut(disasm.get_mut(i).unwrap()).unwrap().counts+=1;
  
            let instrucs:&mut Vec<(u32,u64,u64,String)> = Rc::get_mut(disasm.get_mut(i).unwrap()).unwrap().lines.as_mut();
            match instrucs.binary_search_by(|f| f.2.cmp(&ip)){
              Ok(ii) => {
                instrucs.get_mut(ii).unwrap().0 += 1;
              },
              Err(ii) => {
                instrucs.get_mut(ii).unwrap().0 += 1;
              },
            }
          },
      }
      }
    }
  
    disasm.sort_by(|a,b|b.as_ref().counts.cmp(&a.as_ref().counts));
  
    for func in disasm{
      let func = func.as_ref();
      let total_in_func_cnt = func.counts as f64;
  
      println!("{} {} {:x} {:x}",func.counts,func.name,func.start,func.end);
      for line in &func.lines{
        //println!("{:.2}% {:x} {:x} {} ",line.0 as f64/total_in_func_cnt * 100 as f64,line.1,line.2,line.3);
        if line.0 as f64/total_in_func_cnt *100 as f64 > 20.0{
          println!("****************************************************************************************");
          println!();
          print!("-> ");
        }
        println!("{:05.2}% {} ",line.0 as f64/total_in_func_cnt *100 as f64,line.3);
        if line.0 as f64/total_in_func_cnt *100 as f64 > 20.0{
          println!();
          println!("****************************************************************************************");
        }
      }
      println!("\n\n\n");
    }
  }
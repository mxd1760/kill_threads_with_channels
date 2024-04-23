// todos
/*
  what actions are there?
    - spawn thread
    - kill thread
    - add to workload of thread?
  
  threads should report their status


*/

use std::{sync::mpsc::TryRecvError, time::Duration};



enum ThreadThing{
  Dumbest(String),
  Transmitter(i32,std::sync::mpsc::Sender<String>),
}

fn user_input() -> i8{
  loop{
    println!();
    println!("Select an action");
    println!(" 0 - quit");
    println!(" 1 - create a new thread");
    println!(" 2 - delete an existing thread");
    print!("Selection (0-2): ");
    use std::io::Write;
    let _ = std::io::stdout().flush();

    let something = std::io::stdin();
    let mut input:String = "".into();
    something.read_line(&mut input).expect("failed to read input");
    match input.trim().parse(){
      Ok(val) => return val,
      Err(_) => println!("not a number"),
    }
  }
}

fn spawn_new_thread(list: &mut Vec<ThreadThing>,cnt: &mut i32){
  println!("Spawning Thread");
  let name = *cnt;
  *cnt += 1;
  let (transmit,recieve) = std::sync::mpsc::channel::<String>();
  list.push(ThreadThing::Transmitter(name.clone(),transmit));
  std::thread::spawn(move ||{
    loop{
      println!("THREAD #{}: reporting",name);
      match recieve.try_recv(){
        Ok(msg)=>{
          if msg == "die"{
            println!("THREAD #{}: Time To Die",name);
            return;
          }
          println!("invalid message recieved by Thread #{}",name);
        },
        Err(TryRecvError::Empty)=>{},
        Err(TryRecvError::Disconnected)=>{},
      }
      std::thread::sleep(Duration::from_millis(2000));
    }
  });
}
fn delete_thread_from(list: &mut Vec<ThreadThing>){
  // prompt user for thread to delete (main cant be deleted)

  for i in 0..list.len(){
    match &list[i]{
      ThreadThing::Dumbest(name)=>{println!(" -x Thread {} cannot be deleted",name)},
      ThreadThing::Transmitter(name,_) => println!(" {} - Thread #{} ready to die",name,name),
    }
  }
  let num;
  loop{
    print!("Selection (0-n): ");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let mut text = "".into();
    std::io::stdin().read_line(&mut text).expect("failed to read input");
    match text.trim().parse(){
      Ok(val) => {
        num = val;
        break;
      },
      Err(_) => println!("not a number"),
    }
  }
  // kill specified thread
  for i in 0..list.len(){
    match &list[i]{
      ThreadThing::Dumbest(_)=>{},
      ThreadThing::Transmitter(name,transmit) => {
        if *name == num{
          match transmit.send("die".into()){
            Ok(_) => {
              list.remove(i);
            },
            Err(_)=>{
              println!("deletion failed");
            },
          }
          return;
        }
      }
    }
  }
}

fn main() {
  // main loop
  //    list of running threads (including this as the main one)
  //    commands or whatever
  //    actions (which ones are which)
  let mut all_threads = vec![ThreadThing::Dumbest("main".into())];
  let mut thread_counter = 0;

  loop{
    println!();
    println!("All Threads:");
    for thd in &all_threads{
      match thd{
        ThreadThing::Dumbest(txt) => println!(" - {}",txt),
        ThreadThing::Transmitter(name,_) => println!(" - Thread #{}",name),
      }
    }
    match user_input(){
      0 => return,
      1 => spawn_new_thread(&mut all_threads,&mut thread_counter),
      2 => delete_thread_from(&mut all_threads),
      _ => println!("unexpected selection"),
    }
  }

}

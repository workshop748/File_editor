use std::fs::{self,File};
use std::io::prelude::*;
use std::io;
use std::io::Result;
use std::fs::OpenOptions;
use std::io::BufReader;

fn get_text_file( message:&str)->io::Result<String>
{
let mut name = String::new();

println!("{}",message);
io::stdin().read_line(&mut name )?;
name = name.trim().to_string();
name.push_str(".txt");
   Ok(name)
}
fn wirte_file(file_path: &str,contents:&str)->Result<()>
{
  let mut file =File::create(file_path)?;
  file.write_all(contents.as_bytes())
}

fn search_word_in_file(file_path: &str, search_word:&str)-> std::io::Result<Vec<usize>>{
  let file = File::open(file_path)?;
  let reader = BufReader::new(file);
  let mut line_numbers = Vec::new();


  for(line_number,line)in reader.lines().enumerate()
  {
    let line = line?;
    if line.contains(search_word)
    {
      line_numbers.push(line_number+1);
    }
  }
  Ok(line_numbers)
}
fn main() {
 loop {
   let mut choice_str = String::new();
 
   println!("Please enter a choice");
   println!("1. Create a file");
   println!("2. Write a to a file");
   println!("3. read to a file");
   println!("4. Append to a file");
   println!("5. Search a file for a word");
   println!("6. Exit the program");

   
 io::stdin().read_line(&mut choice_str).expect("Fialed to work");
let choice = choice_str.trim().parse().unwrap();
 match choice {
1=>
{
  
match get_text_file("please enter a name for your text file")
{
  Ok(name)=>
  {
    match File::create(&name)
    {
      Ok(_)=>println!("we have created your  file: {}",name),
      Err(_e)=> println!("Failed to create a file:{}",name),
    }
  },
  Err(e)=> println!("there was an erro reading your input: {}",e),
}

},
2=>
{
  match get_text_file("Please enter the file you want to read")
  {
    Ok(name)=>
    {
      
       

      let mut content = String::new();
    println!("Please enter text you want to write :");
    io::stdin().read_line(&mut content).expect("we could not read from line");
   
   
    match wirte_file(&name, &content)
    {
      Ok(_) => println!("File written successfully"),
      Err(e) => println!("Error writing file: {}", e),
    }
  }
    ,
    Err(e)=>println!("there was an erro reading your input: {}",e),
  }

}3=>
{
  match get_text_file("Please enter the file you want to read")
  {
    Ok(name)=>
    {
  let content = fs::read_to_string(name).expect("fuck you ");
  println!("{}",content);
    },
    Err(e)=>println!("there was an erro reading your input: {}",e),
  }
}4=>{
//this method will append to a file
match get_text_file("please enter a name for your text file")
{
  Ok(name)=>
  {
    let mut file = OpenOptions::new().write(true).append(true).open(name).unwrap();

    let mut append_text = String::new();
    print!("please enter some text to write append to the file:");
    io::stdin().read_line(&mut append_text).expect("Fialed to work");

if let Err(e) = writeln!(file,"{}", append_text){
eprintln!("couldn't write to file: {}",e);
}
  },
  Err(e)=> println!("there was an erro reading your input: {}",e),
}
}5=>{
//this method will find a word in a file

match get_text_file("please enter a name for your text file")
{
  Ok(name)=>
  {
    let mut find_word = String::new();
    println!("Please enter a word you want to search for");
    io::stdin().read_line(&mut find_word).expect("Fialed to work");
find_word = find_word.trim().to_string();
    match search_word_in_file(&name, &find_word){
      Ok(line_numbers) => {
        if line_numbers.is_empty() {
            println!("The word '{}' was not found in the file.", find_word);
        } else {
            println!("The word '{}' was found on the following line(s):", find_word);
            for line_number in line_numbers {
                println!("Line {}", line_number);
            }
        }
    }
    Err(e) => println!("An error occurred: {}", e),
    }
    }

  

  Err(e)=> println!("there was an erro reading your input: {}",e),

 }

}6=>{
println!("thank you for working with the file editor\n Please come again");
break;
}
_=>{
println!("this is not a valid number, Please Try again");
}
 
 }
}
}
   
   


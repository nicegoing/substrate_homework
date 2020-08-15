extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("cal")
        .version("0.1")
        .author("alex")
        .about("计算器")
        .subcommand(
            App::new("minus")
                .arg(Arg::with_name("number")
                .required(true)
                .number_of_values(2)
                .about("减法"),
        ))
        .subcommand(
            App::new("add")
                .arg(Arg::with_name("number")
                .required(true)
                .number_of_values(2)
                .about("加法"),
        ))
        .arg(Arg::with_name("verbose")
        .short('v')
        .multiple(true)
        .about("verbosity level"))
        .get_matches();

    //实现减法
    if let Some(matches) = matches.subcommand_matches("minus") {
       
        println!();
        let number: Vec<_> = matches.values_of("number").unwrap().collect();
        
        let x=number[0].parse::<isize>().unwrap();
        let y=number[1].parse::<isize>().unwrap();

        for i in number{//number所有权转移
            print!("{} ",i)
        }
        println!();
        let operation=Operation{
            x:x,y:y,operator:"minus",};
        operation.display();       
    }
    //实现加法
    if let Some(matches) = matches.subcommand_matches("add") {
        let number: Vec<_> = matches.values_of("number").unwrap().collect();
    
        let x=number[0].parse::<isize>().unwrap();
        let y=number[1].parse::<isize>().unwrap();
        let operation=Operation{
            x:x,y:y,operator:"add",};
        operation.display();
    }

}

pub struct Operation{
    operator:&'static str,
    x:isize,
    y:isize,
}

//定义display trait
pub trait Display{
    fn display(&self);
}

impl Display for Operation{
    fn display(&self){
       match self.operator{
           "minus"=>
           println!("{} - {} = {}",self.x,self.y,self.x-self.y),
           "add"=>
           println!("{} + {} = {}",self.x,self.y,self.x+self.y),
           _ => println!("{}",self.operator),

       }
    }
}

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{

    loop {	
	    println!("Please enter your guessed number");
	    let secret_num = rand::thread_rng().gen_range(1..101);
	    
	    let mut guess = String::new();
	    io::stdin().read_line(&mut guess)
	    	.expect("failed to read the line");
	    	
	    let guess: u32=  guess.trim().parse()
	    	.expect("please type a number");
	    
	    println!("you have guessed {}" , guess);
	    
	    match guess.cmp(&secret_num)
	    {
	    	Ordering::Less =>  println!("Too small"),
	    	Ordering::Greater =>  println!("too big"),
	    	Ordering::Equal =>  
	    	{
	    		println!("correct num");
	    		break;
	    	}
	    }	
 	}     	    		
}

use std::{io, thread, time::Duration};
use rand::Rng;

fn main() 
{
    println!();
    println!("Welcome comarade to russian roulette");

    thread::sleep(Duration::from_secs(1));

    'menu_loop: loop
    {
        let mut play_input:String = String::new();

        println!();
        println!("Do you want to play a round?\n\
        1.Play \n\
        2.Exit");

        io::stdin()
        .read_line(&mut play_input)
        .expect("Failed to read line");

        let play_input = play_input.trim();

        if play_input == "1"
        {
            let mut bullets = vec![0, 0, 0, 0, 0, 0];
            let mut barrel_to_load: usize = 0;

            println!();
            println!("Lets begin commarade");

            thread::sleep(Duration::from_secs(1));

            println!();
            println!("You can't stop playing until you either win or lose");

            thread::sleep(Duration::from_secs(1));
            
            'game_loop: loop
            {
                let mut game_input:String = String::new();

                println!();
                println!("Input 1 to pull trigger");

                io::stdin()
                .read_line(&mut game_input)
                .expect("Failed to read line");

                let game_input = game_input.trim();

                if game_input == "1"
                {
                    let bullets_in_gun:usize  = barrel_to_load + 1;

                    println!();
                    println!("Bullets in gun: {}", bullets_in_gun);

                    thread::sleep(Duration::from_secs(1));

                    bullets[barrel_to_load] = 1;

                    println!();
                    println!("Pulling trigger...");

                    thread::sleep(Duration::from_secs(1));

                    let mut rng = rand::rng();
                    let index = rng.random_range(0..6);

                    let chamber = bullets[index];

                    if chamber == 1
                    {
                        println!();
                        println!("Bang!");

                        thread::sleep(Duration::from_secs(1));
                        
                        println!();
                        println!("You lose better luck next time");

                        thread::sleep(Duration::from_secs(1));

                        break 'game_loop;
                    }
                    else
                    {
                        barrel_to_load += 1;

                        if barrel_to_load == 6
                        {
                            println!();
                            println!("You survived all the rounds!");

                            thread::sleep(Duration::from_secs(1));
                            
                            println!();
                            println!("You win");

                            thread::sleep(Duration::from_secs(1));

                            break 'game_loop;
                        }
                        else
                        {
                            println!();
                            println!("Click!");

                            thread::sleep(Duration::from_secs(1));
                            
                            continue;
                        }
                    }
                }
                else
                {
                    println!();
                    println!("Invalid input please try again");

                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
        else if play_input == "2" 
        {
            break 'menu_loop;
        }
        else 
        {
            println!();
            println!("Invalid input please try again");

            thread::sleep(Duration::from_secs(1));
        }
    }
}

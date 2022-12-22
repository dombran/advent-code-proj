
mod str_mass_2;

fn main() {
    println!("Hello, world!");
    
    let spl_str = str_mass_2::s_sdv.split("\n");
    //let spl_str = "\nA Y\nB X\nC Z\n".split("\n");
    
    let mut sum: u32 = 0;
    
    let mut dum: u32 = 0;
    
    for db in spl_str{
		
		let spl_ax: Vec<&str> = db.split(' ').collect();
		
		
		if spl_ax.len() >= 2 {
			
			
			match spl_ax[1]{
				"X" => sum += 1,	// A - rock
				"Y" => sum += 2,	// B - paper
				"Z" => sum += 3,	// C - scissors
				_ => println!("ERR def"),		
			}
			
			if spl_ax[0] == "A" && spl_ax[1] == "Y"{
				sum += 6;
			}else if spl_ax[0] == "A" && spl_ax[1] == "Z"{
				sum += 0;
			}else if spl_ax[0] == "B" && spl_ax[1] == "X"{
				sum += 0;
			}else if spl_ax[0] == "B" && spl_ax[1] == "Z"{
				sum += 6;
			}else if spl_ax[0] == "C" && spl_ax[1] == "X"{
				sum += 6;
			}else if spl_ax[0] == "C" && spl_ax[1] == "Y"{
				sum += 0;
			}else{
				sum += 3;				
			}
			
			// second part
			
			if spl_ax[0] == "A" && spl_ax[1] == "X"{
				dum += (3+0);
			}else if spl_ax[0] == "A" && spl_ax[1] == "Y"{
				dum += (1+3);
			}else if spl_ax[0] == "A" && spl_ax[1] == "Z"{
				dum += (2+6);
			}else if spl_ax[0] == "B" && spl_ax[1] == "X"{
				dum += (1+0);
			}else if spl_ax[0] == "B" && spl_ax[1] == "Y"{
				dum += (2+3);
			}else if spl_ax[0] == "B" && spl_ax[1] == "Z"{
				dum += (3+6);
			}else if spl_ax[0] == "C" && spl_ax[1] == "X"{
				dum += (2+0);
			}else if spl_ax[0] == "C" && spl_ax[1] == "Y"{
				dum += (3+3);
			}else if spl_ax[0] == "C" && spl_ax[1] == "Z"{
				dum += (1+6);
			}
			
		}
		
	}
	
	println!("sum -> {} : dum -> {}", sum, dum);
}

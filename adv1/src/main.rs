
mod str_mass_1;


fn main() {
    let mut iter_vec = Vec::new();
    println!("Hello, world!");
    
    str_mass_1::print_user_route();
    
    let spl_str = str_mass_1::s_adv1.split("\n");
    
    let mut prev_item_num = 0;
    let mut prev_num = 0;
    
    let mut this_item_num = 0;
    let mut this_num = 0;
        
    let mut num32 = 0;
    
       
    for db in spl_str{	
		
		if db!="" {
			num32 = db.parse::<u32>().unwrap();			
			
			this_num += num32;
			
		}else{
		
			iter_vec.push(this_num);
		
			if this_num > prev_num {
				prev_num = this_num;
				prev_item_num = this_item_num;
			}			
			
			this_num = 0;			
			this_item_num+=1;
		}
	}
	
	println!(" Max calories from Elve num { } : { } ", prev_item_num, prev_num);

	//iter_vec.sort();

	let mut max1 = 0;
	let mut max2 = 0;
	let mut max3 = 0;
	for db in iter_vec{
		
		if db > max1 {
			max3 = max2;
			max2 = max1;
			max1 = db;
		}else if db > max2{
			max3 = max2;
			max2 = db;
		} else if db > max3{
			max3 = db;
		}
		
	}
	
	println!(" Sum max calories { } ", max1+max2+max3);
	
}

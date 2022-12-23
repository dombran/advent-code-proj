
mod str_mass;



fn main() {
    println!("Hello, world!");
    
    let spl_str = str_mass::s_sdv.split("\n");
 /*   let mut spl_str = 
"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".split("\n");*/

	let mut sv: Vec<Vec<u8>> = vec![];

	let mut fb: bool = false;
	let mut sum :u32 = 0;
	for db in spl_str{
		let spl_ax: Vec<u8> = db.as_bytes().to_vec();
		sv.push(db.as_bytes().to_vec());
		
		fb = false;
		// find matches
		for i in 0..spl_ax.len()/2 {
			for j in spl_ax.len()/2..spl_ax.len() {
				
				if spl_ax[i] == spl_ax[j] {
					// a..z 1 - 26 / 97 - 122 
					// A..Z 27 - 52 / 65 - 90
					match spl_ax[i] {
						97u8..=122u8 => sum += u32::from(spl_ax[i])-96u32,
						65u8..=90u8 => sum += u32::from(spl_ax[i])-38u32,
						_ => println!("Out of range!"),
					}
					fb = true;
				}
				if fb{
					break;
				}
			}
			if fb {
				break;
			}
		}
	}
	
	println!("sum => {}", sum);
	
	sum = 0;	
	let mut grp:u8 = 0;
	for itr in (0..sv.len()-2).step_by(3){		
		
		for n1 in &sv[itr+0] {
			grp = 2;
			
			for n2 in &sv[itr+1]{
				if n1 == n2{
					grp -= 1;
					break;
				}
			}			
			for n3 in &sv[itr+2]{
				if n1 == n3{
					grp -= 1;
					break;
				}	
			}
			
			if grp == 0{
				match n1 {
					97u8..=122u8 => sum += u32::from(n1-96u8),
					65u8..=90u8 => sum += u32::from(n1-38u8),
					_ => println!("Out of range!"),
				}
				
				break;
			}
			
		}
		
	}
	println!("sum => {}", sum);
}




mod s_buf;

struct cord{
	x:usize,
	y:usize,
}
struct a_cord{
	el:bool,
	ne:usize,
	xy:cord,
}


fn main() {
    println!("Hello, world!");
    //let s_str = s_buf::s_sdv.split("\n");
    let mut spl_str = 
"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............".split("\n");

	let mut sv: Vec<Vec<u8>> = vec![];
	let mut elves: Vec<Vec<a_cord>> = vec![];
	
	for db in spl_str{
		sv.push(db.as_bytes().to_vec());				
	}
	
	let x_max = sv[0].len();
	let y_max = sv.len();

	// find all elves
	for iv in 0..y_max {	// y
		elves.push(Vec::new());
		for e in 0..x_max {	// x			
			if sv[iv][e] == 35 { // #
				elves[iv].push(a_cord { el:true, ne:0, xy:cord{x: 0, y: 0} });
			}else{
				elves[iv].push(a_cord { el:false, ne:0, xy:cord{x: 0, y: 0} });
			}			
		}
		
    }
    
    for i in 0..10 {
	
	    for y in 0..y_max {	// y
	    	for x in 0..x_max {	// x
	    	
	    		if elves[y][x].el == true {
					let mut go_var:u8 = 0;
					// find next steep
					if y>0 && x>0 && x<x_max-2 { // en north going
						if elves[y-1][x].el == false &&
							elves[y-1][x-1].el == false &&
							elves[y-1][x+1].el == false { // steep enable
								go_var |= 0x08;
							}
					}
					if y<y_max-2 && x>0 && x<x_max-2{ // en south going
						if elves[y+1][x].el == false &&
							elves[y+1][x-1].el == false &&
							elves[y+1][x+1].el == false { // steep enable
								go_var |= 0x04;
							}
					}
					if y>0 && y<y_max-2 && x>0{ // en west going
						if elves[y-1][x-1].el == false &&
							elves[y][x-1].el == false &&
							elves[y+1][x-1].el == false { // steep enable
								go_var |= 0x02;
							}
					}
					if y>0 && y<y_max-2 && x<x_max-2{ // en east going
						if elves[y-1][x+1].el == false &&
							elves[y][x+1].el == false &&
							elves[y+1][x+1].el == false { // steep enable
								go_var |= 0x01;
							}	
					}
					// roler for next steep
					if go_var&0x08 == 0 {
						if y>0 {
							if elves[y-1][x].ne == 0{
								elves[y][x].xy.x = x;
								elves[y][x].xy.y = y-1;
								elves[y-1][x].ne += 1;	
								
								go_var = 0;
							}else if go_var != 0x08 {
								go_var &= 0b00000111;
							}else {
								elves[y][x].xy.x = x;
								elves[y][x].xy.y = y-1;
								elves[y-1][x].ne += 1;	
								
								go_var = 0;
							}
						}
					}
					if go_var&0x04 == 0 {
						if y<y_max-1 {
							if elves[y+1][x].ne == 0 {
								elves[y][x].xy.x = x;
								elves[y][x].xy.y = y+1;
								elves[y+1][x].ne += 1;
		
								go_var = 0;
							}else if go_var != 0x04{
								go_var &= 0b00000011;
							}else {
								elves[y][x].xy.x = x;
								elves[y][x].xy.y = y+1;
								elves[y+1][x].ne += 1;
		
								go_var = 0;
							}
						}
					}
					if go_var&0x02 == 0 {
						if x>0 {
							if elves[y][x-1].ne == 0 {
								elves[y][x].xy.x = x-1;
								elves[y][x].xy.y = y;
								elves[y][x-1].ne += 1;
								
								go_var = 0;
							}else if go_var != 0x02 {
								go_var &= 0b00000001;
							}else {
								elves[y][x].xy.x = x-1;
								elves[y][x].xy.y = y;
								elves[y][x-1].ne += 1;
								
								go_var = 0;
							}
						}
					}
					if go_var&0x01 == 0 {
						if x<x_max-1 {
							if elves[y][x+1].ne == 0 {
								elves[y][x].xy.x = x+1;
								elves[y][x].xy.y = y;
								elves[y][x+1].ne += 1;
								
								go_var = 0;
							}else if go_var != 0x01 {
								go_var &= 0b00000000;
							}else {
								elves[y][x].xy.x = x+1;
								elves[y][x].xy.y = y;
								elves[y][x+1].ne += 1;
								
								go_var = 0;
							}
						}
					}				
					
				}
	    	
	    	}
	    }
    
	    for y in 0..y_max {	// y
	    	for x in 0..x_max {	// x
	    	
		    	if elves[y][x].el == true {
					let xx = elves[y][x].xy.x;
					let yy = elves[y][x].xy.y;
					if elves[yy][xx].ne == 1 {
						elves[yy][xx].el = true;
						elves[yy][xx].ne = 0;
						elves[yy][xx].xy.x = 0;
						elves[yy][xx].xy.y = 0;
						
						elves[y][x].el = false;
						elves[y][x].ne = 0;
						elves[y][x].xy.x = 0;
						elves[y][x].xy.y = 0;
					}
				}
				
	    	}
	    }
    
	    for y in 0..y_max {	// y
	    	for x in 0..x_max {	// x
		    	elves[y][x].ne = 0;
	    	}
	    }
	    
	    for y in 0..y_max {	// y
	    	let mut stri = String::from(" ");
		    for x in 0..x_max {
				if elves[y][x].el == true{
					stri.push('#');
				}else{
					stri.push('.');
				}
			}
			println!("{}", stri);
		}	    
    
    }
    
	    for y in 0..y_max {	// y
	    	let mut stri = String::from(" ");
		    for x in 0..x_max {
				if elves[y][x].el == true{
					stri.push('#');
				}else{
					stri.push('.');
				}
			}
			println!("{}", stri);
		}	
    
    
}

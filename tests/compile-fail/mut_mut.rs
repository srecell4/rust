#![feature(plugin)]
#![plugin(clippy)]

#[deny(mut_mut)]
fn fun(x : &mut &mut u32) -> bool { //~ERROR
	**x > 0
}

#[deny(mut_mut)]
#[allow(unused_mut, unused_variables)]
fn main() {
	let mut x = &mut &mut 1u32; //~ERROR
	{
		let mut y = &mut x; //~ERROR
	}
	
	if fun(x) {
		let y : &mut &mut &mut u32 = &mut &mut &mut 2; 
			 //~^ ERROR
                  //~^^ ERROR
                                  //~^^^ ERROR
									   //~^^^^ ERROR
		***y + **x;
	}
}

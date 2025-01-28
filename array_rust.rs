fn main()
{
	let week = ["Sun","Mon","Tue","Wed","Thu","Fri","Sat"];
	println!("{:?}",week);

	let array:[f64; 3] = [3.14, 1.6e-19, 6.02e23];
	println!("0:{}, 1:{}, 2:{}",array[0], array[1], array[2]);


//	println!("3:{}",array[3]);	// これは必ずエラーになる

	let mut blank:[i32;4]=[0;4];	//[0;4]は0個の4で配列を埋める
	blank[0] = 11;
	println!("{:?}",blank);

}

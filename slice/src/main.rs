// slice

fn main(){
    let numbers = [10, 20, 30, 40, 50];

    let slice=&numbers[1..4]; //20 30 40
    println!("slice:{:?}",slice);

    let slice2=&numbers[..3]; //10,20,30
    println!("slice:{:?}",slice2);

    let slice3=&numbers[3..]; //40,50
    println!("slice:{:?}",slice3);

    let slice4=&numbers[..]; //10 20 30 40 50
    println!("slice:{:?}",slice4);


    let mut numbers1 = [1, 2, 3, 4, 5];
    let slice = &mut numbers1[1..4];
    slice[0]=10;
    slice[1]=20;
    slice[2]=30;
    println!("Updated array: {:?}", numbers1);
}

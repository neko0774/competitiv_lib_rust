fn main(){
    let n: i64 = 4;
    for i in 0..(1 << n){
        let bit = (0..n)
        .map(|x| if i & (1<<x)>0 {1} else {0});
        
        for j in bit{
            //ここに処理を書く            
            //print!("{} ", j);


        }
        //println!();
    }
}
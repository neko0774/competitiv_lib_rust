fn bl(index: i64, &list: [i64]) -> i64{
    let mut lo = 0;
    let mut hi = list.len();
    while lo < hi{
        let mid = (lo+hi)/2;
        if list[mid as usize] < index{lo = mid+1;}
        else{hi = mid+1;}
    }
    return lo;
}

fn br(index: i64, &list: [i64]) -> i64{
    let mut lo = 0;
    let mut hi = list.len();
    while lo < hi{
        let mid = (lo+hi)/2;
        if list[mid as usize] > index{hi = mid;}
        else {lo = mid+1;}
    }
    return mid;
}

fn main(){
    let index = 12;
    let array = [1,5,23,1,42,13,53,53,3,21];
    let ans = br(index, array);
    println!("{}", ans);
}
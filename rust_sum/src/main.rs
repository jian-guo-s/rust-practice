fn total(list: &[u32])->Option<u32>{
    let mut  x =    list.iter();
    x.try_fold(0u32,|acc,&x| acc.checked_add(x))


}
fn main() {
    let list = [1,2,3,4,5,4294967295,4294967295,4294967295];
    let list1 = [1,2,3,4,5,4,4,4];
    let k = total(&list);
    let k1 = total(&list1);
    match k {
        Some(c)=>println!("{c}"),
        None =>{
            println!("溢出错误None")
        }
    }
    match k1 {
        Some(c)=>println!("和为{c}"),
        None =>{
            println!("None")
        }
    }
}

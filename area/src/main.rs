//定义一个特征 有一个打印面积函数
trait areaType{
    fn areaTypesPrint(&self);
}
//定义一个长方形的结构体
struct longBox {
    width:u64,
    long:u64,
}

//定义一个正方形形的结构体
struct squareBox {
    width:u64,
    long:u64,
}
//能打印面积的都是写areaType的特征
impl areaType for longBox{
    fn areaTypesPrint(&self){
        println!("area is（长方形的面积是） {}",self.long*self.width);
    }
}
impl areaType for squareBox{
    fn areaTypesPrint(&self){
        println!("area is（正方形的面积是） {}",self.long*self.width);
    }
}
//打印函数  T限定为 实现了可打印面积特征areaType
fn areaPtint<T:areaType>(item:T){
    item.areaTypesPrint();
}
fn main() {
    let longBox1 = longBox {width:30,long:20};
    let squareBox1 = squareBox{width:20,long:20};
    areaPtint(longBox1);
    areaPtint(squareBox1);
}

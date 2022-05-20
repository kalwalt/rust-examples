use std::fs;

fn arParamLoad(filename: String, num: i32)
{
    let mut ret: i8 = 0;
    if num < 1  {
        ret = -1;
    }
    let info = fs::read(filename).expect("The file could not be read");
    println!("{}", info.len());
    println!("{:?}", info);
}

fn main() {
    println!("Reading a file");
    arParamLoad("camera_para.dat".to_string(), 1);
}

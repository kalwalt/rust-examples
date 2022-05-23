use std::fs;

const AR_DIST_FUNCTION_VERSION_MAX: usize = 4;

struct arParamVersionInfo_t {
    dist_factor_num: usize,
    ARParam_size: usize,
}

impl arParamVersionInfo_t {
    fn arParamVersionInfo(&self, index: usize) -> [u8; 2] {
        let mut i: usize = 0;
        let mut array: [[u8; 2]; AR_DIST_FUNCTION_VERSION_MAX] = [[0, 0], [0, 0], [0, 0], [0, 0]];
        array[0] = [4, 136];
        array[1] = [5, 144];
        array[2] = [6, 152];
        array[3] = [9, 176];
        array[index]
    }
}
//const arParamVersionInfo_t arParamVersionInfo[AR_DIST_FUNCTION_VERSION_MAX];

fn arParamLoad(filename: String, num: i32) {
    let mut ret: i8 = 0;
    let mut dist_function_version: usize = 0;
    let flen: usize;
    let mut i: usize = 0;
    if num < 1 {
        ret = -1;
    }
    let info = fs::read(filename).expect("The file could not be read");
    flen = info.len();
    println!("{}", flen);
    println!("{:?}", info);
    while i < AR_DIST_FUNCTION_VERSION_MAX {
        let ar_param_vi = arParamVersionInfo_t {
            dist_factor_num: 0,
            ARParam_size: 0,
        };
        if flen as u8 % ar_param_vi.arParamVersionInfo(i)[1] == 0 {
            dist_function_version = i + 1;
            break;
        }
        i += 1;
    }

    if i == AR_DIST_FUNCTION_VERSION_MAX {
        println!("Error: supplied buffer does not appear to be ARToolKit camera parameters.\n");
    }

    println!(
        "Reading camera parameters from buffer (distortion function version {}).\n",
        dist_function_version
    );
}

fn main() {
    println!("Reading a file");
    arParamLoad("camera_para.dat".to_string(), 1);
}

use std::fs;

const AR_DIST_FUNCTION_VERSION_MAX: usize = 4;
const AR_DIST_FACTOR_NUM_MAX: usize = 9;

struct arParamVersionInfo_t {
    dist_factor_num: usize,
    ARParam_size: usize,
}

struct ARParamd {
    xsize: i32,
    ysize: i32,
    mat: [[f64; 3];  4],
    dist_factor: [f64; AR_DIST_FACTOR_NUM_MAX],
    dist_function_version: i32,
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

fn arParamLoad(filename: String, num: i32) {
    let mut ret: i8 = 0;
    let mut dist_function_version: usize = 0;
    let flen: usize;
    let mut i: usize = 0;
    let mut dst: [u8; 176] = [0; 176];
    if num < 1 {
        ret = -1;
    }
    let info = fs::read(filename).expect("The file could not be read");
    flen = info.len();
    println!("{}", flen);
    println!("{:?}", info);
    let ar_param_vi = arParamVersionInfo_t {
        dist_factor_num: 0,
        ARParam_size: 0,
    };
    while i < AR_DIST_FUNCTION_VERSION_MAX {
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
    let slice = ar_param_vi.arParamVersionInfo(dist_function_version - 1)[1];
    println!("slice is: {}", slice);
    dst.copy_from_slice(&info[slice as usize..]);
}

fn main() {
    println!("Reading a file");
    arParamLoad("camera_para.dat".to_string(), 1);
}

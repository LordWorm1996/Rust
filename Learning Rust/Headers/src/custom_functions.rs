
pub fn intMax (num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        return num1;
    } else {
        return num2;
    }
}

pub fn intMin (num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        return num1;
    } else {
        return num2;
    }
}

pub fn floatMax (num1: f64, num2: f64) -> f64 {
    if num1 > num2 {
        return num1;
    } else {
        return num2;
    }
}

pub fn floatMin (num1: f64, num2: f64) -> f64 {
    if num1 > num2 {
        return num1;
    } else {
        return num2;
    }
}

pub fn intArrayMax (arr: &[i32]) -> i32 {
    let mut tmp = arr[0];
    for &i in arr.iter() {
        if i > tmp {
            tmp = i;
        }
    }
    return tmp;
}

pub fn intArrayMin (arr: &[i32]) -> i32 {
    let mut tmp = arr[0];
    for &i in arr.iter() {
        if i < tmp {
            tmp = i;
        }
    }
    return tmp;
}

pub fn floatArrayMax (arr: &[f64]) -> f64 {
    let mut tmp = arr[0];
    for &i in arr.iter() {
        if i > tmp {
            tmp = i;
        }
    }
    return tmp;
}

pub fn floatArrayMin (arr: &[f64]) -> f64 {
    let mut tmp = arr[0];
    for &i in arr.iter() {
        if i < tmp {
            tmp = i;
        }
    }
    return tmp;
}
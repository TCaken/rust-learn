fn main() {
    //Integers
    let _i1  = 5;
    let _i2 : isize = -6;
    let _i3 : u32 = 90000;

    //Floating points
    let _f1 = 0.9999;
    let _f2 : f64 = 1.0;

    //Char (Single quoation)
    let _ch1 = 'z';
    let _ch2 : char = 'Q';

    //Boolean
    let _b1 = true;
    let _b2 : bool = false;

    //Compound data types
    //Tuples
    //can consists of differents data types
    //can be destructured
    let t1 = (9, "abc", false);
    println!("Tuples: {}, {}, {}", t1.0, t1.1, t1.2);

    //Array
    //fixed length collection with same data types
    let _arr1 = [1,2,3,4,5];
    let _arr2 : [bool ; 5] = [false, false, false, false, false];

}

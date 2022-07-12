fn is_copy<T: Copy>() {}

fn types_impl_copy_trait(){
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // DST 类型不是 copy
    // is::<str>();
    // is_copy::<u8>();

    // 有堆内存的类型不是copy
    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // 可变引用不是 copy
    // is_copy::<&mut String>();

    // 对于数组/元组/，如果其内部类不是copy，那么它们也不是copy
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}
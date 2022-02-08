#[macro_export]
macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();
            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];
            unsafe { Box::from_raw(ptr) }
        }
        vec_to_boxed_array(vec![$val; $len])
    }};
}

pub(crate) use box_array;
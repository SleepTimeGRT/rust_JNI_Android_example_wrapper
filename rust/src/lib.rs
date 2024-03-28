use std::convert::TryFrom;
use std::ffi::{CStr, CString};

use jni::JNIEnv;
use jni::objects::{JObject, JObjectArray, JString};
use num_traits::{Num, Zero};
use shamir_secret_sharing::num_bigint::BigInt;
use zeroize::{Zeroize, Zeroizing};

#[allow(dead_code)]
mod lib_example;
mod lib_sss;

pub static FILES_PATH: &str = "/data/user/0/com.example.android/files/";

// Return a i32 from another i32
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returni32(
    _env: JNIEnv,
    _: JObject,
    int: i32,
) -> i32 {
    lib_example::int_times_3(int)
}

// Return a bool (true when a specific file in our folder exists)
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_getFileStatus(
    _env: JNIEnv,
    _: JObject,
) -> bool {
    lib_example::get_file_status(&FILES_PATH)
}

// Return JString from JString
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returnJString<'local>(
    mut env: JNIEnv<'local>,
    _: JObject<'local>,
    old_jstring: JString<'local>,
) -> JString<'local> {
    let ptr = env.get_string(&old_jstring).unwrap().as_ptr();
    // convert JString to CString and append it to our JNIEnv
    env.new_string(lib_example::return_string(
        CString::from(CStr::from_ptr(ptr))
            .to_str()
            .unwrap()
            .to_string(),
    ))
        .unwrap()
}

// Return Array from Vector from JString
#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_returnJArrayfromJString<'local>(
    mut env: JNIEnv<'local>,
    _: JObject<'local>,
    jstring_sentence: JString<'local>,
) -> JObjectArray<'local> {
    let new_vector = lib_example::split_string_into_words(
        CString::from(CStr::from_ptr(
            env.get_string(&jstring_sentence).unwrap().as_ptr(),
        ))
            .to_str()
            .unwrap()
            .to_string(),
    );
    // Initialize our array with the length of the vector. JArray(Length, Class, Initial Value)
    let class = env.find_class("java/lang/String").unwrap();
    let array = env
        .new_object_array(
            i32::try_from(new_vector.len()).unwrap(),
            class,
            env.new_string("").unwrap(),
        )
        .unwrap();

    let mut i = 0;
    // Edit every Item of the Array to give it the values we want
    for item in &new_vector {
        let jstring = env.new_string(item.to_owned()).unwrap();
        env.set_object_array_element(
            &array,
            i,
            jstring,
        )
            .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }
    array
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_split<'local>(
    mut env: JNIEnv<'local>,
    _: JObject<'local>,
    secret: JString<'local>,
) -> JObjectArray<'local> {
    let mut secret_string = env.get_string(&secret).unwrap();
    let secret_big_int = <BigInt as Num>::from_str_radix(secret_string.to_str().unwrap(), 16).unwrap();
    let mut shares = lib_sss::split(2, 3, secret_big_int);

    let class = env.find_class("java/lang/String").unwrap();
    let array = env
        .new_object_array(
            i32::try_from(shares.len()).unwrap(),
            class,
            env.new_string("").unwrap(),
        )
        .unwrap();

    let mut i = 0;
    // Edit every Item of the Array to give it the values we want
    for item in &shares {
        let j_string = env.new_string(item.to_owned()).unwrap();
        env.set_object_array_element(&array, i, j_string)
            .expect("Could not perform set_object_array_element on array element.");
        i += 1;
    }

    array
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_example_android_MainActivity_recover<'local>(
    mut env: JNIEnv<'local>,
    _: JObject<'local>,
    shares: JObjectArray<'local>,
) -> JString<'local> {

    let mut rust_shares = Vec::new();
    let array_length = env.get_array_length(&shares).unwrap();
    for i in 0..array_length {
        let mut java_object = env.get_object_array_element(&shares, i).unwrap();
        let mut java_string = JString::from(java_object);
        let mut j_string = env.get_string(&java_string).unwrap();
        let mut rust_string = j_string.to_str().unwrap();
        rust_shares.push(rust_string.to_owned());

    }

    let recover = lib_sss::recover(2, 3, rust_shares.as_slice());

    env.new_string(recover.to_str_radix(16)).unwrap()
}

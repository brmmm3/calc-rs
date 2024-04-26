use std::ffi::c_double;

// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::JClass;

use jni::sys::jlong;

use calclib::Calc;

/// # Safety
///
/// This function creates a new instance.
#[no_mangle]
pub unsafe extern "system" fn Java_JavaCalcTest_calcNew(_env: JNIEnv, _class: JClass) -> jlong {
    let calc = Calc::new(0.0);

    Box::into_raw(Box::new(calc)) as jlong
}

/// # Safety
///
/// This function adds a value.
#[no_mangle]
pub unsafe extern "system" fn Java_JavaCalcTest_calcAdd(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    mut _env: JNIEnv,
    _class: JClass,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    calc_ptr: jlong,
    value: c_double
) -> f64 {
    let calc = &mut *(calc_ptr as *mut Calc);

    calc.add(value)
}

/// # Safety
///
/// This function subtracts a value.
#[no_mangle]
pub unsafe extern "system" fn Java_JavaCalcTest_calcSub(
    mut _env: JNIEnv,
    _class: JClass,
    calc_ptr: jlong,
    value: c_double
) {
    let calc = &mut *(calc_ptr as *mut Calc);

    calc.sub(value);
}

/// # Safety
///
/// This function returns the current result.
#[no_mangle]
pub unsafe extern "system" fn Java_JavaCalcTest_calcResult(
    mut _env: JNIEnv,
    _class: JClass,
    calc_ptr: jlong
) -> f64 {
    let calc = &mut *(calc_ptr as *mut Calc);

    calc.result()
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_JavaCalcTest_calcDestroy(
    _env: JNIEnv,
    _class: JClass,
    calc_ptr: jlong
) {
    let _boxed_calc = Box::from_raw(calc_ptr as *mut Calc);
}

//
//  this file provide another wrapper around the "unsafe" wrapper of C library
//

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;

include!(concat!(env!("OUT_DIR"), "/auto.rs"));     //include the raw wrapper of the C library

static mut _callbacks: rsCallbacks = rsCallbacks{
    prnCallback : None,
    incCallback : None,
    incAmmount  : None
};

pub type rsPrintCallback = fn();
pub type rsIncreaseCallback = fn(ammount: &i32);

pub struct rsCallbacks
{
    pub prnCallback : Option<rsPrintCallback>,
    pub incCallback : Option<rsIncreaseCallback>,
    pub incAmmount : Option<i32>
}

pub fn rsAutoInitialize(callbacks: &rsCallbacks)
{
    unsafe
    {
        _callbacks.incCallback = callbacks.incCallback;
        _callbacks.prnCallback = callbacks.prnCallback;
        _callbacks.incAmmount = callbacks.incAmmount;

        let mut callbacks: CallbacksT = CallbacksT{
            prnCallback : Some(_printCallback),
            incCallback : Some(_increaseCallback),
            incAmmount : callbacks.incAmmount.unwrap()
        };

        let callbacks_raw_ptr: *mut CallbacksT = &mut callbacks as *mut CallbacksT;
        autoInitialize(callbacks_raw_ptr);
    }
}

unsafe extern "C" fn _printCallback()
{
    let prnCallback: rsPrintCallback = _callbacks.prnCallback.unwrap();
    prnCallback();
}

unsafe extern "C" fn _increaseCallback(ammount: *mut ::std::os::raw::c_int)
{
    let incCallback: rsIncreaseCallback = _callbacks.incCallback.unwrap();
    unsafe
    {
        let ammountRef: &i32 = mem::transmute(ammount);     //transporm a raw ptr into a ref
        incCallback(ammountRef);
    }
}
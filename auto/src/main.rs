#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod autoWrapper;

static mut accum: i32 = 0;

fn main()
{
    let callbacks: autoWrapper::rsCallbacks = autoWrapper::rsCallbacks{
        prnCallback : Some(myPrintCallback),
        incCallback : Some(myIncreaseCallback),
        incAmmount  : Some(20)
    };

    autoWrapper::rsAutoInitialize(&callbacks);
}

fn myPrintCallback()
{
    println!("Hello from Rust");
}

fn myIncreaseCallback(ammount: &i32)
{
    unsafe
    {
        accum += ammount;
        println!("Accum [{}]", accum);
    }
}


use std::{cell::OnceCell, sync::{Arc, OnceLock, RwLock}};

use futures::executor::block_on;
use jni::{objects::{ JClass, JObject, JPrimitiveArray, JString}, sys::{ jint, jintArray}, JNIEnv};
use jni::sys::{jstring, jdouble};

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_hello<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, name: JString<'local>) -> jstring{ //slower but safer
    if let Ok(out) = match env.get_string(&name){
        Ok(val) => (&env).new_string(format!("Hello {}!" , Into::<String>::into(val))),
        Err(_) => {
            let _ = env.throw_new("java/lang/RuntimeException", "JNI Get String Exception");
            return JString::default().into_raw()
        },
    }{
        return out.into_raw() //if a new_string is created return it
    }else{
        let _ = env.throw_new("java/lang/RuntimeException", "JNI New String Exception");
        return JString::default().into_raw()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_hello_unchecked<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, name: JString<'local>) -> jstring{ //faster but requires guarentee string is java.lang.String
    if let Ok(out) = match unsafe { env.get_string_unchecked(&name) }{
        Ok(val) => (&env).new_string(format!("Hello {}!" , Into::<String>::into(val))),
        Err(_) => {
            let _ = env.throw_new("java/lang/RuntimeException", "JNI Get String Exception");
            return JString::default().into_raw()
        },
    }{
        return out.into_raw() //if a new_string is created return it
    }else{
        let _ = env.throw_new("java/lang/RuntimeException", "JNI New String Exception");
        return JString::default().into_raw()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_double_array_sum<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, array: JPrimitiveArray<jdouble>) -> jdouble{
    let mut buffer:Vec<f64> = Vec::new();
    if env.get_double_array_region(&array, 0, &mut buffer).is_err(){
        //Throw error in java env
        let _ = env.throw_new("java/lang/RuntimeException", "JNI DoubleArray Region Eception");
        return -1.0;
    };
    buffer.iter().sum()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_double_array_squared<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, array: JPrimitiveArray<jdouble>){
    let mut buffer:Vec<f64> = Vec::new();
    if env.get_double_array_region(&array, 0, &mut buffer).is_err(){
        //Throw error in java env
        let _ = env.throw_new("java/lang/RuntimeException", "JNI DoubleArray Region Eception");
        return;
    };
    buffer.square();
    let _ = env.set_double_array_region(array, 0, &buffer);
}
#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_async_sieve_of_eratosthenes<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, cap: jint) -> jintArray{
    block_on(sieve_of_eratosthenes(&env, cap))
}
#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_async_all_prime_factors<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, number: jint) -> jintArray{
    block_on(all_prime_factors(&env, number))
}
#[unsafe(no_mangle)]
pub extern "system" fn Java_RustJNI_async_prime_factorization<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>, number: jint) -> jintArray{
    block_on(prime_factorization(&env, number))
}
trait Square{
    fn square(&mut self);
}
impl Square for Vec<f64> {
    fn square(&mut self) {
        for double in self.iter_mut() {
            *double *= *double;
        }
    }

}


static SIEVE: OnceLock<Arc<RwLock<SieveOfEratosthenes>>> = OnceLock::new();
fn get_sieve() -> Arc<RwLock<SieveOfEratosthenes>> {
    SIEVE.get_or_init(||Arc::new(RwLock::new(SieveOfEratosthenes::default()))).clone()
}

struct SieveOfEratosthenes{
    pub primes: Vec<i32>,
    pub last_known_cap: i32
}

impl Default for SieveOfEratosthenes {
    fn default() -> Self {
        Self { primes: Vec::new(), last_known_cap: 0 }
    }
}


async fn sieve_of_eratosthenes<'local>(env: &JNIEnv<'local>, cap: i32) -> jintArray{
    todo!()
}

async fn all_prime_factors<'local>(env: &JNIEnv<'local>, number: i32) -> jintArray{
    let factors : Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(Vec::new()));
    
    todo!()
}
async fn prime_factorization<'local>(env: &JNIEnv<'local>, number: i32) -> jintArray{
    let factors : Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(Vec::new()));
    
    todo!()
}
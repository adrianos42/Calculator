use idl_internal::{StreamInstance, StreamReceiver, StreamSender};
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, thread::sleep, time::Duration};

#[macro_use]
extern crate idl_gen;

mod calc;
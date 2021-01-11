use crate::idl_impl::*;
use crate::idl_types::*;
use idl_internal::*;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, thread::sleep, time::Duration};

pub(crate) struct Programmer {
    stream_instances: Arc<RwLock<i64>>,
    stream_subscriber: Option<Box<dyn StreamInstance>>,
}

impl ProgrammerInstance for Programmer {
    fn commands(
        &mut self,
        value: Box<dyn StreamInstance>,
        stream_instance: Box<dyn StreamInstance>,
    ) {
        let context = self.stream_instances.clone();
        value.wake_client();
        self.stream_subscriber = Some(value);

        // std::thread::spawn(move || {
        //     let mut prev = 0;
        //     loop {
        //         sleep(Duration::from_millis(600));
        //         *context.write().unwrap() = prev;
        //         stream_instance.wake_client();
        //         if prev > 5 {
        //             return;
        //         }
        //         prev += 1;
        //     }
        // });
    }

    fn commands_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamReceiver,
    ) -> StreamSender<i64> {
        match stream {
            StreamReceiver::Request => {
                let value = *self.stream_instances.read().unwrap();
                if value <= 5 {
                    StreamSender::Value(value)
                } else {
                    StreamSender::Done
                }
            }
            StreamReceiver::Close => StreamSender::Ok,
            StreamReceiver::Pause => StreamSender::Ok,
            StreamReceiver::Resume => StreamSender::Ok,
            _ => panic!()
        }
    }

    fn commands_stream_sender(
        &mut self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamSender<i64>,
    ) -> StreamReceiver {
        match stream {
            StreamSender::Request => StreamReceiver::Start,
            StreamSender::Value(value) => {
                println!("from client {}", value + 100);
                StreamReceiver::Ok
            }
            StreamSender::Done => {
                println!("done");
                StreamReceiver::Ok
            }
            _ => panic!(),
        }
    }
}

impl Programmer {
    pub(crate) fn new() -> Self {
        Self {
            stream_instances: Default::default(),
            stream_subscriber: None,
        }
    }
}

impl ProgrammerStatic for Programmer {
    fn commandss(&self, value: Box<dyn StreamInstance>, stream_instance: Box<dyn StreamInstance>) {
        todo!()
    }

    fn commandss_stream_sender(
        &self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamSender<i64>,
    ) -> StreamReceiver {
        todo!()
    }

    fn commandss_stream(
        &self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamReceiver,
    ) -> StreamSender<i64> {
        todo!()
    }
}
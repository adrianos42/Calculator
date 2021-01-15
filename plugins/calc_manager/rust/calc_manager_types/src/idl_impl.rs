use idl_internal::{StreamInstance, StreamReceiver, StreamSender};
pub trait ProgrammerInstance {
    fn commands(
        &mut self,
        value: Box<dyn StreamInstance>,
        stream_instance: Box<dyn StreamInstance>,
    );
    fn commands_stream_sender(
        &mut self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamSender<i64>,
    ) -> StreamReceiver;
    fn commands_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance>,
        stream: StreamReceiver,
    ) -> StreamSender<i64>;
}

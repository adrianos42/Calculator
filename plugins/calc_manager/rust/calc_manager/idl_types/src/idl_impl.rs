use idl_internal::{StreamInstance, StreamReceiver, StreamSender};
pub trait ProgrammerInstance {
    fn commands(
        &mut self,
        value: Box<dyn StreamInstance + Send>,
        stream_instance: Box<dyn StreamInstance + Send>,
    );
    fn commands_stream_sender(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamSender<i64>,
    ) -> StreamReceiver;
    fn commands_stream(
        &mut self,
        stream_instance: Box<dyn StreamInstance + Send>,
        stream: StreamReceiver,
    ) -> StreamSender<i64>;
    fn shaa(&mut self, name: (String, i64)) -> (i64, String);
}

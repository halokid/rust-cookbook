

#[derive(Clone, Debug)]
pub struct MessageNotification {
  pub no_messages: u64,
  pub device: IoTDevice,
}

impl MessageNotification {
  pub fn new(device: IoTDevice, no_messages: u64) -> MessageNotification {
    MessageNotification {
      no_messages: no_messages,
      device: device,
    }
  }
}



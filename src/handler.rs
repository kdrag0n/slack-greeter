use slack::{EventHandler, RtmClient, Event, Message};
use slack::api::MessageChannelJoin;

pub struct Handler;

impl EventHandler for Handler {
    fn on_event(&mut self, client: &RtmClient, event: Event) {
        debug!("Received event: {:?}", event);

        match event {
            Event::Message(message) => self.handle_message(client, *message),
            Event::MessageError(error) => error!("Error sending message: {:?}", error),
            _ => {}
        }
    }

    fn on_close(&mut self, _client: &RtmClient) {
        info!("Stopped");
    }

    fn on_connect(&mut self, _client: &RtmClient) {
        info!("Connected");
    }
}

impl Handler {
    fn get_greeting(&mut self) -> &str {
        return "Hi"
    }

    fn handle_message(&mut self, client: &RtmClient, message: Message) {
        match message {
            Message::ChannelJoin(join_msg) => self.handle_join(client, join_msg),
            _ => {}
        }
    }

    fn handle_join(&mut self, client: &RtmClient, join_msg: MessageChannelJoin) {
        let user = join_msg.user.as_ref().unwrap();
        let self_user = client.start_response().slf.as_ref().unwrap().id.as_ref().unwrap();
        if user == self_user {
            debug!("Join event from ourself, ignoring");
            return
        }

        let channel = client.start_response().ims
            .as_ref()
            .unwrap()
            .iter()
            .find(|im| im.user == Some(user.to_string()))
            .unwrap()
            .id
            .as_ref()
            .unwrap();
        match client.sender().send_message(&channel, self.get_greeting()) {
            Ok(_) => {}
            Err(err) => error!("Error greeting user {}: {:?}", user, err)
        }
    }
}

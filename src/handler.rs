use slack::{EventHandler, RtmClient, Event, Message};
use slack::api::MessageChannelJoin;
use rand::seq::SliceRandom;

pub struct Handler {
    pub greetings: Vec<String>
}

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
    fn get_greeting(&mut self, client: &RtmClient, user_id: &str) -> Option<String> {
        let start_response = client.start_response();
        let user = start_response
            .users
            .as_ref()?
            .iter()
            .find(|u| u.id.as_ref().unwrap() == user_id)?;

        let template = self.greetings.choose(&mut rand::thread_rng())?;
        let workspace = start_response.team.as_ref()?.name.as_ref()?;
        let name = user.real_name.as_ref()?;
        let mention = &format!("<@{}>", user_id);

        Some(template.replace("{workspace}", workspace)
            .replace("{name}", name)
            .replace("{mention}", mention))
    }

    fn handle_message(&mut self, client: &RtmClient, message: Message) {
        match message {
            Message::ChannelJoin(join_msg) => {
                let success = self.handle_join(client, join_msg);
                if success.is_none() {
                    error!("Error handling join event");
                }
            },
            _ => {}
        }
    }

    fn handle_join(&mut self, client: &RtmClient, join_msg: MessageChannelJoin) -> Option<()> {
        let user = join_msg.user.as_ref()?;
        let self_user = client.start_response().slf.as_ref()?.id.as_ref()?;
        if user == self_user {
            debug!("Join event from ourself, ignoring");
            return Some(())
        }

        let channel = client.start_response().ims
            .as_ref()?
            .iter()
            .find(|im| im.user == Some(user.to_string()))?
            .id
            .as_ref()?;
        let greeting = self.get_greeting(client, user)?;

        match client.sender().send_message(&channel, &greeting) {
            Ok(_) => Some(()),
            Err(err) => {
                error!("Error greeting user {}: {:?}", user, err);
                None
            }
        }
    }
}

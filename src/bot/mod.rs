use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{Channel, Message},
        prelude::Ready,
    },
    Client,
};

use crate::channel::{get_bot_channel, get_ws_channel, receive, send};

const CHANNEL_ID: u64 = 829304052189757471;

pub async fn start() {
    let token = env::var("DISCORD_TOKEN").expect("cade o token mlk");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("cliente deu erro foda");
    client.start().await.expect("cliente morreu fodase");
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        println!("{} online.", data_about_bot.user.tag());
        let channel = ctx
            .http
            .get_channel(CHANNEL_ID)
            .await
            .expect("cade a lixeira?????");

        if let Channel::Guild(channel) = channel {
            let webhooks = channel.webhooks(&ctx).await.expect("cade os webhoko?");
            let webhook = match webhooks.first() {
                Some(webhook) => webhook.clone(),
                None => channel
                    .create_webhook(&ctx, "falsidades")
                    .await
                    .expect("morreu webhook, pls"),
            };
            loop {
                let data = receive(&get_bot_channel());
                webhook
                    .execute(&ctx, false, |w| {
                        w.avatar_url(data.author.avatar_url)
                            .username(data.author.username)
                            .content(data.content)
                    })
                    .await
                    .expect("rip mensagem :pensive:");
            }
        }
    }

    async fn message(&self, _ctx: Context, new_message: Message) {
        if new_message.channel_id == CHANNEL_ID {
            send(&get_ws_channel(), new_message.into());
        }
    }
}

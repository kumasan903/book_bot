use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
	// Set a handler for the `message` event - so that whenever a new message
	// is received - the closure (or function) passed will be called.
	//
	// Event handlers are dispatched through a threadpool, and so multiple
	// events can be dispatched simultaneously.
	async fn message(&self, ctx: Context, msg: Message)
	{
		if msg.author.bot
		{
			return ;
		}
		if msg.content.starts_with("(FPL")
		{
			let author = msg.author.name.to_string();
			let raw_fpl_text = msg.content;
			let raw_lines:Vec<&str> = raw_fpl_text.split('\n').collect();
			let callsign:Vec<&str> = raw_lines[0].split('-').collect();
			let callsign = callsign[1];
			let aircraft_type = raw_lines[1].to_string();
			let aircraft_type = &aircraft_type[1..7];
			let from = raw_lines[2].to_string();
			let from = &from[1..5];
			let to = raw_lines[4];
			let to = &to[1..5];
			let mut route = raw_lines[3].to_string();
			for _ in 1..12
			{
				route.remove(0);
			}
			let cruise_altitude:Vec<&str> = raw_lines[3].split(' ').collect();
			let mut cruise_altitude = cruise_altitude[0].to_string();
			for _ in 1..8
			{
				cruise_altitude.remove(0);
			}
			let mut eobt = raw_lines[2].to_string();
			for _ in 1..6
			{
				eobt.remove(0);
			}
			let response_message = format!("パイロット　：{author}\nコールサイン：{callsign}\n出発地　　　：{from}\n到着地　　　：{to}\n巡航高度　　：FL{cruise_altitude}\nルート　　　：`{route}`\n機体　　　　：{aircraft_type}\nEOBT   　　　：{eobt}z\n");
			if let Err(why) = msg.channel_id.say(&ctx.http, response_message).await
			{
				println!("Error sending message: {:?}", why);
			}
		}
	}

	// Set a handler to be called on the `ready` event. This is called when a
	// shard is booted, and a READY payload is sent by Discord. This payload
	// contains data like the current user's guild Ids, current user data,
	// private channels, and more.
	//
	// In this case, just print what the current user's username is.
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main()
{
	// Configure the client with your Discord bot token in the environment.
	let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
	// Set gateway intents, which decides what events the bot will be notified about
	let intents = GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::DIRECT_MESSAGES
		| GatewayIntents::MESSAGE_CONTENT;

	// Create a new instance of the Client, logging in as a bot. This will
	// automatically prepend your bot token with "Bot ", which is a requirement
	// by Discord for bot users.
	let mut client =
		Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

	// Finally, start a single shard, and start listening to events.
	//
	// Shards will automatically attempt to reconnect, and will perform
	// exponential backoff until it reconnects.
	if let Err(why) = client.start().await
	{
		println!("Client error: {:?}", why);
	}
}

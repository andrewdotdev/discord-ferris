use reqwest::Client as ReqClient;
use reqwest::header::AUTHORIZATION;
use serde::Serialize;

// Keep base URL as a constant; avoid storing it per-instance.
const DISCORD_API_BASE: &str = "https://discord.com/api/v10";

pub struct Http {
    token: String,
    client: ReqClient,
}

// Payload types live at module scope (not inside impl).
#[derive(Debug, Serialize)]
struct CreateMsg<'a> {
    content: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_reference: Option<MessageRef<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug, Serialize)]
struct MessageRef<'a> {
    message_id: &'a str,
    // Channel is optional; include to be explicit.
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct AllowedMentions {
    // Minimal control: only toggle reply ping.
    replied_user: bool,
}

impl Http {
    pub fn new(token: impl Into<String>) -> Self {
        // One shared client; connection pooling by default.
        let client = ReqClient::builder()
            .user_agent("discord-ferris (github.com/andrewdotdev/discord-ferris)")
            .build()
            .expect("failed to build HTTP Client");

        Self {
            token: token.into(),
            client,
        }
    }

    /// POST /channels/{channel_id}/messages
    /// Uses typed payload to avoid building ad-hoc JSON maps.
    pub async fn send_message(
        &self,
        channel_id: &str,
        content: &str,
        reply_to: Option<&str>,
        mention_replied_user: bool,
    ) -> anyhow::Result<()> {
        let url = format!("{}/channels/{}/messages", DISCORD_API_BASE, channel_id);

        let body = CreateMsg {
            content,
            message_reference: reply_to.map(|mid| MessageRef {
                message_id: mid,
                channel_id: Some(channel_id),
            }),
            allowed_mentions: reply_to.map(|_| AllowedMentions {
                replied_user: mention_replied_user,
            }),
        };

        let resp = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bot {}", self.token))
            .json(&body)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("discord http error {}: {}", status, text))
        }
    }
}

use reqwest::Client as ReqClient;
use serde_json::json;

pub struct Http {
    token: String,
    client: ReqClient,
    base: String,
}

impl Http {
    pub fn new(token: impl Into<String>) -> Self {
        let client = ReqClient::builder()
            .user_agent("discord-ferris (github.com/andrewdotdev/discord-ferris)")
            .build()
            .expect("failed to build HTTP Client");

        Self {
            token: token.into(),
            client,
            base: "https://discord.com/api/v10".to_string(),
        }
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        content: &str,
        reply_to: Option<&str>,
        mention_replied_user: bool,
    ) -> anyhow::Result<()> {
        let url = format!("{}/channels/{}/messages", self.base, channel_id);

        let mut body = json!({ "content": content });
        if let Some(mid) = reply_to {
            body["message_reference"] = json!({ "message_id": mid, "channel_id": channel_id });
            body["allowed_mentions"] = json!({ "replied_user": mention_replied_user });
        }

        let resp = self.client
            .post(url)
            .header(reqwest::header::AUTHORIZATION, format!("Bot {}", self.token))
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

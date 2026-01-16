use clap::Parser;
use wezterm_client::client::Client;

#[derive(Debug, Parser, Clone)]
pub struct WebOpen {
    /// The URL to open
    url: String,
}

impl WebOpen {
    pub async fn run(&self, client: Client) -> anyhow::Result<()> {
        let pane_id = client.resolve_pane_id(None).await?;
        let response = client
            .web_open(codec::WebOpen {
                pane_id,
                url: self.url.clone(),
            })
            .await?;
        println!("{}", response.message);
        Ok(())
    }
}

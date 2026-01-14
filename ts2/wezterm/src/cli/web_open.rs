use clap::Parser;
use mux::pane::PaneId;
use wezterm_client::client::Client;

#[derive(Debug, Parser, Clone)]
pub struct WebOpen {
    /// Specify the target pane.
    /// The default is to use the current pane based on the
    /// environment variable WEZTERM_PANE.
    #[arg(long)]
    pane_id: Option<PaneId>,

    /// The URL to open
    url: String,
}

impl WebOpen {
    pub async fn run(&self, client: Client) -> anyhow::Result<()> {
        let pane_id = client.resolve_pane_id(self.pane_id).await?;
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

use super::Service;

pub struct FirewallService {}

impl FirewallService {}

#[async_trait::async_trait]
impl Service for FirewallService {
    async fn exec(&mut self) {}
}

// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::net::SocketAddr;

use anyhow::Result;
use bitcoin_hashes::sha256::Hash;
use nostr::key::XOnlyPublicKey;
use nostr::{Contact, Event, Keys, Metadata, SubscriptionFilter, Tag};
use tokio::sync::broadcast;
use url::Url;

use crate::client::Entity;
use crate::relay::pool::RelayPoolNotifications;
use crate::RUNTIME;

pub struct Client {
    client: super::Client,
}

impl Client {
    pub fn new(keys: &Keys) -> Self {
        Self {
            client: super::Client::new(keys),
        }
    }

    pub fn generate_keys() -> Keys {
        super::Client::generate_keys()
    }

    pub fn notifications(&self) -> broadcast::Receiver<RelayPoolNotifications> {
        self.client.notifications()
    }

    pub fn add_relay(&mut self, url: &str, proxy: Option<SocketAddr>) -> Result<()> {
        self.client.add_relay(url, proxy)
    }

    pub fn remove_relay(&mut self, url: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.remove_relay(url).await })
    }

    pub fn connect_relay(&mut self, url: &str, wait_for_connection: bool) -> Result<()> {
        RUNTIME.block_on(async { self.client.connect_relay(url, wait_for_connection).await })
    }

    pub fn disconnect_relay(&mut self, url: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.disconnect_relay(url).await })
    }

    pub fn connect(&mut self) -> Result<()> {
        RUNTIME.block_on(async { self.client.connect().await })
    }

    pub fn connect_and_wait(&mut self) -> Result<()> {
        RUNTIME.block_on(async { self.client.connect_and_wait().await })
    }

    pub fn disconnect(&mut self) -> Result<()> {
        RUNTIME.block_on(async { self.client.disconnect().await })
    }

    pub fn subscribe(&mut self, filters: Vec<SubscriptionFilter>) -> Result<()> {
        RUNTIME.block_on(async { self.client.subscribe(filters).await })
    }

    pub fn get_events_of(&mut self, filters: Vec<SubscriptionFilter>) -> Result<Vec<Event>> {
        RUNTIME.block_on(async { self.client.get_events_of(filters).await })
    }

    /// Send event
    pub fn send_event(&self, event: Event) -> Result<()> {
        RUNTIME.block_on(async { self.client.send_event(event).await })
    }

    pub fn update_profile(&self, metadata: Metadata) -> Result<()> {
        RUNTIME.block_on(async { self.client.update_profile(metadata).await })
    }

    pub fn publish_text_note(&self, content: &str, tags: &[Tag]) -> Result<()> {
        RUNTIME.block_on(async { self.client.publish_text_note(content, tags).await })
    }

    pub fn publish_pow_text_note(&self, content: &str, tags: &[Tag], difficulty: u8) -> Result<()> {
        RUNTIME.block_on(async {
            self.client
                .publish_pow_text_note(content, tags, difficulty)
                .await
        })
    }

    pub fn add_recommended_relay(&self, url: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.add_recommended_relay(url).await })
    }

    pub fn set_contact_list(&self, list: Vec<Contact>) -> Result<()> {
        RUNTIME.block_on(async { self.client.set_contact_list(list).await })
    }

    pub fn get_contact_list(&mut self) -> Result<Vec<Contact>> {
        RUNTIME.block_on(async { self.client.get_contact_list().await })
    }

    pub fn send_direct_msg(&self, recipient: &Keys, msg: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.send_direct_msg(recipient, msg).await })
    }

    pub fn delete_event(&self, event_id: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.delete_event(event_id).await })
    }

    pub fn like(&self, event: &Event) -> Result<()> {
        RUNTIME.block_on(async { self.client.like(event).await })
    }

    pub fn dislike(&self, event: &Event) -> Result<()> {
        RUNTIME.block_on(async { self.client.dislike(event).await })
    }

    pub fn new_channel(
        &self,
        name: &str,
        about: Option<&str>,
        picture: Option<&str>,
    ) -> Result<()> {
        RUNTIME.block_on(async { self.client.new_channel(name, about, picture).await })
    }

    pub fn update_channel(
        &self,
        channel_id: Hash,
        relay_url: Url,
        name: Option<&str>,
        about: Option<&str>,
        picture: Option<&str>,
    ) -> Result<()> {
        RUNTIME.block_on(async {
            self.client
                .update_channel(channel_id, relay_url, name, about, picture)
                .await
        })
    }

    pub fn send_channel_msg(&self, channel_id: Hash, relay_url: Url, msg: &str) -> Result<()> {
        RUNTIME.block_on(async {
            self.client
                .send_channel_msg(channel_id, relay_url, msg)
                .await
        })
    }

    pub fn hide_channel_msg(&self, message_id: Hash, reason: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.hide_channel_msg(message_id, reason).await })
    }

    pub fn mute_channel_user(&self, pubkey: XOnlyPublicKey, reason: &str) -> Result<()> {
        RUNTIME.block_on(async { self.client.mute_channel_user(pubkey, reason).await })
    }

    pub fn get_entity_of_pubkey(&self, pubkey: XOnlyPublicKey) -> Result<Entity> {
        RUNTIME.block_on(async { self.client.get_entity_of_pubkey(pubkey).await })
    }

    pub fn handle_notifications<F>(&self, func: F) -> Result<()>
    where
        F: Fn(RelayPoolNotifications) -> Result<()>,
    {
        RUNTIME.block_on(async { self.client.handle_notifications(func).await })
    }
}

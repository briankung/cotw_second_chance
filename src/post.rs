// Generated with https://transform.tools/json-to-rust-serde

use serde_derive::Deserialize;
use serde_derive::Serialize;

use time::OffsetDateTime;

use crate::actions_summary::ActionsSummary;
use crate::link_count::LinkCount;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub name: String,
    pub username: String,
    // #[serde(rename = "avatar_template")]
    // pub avatar_template: String,
    #[serde(rename = "created_at")]
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    pub cooked: String,
    // #[serde(rename = "post_number")]
    // pub post_number: i64,
    // #[serde(rename = "post_type")]
    // pub post_type: i64,
    // #[serde(rename = "updated_at")]
    // #[serde(with = "time::serde::rfc3339::option")]
    // pub updated_at: Option<OffsetDateTime>,
    // #[serde(rename = "reply_count")]
    // pub reply_count: i64,
    // #[serde(rename = "reply_to_post_number")]
    // pub reply_to_post_number: Value,
    // #[serde(rename = "quote_count")]
    // pub quote_count: i64,
    // #[serde(rename = "incoming_link_count")]
    // pub incoming_link_count: i64,
    // pub reads: i64,
    // #[serde(rename = "readers_count")]
    // pub readers_count: i64,
    // pub score: f64,
    // pub yours: bool,
    // #[serde(rename = "topic_id")]
    // pub topic_id: i64,
    // #[serde(rename = "topic_slug")]
    // pub topic_slug: String,
    // #[serde(rename = "display_username")]
    // pub display_username: String,
    // #[serde(rename = "primary_group_name")]
    // pub primary_group_name: Value,
    // #[serde(rename = "flair_name")]
    // pub flair_name: Value,
    // #[serde(rename = "flair_url")]
    // pub flair_url: Value,
    // #[serde(rename = "flair_bg_color")]
    // pub flair_bg_color: Value,
    // #[serde(rename = "flair_color")]
    // pub flair_color: Value,
    // pub version: i64,
    // #[serde(rename = "can_edit")]
    // pub can_edit: bool,
    // #[serde(rename = "can_delete")]
    // pub can_delete: bool,
    // #[serde(rename = "can_recover")]
    // pub can_recover: bool,
    // #[serde(rename = "can_wiki")]
    // pub can_wiki: bool,
    #[serde(rename = "link_counts")]
    #[serde(default)]
    pub link_counts: Vec<LinkCount>,
    // pub read: bool,
    // #[serde(rename = "user_title")]
    // pub user_title: Value,
    // pub bookmarked: bool,
    #[serde(rename = "actions_summary")]
    pub actions_summary: Vec<ActionsSummary>,
    // pub moderator: bool,
    // pub admin: bool,
    // pub staff: bool,
    // #[serde(rename = "user_id")]
    // pub user_id: i64,
    // pub hidden: bool,
    // #[serde(rename = "trust_level")]
    // pub trust_level: i64,
    // #[serde(rename = "deleted_at")]
    // #[serde(with = "time::serde::rfc3339::option")]
    // pub deleted_at: Option<OffsetDateTime>,
    // #[serde(rename = "user_deleted")]
    // pub user_deleted: bool,
    // #[serde(rename = "edit_reason")]
    // pub edit_reason: Value,
    // #[serde(rename = "can_view_edit_history")]
    // pub can_view_edit_history: bool,
    // pub wiki: bool,
    // #[serde(rename = "calendar_details")]
    // #[serde(default)]
    // pub calendar_details: Vec<Value>,
    // #[serde(rename = "can_accept_answer")]
    // pub can_accept_answer: bool,
    // #[serde(rename = "can_unaccept_answer")]
    // pub can_unaccept_answer: bool,
    // #[serde(rename = "accepted_answer")]
    // pub accepted_answer: bool,
    // #[serde(rename = "can_vote")]
    // #[serde(default)]
    // pub can_vote: bool,
}

impl Post {
    pub fn like_count(&self) -> i64 {
        if let Some(likes_action) = self.actions_summary.iter().find(|asum| asum.id == 2) {
            likes_action.count
        } else {
            0
        }
    }
}

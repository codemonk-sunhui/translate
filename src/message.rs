use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct Message
{
    #[serde(rename = "senderCorpId")]
    pub sender_corp_id: String,
    #[serde(rename = "sessionWebhook")]
    pub session_webhook: String,
    #[serde(rename = "isInAtList")]
    pub is_in_at_list: bool,
    #[serde(rename = "chatbotUserId")]
    pub chatbot_user_id: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "conversationType")]
    pub conversation_type: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "senderNick")]
    pub sender_nick: String,
    #[serde(rename = "atUsers")]
    pub at_users: Vec<DingtalkId>,
    pub msgtype: String,
    #[serde(rename = "senderStaffId")]
    pub sender_staff_id: String,
    #[serde(rename = "sessionWebhookExpiredTime")]
    pub session_webhook_expired_time: i64,
    #[serde(rename = "createAt")]
    pub create_at: i64,
    #[serde(rename = "robotCode")]
    pub robot_code: String,
    #[serde(rename = "conversationTitle")]
    pub conversation_title: String,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "chatbotCorpId")]
    pub chatbot_corp_id: String,
    pub text: Text,
    #[serde(rename = "msgId")]
    pub msg_id: String,
}


#[derive(Serialize, Deserialize)]
pub struct Text {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct DingtalkId {
    #[serde(rename = "dingtalkId")]
    pub dingtalk_id: String,
}
use super::Matcher;
use crate::api_resp;
use crate::event::SelfId;
use colored::*;
use tracing::{event, Level};

macro_rules! no_resp_api {
    ($fn_name: ident, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) {
            if let Some(bot) = &self.bot {
                bot.$fn_name($param).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
            }
        }
    };
    ($fn_name: ident, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) {
            if let Some(bot) = &self.bot {
                bot.$fn_name($($param,)*).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
            }
        }
    };
}

macro_rules! resp_api {
    ($fn_name: ident, $resp_data_type: ty) => {
        pub async fn $fn_name(&self) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name().await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
    ($fn_name: ident, $resp_data_type: ty, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name($param).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
    ($fn_name: ident, $resp_data_type: ty, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) -> Option<$resp_data_type> {
            if let Some(bot) = &self.bot {
                bot.$fn_name($($param,)*).await
            } else {
                event!(
                    Level::ERROR,
                    "Calling api {} {}",
                    stringify!($fn_name).blue(),
                    "with unbuilt matcher!".red()
                );
                None
            }
        }
    };
}

impl<E> Matcher<E>
where
    E: Clone + SelfId,
{
    /// 请求 Onebot Api，不等待 Onebot 返回
    pub async fn call_api(&self, api: crate::api::Api) {
        if let Some(bot) = &self.bot {
            bot.call_api(api).await;
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
        }
    }

    /// 请求 Onebot Api，等待 Onebot 返回项（30s 后 timeout 返回 None）
    pub async fn call_api_resp(&self, api: crate::api::Api) -> Option<crate::api_resp::ApiResp> {
        if let Some(bot) = &self.bot {
            bot.call_api_resp(api).await
        } else {
            event!(
                Level::ERROR,
                "{}",
                "Calling api with unbuilt matcher!".red()
            );
            None
        }
    }

    no_resp_api!(delete_msg, message_id: i32);
    no_resp_api!(send_like, user_id: String, times: u8);
    no_resp_api!(
        set_group_kick,
        group_id: String,
        user_id: String,
        reject_add_request: bool
    );
    no_resp_api!(
        set_group_ban,
        group_id: String,
        user_id: String,
        duration: i64
    );
    no_resp_api!(
        set_group_anonymous_ban,
        group_id: String,
        anonymous: crate::event::Anoymous,
        flag: String,
        duration: i64
    );
    no_resp_api!(set_group_whole_ban, group_id: String, enable: bool);
    no_resp_api!(
        set_group_admin,
        group_id: String,
        user_id: String,
        enable: bool
    );
    no_resp_api!(set_group_anonymous, group_id: String, enable: bool);
    no_resp_api!(
        set_group_card,
        group_id: String,
        user_id: String,
        card: String
    );
    no_resp_api!(set_group_name, group_id: String, group_name: String);
    no_resp_api!(set_group_leave, group_id: String, is_dismiss: bool);
    no_resp_api!(
        set_group_special_title,
        group_id: String,
        user_id: String,
        special_title: String,
        duration: i64
    );
    no_resp_api!(
        set_friend_add_request,
        flag: String,
        approve: bool,
        remark: String
    );
    no_resp_api!(
        set_group_add_request,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String
    );
    no_resp_api!(set_restart, delay: i64);

    resp_api!(
        send_msg,
        api_resp::MessageId,
        message_type: Option<String>,
        user_id: Option<String>,
        group_id: Option<String>,
        message: Vec<crate::Message>,
        auto_escape: bool
    );
    resp_api!(get_msg, api_resp::Message, message_id: i32);
    resp_api!(get_forward_msg, api_resp::Message, id: String);
    resp_api!(get_login_info, api_resp::LoginInfo);
    resp_api!(
        get_stranger_info,
        api_resp::StrangerInfo,
        user_id: String,
        no_cache: bool
    );
    resp_api!(get_friend_list, Vec<api_resp::FriendListItem>);
    resp_api!(
        get_group_info,
        api_resp::GroupInfo,
        group_id: String,
        no_cache: bool
    );
    resp_api!(get_group_list, Vec<api_resp::GroupListItem>);
    resp_api!(
        get_group_member_info,
        api_resp::GroupMemberInfo,
        group_id: String,
        user_id: String,
        no_cache: bool
    );
    resp_api!(
        get_group_member_list,
        Vec<api_resp::GroupMember>,
        group_id: String
    );
    resp_api!(
        get_group_honor_info,
        api_resp::GroupHonorInfo,
        group_id: String,
        ty: String
    );
    resp_api!(get_cookies, api_resp::Cookies, domain: String);
    resp_api!(get_csrf_token, api_resp::ScrfToken);
    resp_api!(get_credentials, api_resp::Credentials, domain: String);
    resp_api!(get_record, api_resp::File, file: String, out_format: String);
    resp_api!(get_image, api_resp::File, file: String);
    resp_api!(can_send_record, api_resp::SendCheck);
    resp_api!(can_send_image, api_resp::SendCheck);
    resp_api!(get_status, crate::event::Status);
    resp_api!(get_version_info, api_resp::VersionInfo);
}

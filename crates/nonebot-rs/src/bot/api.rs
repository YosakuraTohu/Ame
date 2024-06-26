use crate::{api, api_resp, RespData};

macro_rules! no_resp_api {
    ($fn_name: ident, $struct_name: tt, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) {
            self.call_api(api::Api::$fn_name(api::$struct_name { $param }))
                .await;
        }
    };
    ($fn_name: ident, $struct_name: tt, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) {
            self.call_api(api::Api::$fn_name(api::$struct_name {
                $($param,)*
            })).await;
        }
    };
}

macro_rules! resp_api {
    ($fn_name: ident,$resp_data: tt, $resp_data_type: ty) => {
        pub async fn $fn_name(&self) -> Option<$resp_data_type> {
            let resp = self.call_api_resp(api::Api::$fn_name()).await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $param: ident: $param_type: ty) => {
        pub async fn $fn_name(&self, $param: $param_type) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name { $param }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
    ($fn_name: ident, $struct_name: tt, $resp_data: tt, $resp_data_type: ty, $($param: ident: $param_type: ty),*) => {
        pub async fn $fn_name(&self, $($param: $param_type,)*) -> Option<$resp_data_type> {
            let resp = self
                .call_api_resp(api::Api::$fn_name(api::$struct_name {
                    $($param,)*
                }))
                .await;
            if let RespData::$resp_data(d) = resp.unwrap().data {
                Some(d)
            } else {
                None
            }
        }
    };
}

impl super::Bot {
    no_resp_api!(delete_msg, DeleteMsg, message_id: i32);
    no_resp_api!(send_like, SendLike, user_id: String, times: u8);
    no_resp_api!(
        set_group_kick,
        SetGroupKick,
        group_id: String,
        user_id: String,
        reject_add_request: bool
    );
    no_resp_api!(
        set_group_ban,
        SetGroupBan,
        group_id: String,
        user_id: String,
        duration: i64
    );
    no_resp_api!(
        set_group_anonymous_ban,
        SetGroupAnonymousBan,
        group_id: String,
        anonymous: crate::event::Anoymous,
        flag: String,
        duration: i64
    );
    no_resp_api!(
        set_group_whole_ban,
        SetGroupWholeBan,
        group_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_admin,
        SetGroupAdmin,
        group_id: String,
        user_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_anonymous,
        SetGroupAnonymous,
        group_id: String,
        enable: bool
    );
    no_resp_api!(
        set_group_card,
        SetGroupCard,
        group_id: String,
        user_id: String,
        card: String
    );
    no_resp_api!(
        set_group_name,
        SetGroupName,
        group_id: String,
        group_name: String
    );
    no_resp_api!(
        set_group_leave,
        SetGroupLeave,
        group_id: String,
        is_dismiss: bool
    );
    no_resp_api!(
        set_group_special_title,
        SetGroupSpecialTitle,
        group_id: String,
        user_id: String,
        special_title: String,
        duration: i64
    );
    no_resp_api!(
        set_friend_add_request,
        SetFriendAddRequest,
        flag: String,
        approve: bool,
        remark: String
    );
    no_resp_api!(
        set_group_add_request,
        SetGroupAddRequest,
        flag: String,
        sub_type: String,
        approve: bool,
        reason: String
    );
    no_resp_api!(set_restart, SetRestart, delay: i64);

    resp_api!(
        send_msg,
        SendMsg,
        MessageId,
        api_resp::MessageId,
        message_type: Option<String>,
        user_id: Option<String>,
        group_id: Option<String>,
        message: Vec<crate::Message>,
        auto_escape: bool
    );
    resp_api!(get_msg, GetMsg, Message, api_resp::Message, message_id: i32);
    resp_api!(
        get_forward_msg,
        GetForwardMsg,
        Message,
        api_resp::Message,
        id: String
    );
    resp_api!(get_login_info, LoginInfo, api_resp::LoginInfo);
    resp_api!(
        get_stranger_info,
        GetStrangerInfo,
        StrangerInfo,
        api_resp::StrangerInfo,
        user_id: String,
        no_cache: bool
    );
    resp_api!(get_friend_list, FriendList, Vec<api_resp::FriendListItem>);
    resp_api!(
        get_group_info,
        GetGroupInfo,
        GroupInfo,
        api_resp::GroupInfo,
        group_id: String,
        no_cache: bool
    );
    resp_api!(get_group_list, GroupList, Vec<api_resp::GroupListItem>);
    resp_api!(
        get_group_member_info,
        GetGroupMemberInfo,
        GroupMemberInfo,
        api_resp::GroupMemberInfo,
        group_id: String,
        user_id: String,
        no_cache: bool
    );
    resp_api!(
        get_group_member_list,
        GetGroupMemberList,
        GroupMemberList,
        Vec<api_resp::GroupMember>,
        group_id: String
    );
    resp_api!(
        get_group_honor_info,
        GetGroupHonorInfo,
        GroupHonorInfo,
        api_resp::GroupHonorInfo,
        group_id: String,
        ty: String
    );
    resp_api!(
        get_cookies,
        GetCookies,
        Cookies,
        api_resp::Cookies,
        domain: String
    );
    resp_api!(get_csrf_token, ScrfToken, api_resp::ScrfToken);
    resp_api!(
        get_credentials,
        GetCookies,
        Credentials,
        api_resp::Credentials,
        domain: String
    );
    resp_api!(
        get_record,
        GetRecord,
        File,
        api_resp::File,
        file: String,
        out_format: String
    );
    resp_api!(get_image, GetImage, File, api_resp::File, file: String);
    resp_api!(can_send_record, SendCheck, api_resp::SendCheck);
    resp_api!(can_send_image, SendCheck, api_resp::SendCheck);
    resp_api!(get_status, Status, crate::event::Status);
    resp_api!(get_version_info, VersionInfo, api_resp::VersionInfo);
}

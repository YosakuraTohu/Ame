use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

static API_URL: &str = "https://api.mlyai.com/reply";

pub struct Moli {
    api_key: &'static str,
    api_secret: &'static str,
}

impl Moli {
    pub const fn new(api_key: &'static str, api_secret: &'static str) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }

    pub async fn get_response(
        &self,
        data: MoliReqestParameter,
    ) -> Result<MoliResponseBody, String> {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::with_capacity(3);
        headers.insert("Api-Key", self.api_key.parse().unwrap());
        headers.insert("Api-Secret", self.api_secret.parse().unwrap());
        headers.insert(
            CONTENT_TYPE,
            "application/json;charset=UTF-8".parse().unwrap(),
        );
        let res = client
            .post(API_URL)
            .headers(headers)
            .json(&data)
            .send()
            .await;
        match res {
            Ok(body) => match body.json::<MoliResponseBody>().await {
                Ok(r) => {
                    if r.code == "00000" {
                        return Ok(r);
                    }
                    Err(r.message)
                }
                Err(_) => Err("JSON parse error.".to_owned()),
            },
            Err(_) => Err("Api reqest error.".to_owned()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MoliReqestParameter {
    content: String,
    #[serde(rename = "type")]
    ty: u8,
    from: String,
    #[serde(rename = "fromName")]
    from_name: String,
    to: String,
    #[serde(rename = "toName")]
    to_name: String,
}

impl MoliReqestParameter {
    pub fn new(
        content: String,
        ty: u8,
        from: String,
        from_name: String,
        to: String,
        to_name: String,
    ) -> Self {
        MoliReqestParameter {
            content,
            ty,
            from,
            from_name,
            to,
            to_name,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct MoliResponseBody {
    pub code: String,
    pub message: String,
    pub plugin: Option<String>,
    pub data: Option<Vec<MoliMessageData>>,
}

#[derive(Deserialize, Debug)]
pub struct MoliMessageData {
    pub content: String,
    pub typed: u8,
    pub remark: Option<String>,
}

#[tokio::test]
async fn test_without_apikey() {
    // static MOLI: Moli = Moli::new("ulpxdy2moi9ya4fm", "92kqb4ye");
    static MOLI: Moli = Moli::new("", "");
    let msg = MoliReqestParameter::new(
        "呜呜……".to_string(),
        1,
        "2956419803".to_string(),
        "祈洱".to_string(),
        "".to_string(),
        "".to_string(),
    );
    let res = MOLI.get_response(msg).await;
    println!("{:#?}", res);
}

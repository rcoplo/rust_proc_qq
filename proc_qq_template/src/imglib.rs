use proc_qq::re_export::rs_qq::client::event::{GroupMessageEvent, PrivateMessageEvent};
use proc_qq::re_export::rs_qq::msg::elem::Text;
use proc_qq::re_export::rs_qq::msg::MessageChain;
use proc_qq::re_export::{bytes, reqwest};
use proc_qq::{event, MessageChainParseTrait, module, Module};

static ID: &'static str = "imglib";
static NAME: &'static str = "图库";
static MENU: &'static str = "图库 (请直接回复功能名) : \n ❤️ 动漫壁纸";
static UA: &'static str = "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.80 Mobile Safari/537.36";

pub(crate) fn module() -> Module {
    module!(ID, NAME, group_message, private_message)
}

#[event]
async fn group_message(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let content = event.message.elements.to_string();
    if content.eq(NAME) {
        let chain = MessageChain::new(Text::new(MENU.to_owned()));
        event
            .client
            .send_group_message(event.message.group_code, chain)
            .await?;
        Ok(true)
    } else if content.eq("动漫壁纸") {
        let img = get_img().await?.to_vec();
        let img = event
            .client
            .upload_group_image(event.message.group_code, img)
            .await?;
        event
            .client
            .send_group_message(event.message.group_code, img.parse_message_chain())
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[event]
async fn private_message(event: &PrivateMessageEvent) -> anyhow::Result<bool> {
    let content = event.message.elements.to_string();
    if content.eq(NAME) {
        let chain = MessageChain::new(Text::new(MENU.to_owned()));
        event
            .client
            .send_private_message(event.message.from_uin, chain)
            .await?;
        Ok(true)
    } else if content.eq("动漫壁纸") {
        let chain = MessageChain::new(Text::new("去群里用".to_owned()));
        event
            .client
            .send_private_message(event.message.from_uin, chain)
            .await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
async fn get_img() -> anyhow::Result<bytes::Bytes> {
    let buff = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()?
        .request(
            reqwest::Method::GET,
            "http://api.molure.cn/sjbz/api.php?lx=dongman",
        )
        .header("User-Agent", UA)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    Ok(buff)
}

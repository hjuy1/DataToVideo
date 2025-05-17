#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, result::Result};

#[derive(Deserialize, Serialize)]
pub struct CharFile {
    pub Name: String,
    pub sex: Option<String>,
    pub combatExperience: Option<String>,
    pub birthPlace: Option<String>,
    #[serde(deserialize_with = "date_of_birth")]
    pub dateOfBirth: Option<String>,
    pub race: Option<String>,
    pub height: Option<String>,
    pub infectionStatus: Option<String>,
    pub cellOriginiumAssimilation: Option<String>,
    pub bloodOriginiumCrystalDensity: Option<String>,
    pub phy: Option<String>,
    pub flex: Option<String>,
    pub tolerance: Option<String>,
    pub plan: Option<String>,
    pub skill: Option<String>,
    pub adapt: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CarFile {
    pub 代号: String,
    pub 设定性别: String,
    pub 出厂时间: String,
    pub 制造商: String,
    pub 产地: String,
    pub 出厂日: String,
    pub 高度: String,
    pub 重量: String,
    pub 维护检测报告: String,
    pub 最高速度: String,
    pub 爬坡能力: String,
    pub 制动效能: String,
    pub 通过性: String,
    pub 续航: String,
    pub 结构稳定性: String,
}

#[derive(Deserialize, Serialize)]
pub struct CharData {
    pub Name: String,
    pub hp: Option<String>,
    pub atk: Option<String>,
    pub def: Option<String>,
    pub res: Option<String>,
    pub reDeploy: Option<String>,
    pub cost: Option<String>,
    pub block: Option<String>,
    pub atkSpeed: Option<String>,
    #[serde(alias = "trust")]
    pub trust_hp_atk_def: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CharInfo {
    pub Name: String,
    pub en: Option<String>,
    pub profession: Option<String>,
    pub subProfession: Option<String>,
    pub position: Option<String>,
    #[serde(deserialize_with = "rarity")]
    pub rarity: Option<u8>,
    pub logo: Option<String>,
    pub tag: Option<String>,
    pub skin1name: Option<String>,
    pub skin2name: Option<String>,
    pub skin3name: Option<String>,
    pub skin4name: Option<String>,
    pub skin5name: Option<String>,
    pub skin6name: Option<String>,
    pub skin7name: Option<String>,
    pub skin8name: Option<String>,
    pub skin9name: Option<String>,
    pub skin10name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CharObtain {
    pub name: String,
    pub obtain_date: (u16, u8, u8),
    pub obtain_way: String,
    pub get_by: String,
}

#[derive(Deserialize, Serialize)]
pub struct Char {
    pub Name: String,
    pub sex: Option<String>,
    pub combatExperience: Option<String>,
    pub birthPlace: Option<String>,
    pub dateOfBirth: Option<String>,
    pub race: Option<String>,
    pub height: Option<String>,
    pub infectionStatus: Option<String>,
    pub cellOriginiumAssimilation: Option<String>,
    pub bloodOriginiumCrystalDensity: Option<String>,
    pub phy: Option<String>,
    pub flex: Option<String>,
    pub tolerance: Option<String>,
    pub plan: Option<String>,
    pub skill: Option<String>,
    pub adapt: Option<String>,
    pub hp: Option<String>,
    pub atk: Option<String>,
    pub def: Option<String>,
    pub res: Option<String>,
    pub reDeploy: Option<String>,
    pub cost: Option<String>,
    pub block: Option<String>,
    pub atkSpeed: Option<String>,
    pub trust_hp_atk_def: Option<String>,
    pub en: Option<String>,
    pub profession: Option<String>,
    pub subProfession: Option<String>,
    pub position: Option<String>,
    pub rarity: Option<u8>,
    pub logo: Option<String>,
    pub tag: Option<String>,
    pub skin1name: Option<String>,
    pub skin2name: Option<String>,
    pub skin3name: Option<String>,
    pub skin4name: Option<String>,
    pub skin5name: Option<String>,
    pub skin6name: Option<String>,
    pub skin7name: Option<String>,
    pub skin8name: Option<String>,
    pub skin9name: Option<String>,
    pub skin10name: Option<String>,
    pub obtain_date: Option<(u16, u8, u8)>,
    pub obtain_way: Option<String>,
    pub get_by: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Memory {
    pub Name: String,
    pub storySetName: Option<String>,
    pub storyIntro: Option<String>,
    #[serde(deserialize_with = "story_txt")]
    pub storyTxt: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Mod {
    pub Name: String,
    #[serde(alias = "name")]
    pub 模组名: Option<String>,
    #[serde(alias = "charModuleN")]
    pub 模组数: Option<String>,
    #[serde(alias = "type")]
    pub 类型: Option<String>,
    #[serde(alias = "mission1")]
    pub 任务1: Option<String>,
    #[serde(alias = "mission2")]
    pub 任务2: Option<String>,
    #[serde(alias = "mission2Operation")]
    pub 任务2关卡: Option<String>,
    #[serde(alias = "traitadd")]
    #[serde(deserialize_with = "traitadd")]
    pub 是否增加特性: bool,
    #[serde(alias = "trait")]
    #[serde(deserialize_with = "del_lt_gt")]
    pub 等级1特性: Option<String>,
    #[serde(alias = "talent2")]
    #[serde(deserialize_with = "del_lt_gt")]
    pub 等级2天赋: Option<String>,
    #[serde(alias = "talent3")]
    #[serde(deserialize_with = "del_lt_gt")]
    pub 等级3天赋: Option<String>,
    #[serde(alias = "hp")]
    pub 加血量: Option<String>,
    #[serde(alias = "atk")]
    pub 加攻: Option<String>,
    #[serde(alias = "def")]
    pub 加防御: Option<String>,
    #[serde(alias = "res")]
    pub 加法抗: Option<String>,
    #[serde(alias = "time")]
    pub 时间: Option<String>,
    #[serde(alias = "cost")]
    pub 减费: Option<String>,
    #[serde(alias = "block")]
    pub 加阻挡: Option<String>,
    #[serde(alias = "atkspd")]
    pub 加攻速: Option<String>,
    #[serde(alias = "other")]
    pub 其他: Option<String>,
}

fn date_of_birth<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(
        Option::<String>::deserialize(deserializer)?.map(|s| match s.split_once('月') {
            Some((m, d)) => format!("{:0>2}月{:0>2}日", m, d.trim_end_matches('日')),
            None => s.to_string(),
        }),
    )
}

fn rarity<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<String>::deserialize(deserializer)?;
    Ok(v.and_then(|s| s.parse::<u8>().ok().and_then(|n| n.checked_add(1))))
}

fn story_txt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<String>::deserialize(deserializer)?;
    Ok(v.map(|s| format!("https://prts.wiki/w/{s}")))
}

fn del_lt_gt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v = Option::<String>::deserialize(deserializer)?;
    Ok(v.map(|mut s| {
        while let Some(lt) = s.find("&lt") {
            let gt = s.find("gt;").unwrap();
            s.replace_range(lt..gt + 3, "");
        }
        s
    }))
}

fn traitadd<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?.is_some())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Mastery {
    pub name: String,
    pub 专精: String,
    pub 职能: String,
}

#[derive(Deserialize, Serialize)]
pub struct Painter {
    pub name: String,
    pub opus: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Author {
    pub kind: String,
    pub intro: String,
    pub content: Vec<Painter>,
}

#[derive(Deserialize, Serialize)]
pub struct Preview {
    pub date: String,
    pub way: String,
    pub operator: String,
    pub preview: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Profession {
    pub profession: String,
    pub subprofession: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct RealName {
    pub operator: String,
    pub real_name: Vec<String>,
    pub source: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Skin {
    pub name: String,
    pub owner: String,
    pub essay: String,
    pub painter: String,
    pub access: String,
    pub brand_group: String,
    pub date_launch: (u16, u8, u8),
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Brand {
    pub name: String,
    pub pic_url: String,
    pub intro: String,
    pub skin: Vec<Skin>,
}

#[derive(Deserialize, Serialize)]
pub struct VoiceItem {
    pub voice_title: String,
    pub voice_filename: String,
    pub item: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct Voice {
    pub voice_base: HashMap<String, String>,
    pub voice_item: Vec<VoiceItem>,
}

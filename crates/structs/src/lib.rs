#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, result::Result};

#[derive(Deserialize, Serialize)]
pub struct CharFile {
    pub Name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combatExperience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthPlace: Option<String>,
    #[serde(deserialize_with = "date_of_birth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateOfBirth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infectionStatus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellOriginiumAssimilation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloodOriginiumCrystalDensity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reDeploy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atkSpeed: Option<String>,
    #[serde(alias = "trust")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_hp_atk_def: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CharInfo {
    pub Name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subProfession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(deserialize_with = "rarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin1name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin2name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin3name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin4name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin5name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin6name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin7name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin8name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin9name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combatExperience: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthPlace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateOfBirth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub race: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infectionStatus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cellOriginiumAssimilation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloodOriginiumCrystalDensity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tolerance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reDeploy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atkSpeed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_hp_atk_def: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subProfession: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin1name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin2name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin3name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin4name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin5name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin6name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin7name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin8name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin9name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin10name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtain_date: Option<(u16, u8, u8)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtain_way: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_by: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Memory {
    pub Name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storySetName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storyIntro: Option<String>,
    #[serde(deserialize_with = "story_txt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storyTxt: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Mod {
    pub Name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(alias = "charModuleN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub 模组数: Option<String>,
    #[serde(alias = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub 类型: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mission2Operation: Option<String>,
    #[serde(deserialize_with = "traitadd")]
    pub traitadd: bool,
    #[serde(alias = "trait")]
    #[serde(deserialize_with = "del_lt_gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub 等级1特性: Option<String>,
    #[serde(deserialize_with = "del_lt_gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent2: Option<String>,
    #[serde(deserialize_with = "del_lt_gt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub talent3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atkspd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
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

use serde::{Deserialize, Serialize};

// ここをValueEnumにして分割するのはありかも
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityVersion {
    CherryPickVersion(CherryPickVersion),
    MisskeyVersion(MisskeyVersion),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum EntitySoftware {
    Misskey,
    CherryPick,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum CherryPickVersion {
    V4_15_0,
    V4_15_1,
    V4_16_0,
    Latest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum MisskeyVersion {
    V2025_5_1,
    V2025_6_0,
    V2025_6_1,
    V2025_7_0,
    Latest,
}

impl EntityVersion {
    pub fn from(software: &EntitySoftware, version: &EntityVersion) -> Result<Self, String> {
        match software {
            EntitySoftware::CherryPick => {
                if let EntityVersion::CherryPickVersion(v) = version {
                    Ok(EntityVersion::CherryPickVersion(v.clone()))
                } else {
                    Err("Unsupported or uncorrected version. Check your software version.".to_string())
                }
            },
            EntitySoftware::Misskey => {
                if let EntityVersion::MisskeyVersion(v) = version {
                    Ok(EntityVersion::MisskeyVersion(v.clone()))
                } else {
                    Err("Unsupported or uncorrected version. Check your software version.".to_string())
                }
            }
        }
    }
}

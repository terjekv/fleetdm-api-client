use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetHost {
    id: u64,
    hostname: String,
    display_name: String,
}

impl TargetHost {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn hostname(&self) -> &str {
        &self.hostname
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetLabel {
    id: u64,
    name: String,
    display_text: String,
    count: u32,
}

impl TargetLabel {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn display_text(&self) -> &str {
        &self.display_text
    }
    pub fn count(&self) -> u32 {
        self.count
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetFleet {
    id: u64,
    name: String,
    count: u32,
}

impl TargetFleet {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn count(&self) -> u32 {
        self.count
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Targets {
    hosts: Vec<TargetHost>,
    labels: Vec<TargetLabel>,
    #[serde(default)]
    fleets: Vec<TargetFleet>,
    #[serde(default, rename = "teams")]
    legacy_teams: Vec<TargetFleet>,
}

impl Targets {
    pub fn hosts(&self) -> &[TargetHost] {
        &self.hosts
    }
    pub fn labels(&self) -> &[TargetLabel] {
        &self.labels
    }
    pub fn fleets(&self) -> &[TargetFleet] {
        if self.fleets.is_empty() {
            &self.legacy_teams
        } else {
            &self.fleets
        }
    }
    #[deprecated(note = "use fleets()")]
    pub fn teams(&self) -> &[TargetFleet] {
        self.fleets()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchTargetsRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected: Option<SearchTargetsSelected>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_observer: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTargetsSelected {
    pub hosts: Vec<u64>,
    pub labels: Vec<u64>,
    #[serde(rename = "fleets", alias = "teams")]
    pub fleets: Vec<u64>,
}

#[deprecated(note = "use TargetFleet")]
pub type TargetTeam = TargetFleet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTargetsResponse {
    targets: Targets,
    targets_count: u32,
    targets_online: u32,
    targets_offline: u32,
    targets_missing_in_action: u32,
}

impl SearchTargetsResponse {
    pub fn targets(&self) -> &Targets {
        &self.targets
    }
    pub fn targets_count(&self) -> u32 {
        self.targets_count
    }
    pub fn targets_online(&self) -> u32 {
        self.targets_online
    }
    pub fn targets_offline(&self) -> u32 {
        self.targets_offline
    }
    pub fn targets_missing_in_action(&self) -> u32 {
        self.targets_missing_in_action
    }
}

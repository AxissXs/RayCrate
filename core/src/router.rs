use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RuleType {
    Geosite(String),
    GeoIP(String),
    Domain(String),
    IP(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoutingRule {
    pub rule_type: RuleType,
    pub target_outbound: String, // "proxy", "direct", "block"
}

pub struct Router {
    pub rules: Vec<RoutingRule>,
}

impl Router {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: RoutingRule) {
        self.rules.push(rule);
    }

    pub fn default_routing() -> Self {
        let mut router = Self::new();
        router.add_rule(RoutingRule {
            rule_type: RuleType::Geosite("private".to_string()),
            target_outbound: "direct".to_string(),
        });
        router.add_rule(RoutingRule {
            rule_type: RuleType::GeoIP("private".to_string()),
            target_outbound: "direct".to_string(),
        });
        router
    }
}

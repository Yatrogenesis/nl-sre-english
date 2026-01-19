//! # Domain Plugin Module
//!
//! Provides the trait and infrastructure for injectable domain-specific vocabularies.
//! This enables NL-SRE-English to be extended for specialized domains like:
//! - Plasma Physics (Tokamaks, MHD, etc.)
//! - Nuclear Engineering
//! - Medical terminology
//! - Legal language
//!
//! ## Architecture
//!
//! ```text
//! nl-sre-english (Public)     nl-sre-domains (Private)
//! ┌─────────────────────┐     ┌─────────────────────────┐
//! │ DomainPlugin trait  │◄────│ PlasmaPhysicsDomain     │
//! │ QuantityValidation  │     │ NuclearEngineeringDomain│
//! │ EmergencyKeywords   │     │ MedicalDomain           │
//! └─────────────────────┘     └─────────────────────────┘
//! ```
//!
//! ## Author
//! Francisco Molina-Burgos, Avermex Research Division
//!
//! ## Date
//! January 2026

use std::collections::HashSet;

/// A domain-specific plugin that provides specialized vocabulary and validation.
///
/// Implementations define the units, keywords, and validation rules for a specific
/// domain (e.g., plasma physics, nuclear engineering, medicine).
///
/// # Example
///
/// ```rust,ignore
/// use nl_sre_english::domain::DomainPlugin;
///
/// struct PlasmaPhysicsDomain;
///
/// impl DomainPlugin for PlasmaPhysicsDomain {
///     fn name(&self) -> &str { "plasma-physics" }
///
///     fn get_special_units(&self) -> Vec<&str> {
///         vec!["keV", "Tesla", "MW/m²", "MA", "n20"]
///     }
///
///     fn get_context_keywords(&self) -> Vec<&str> {
///         vec!["plasma", "tokamak", "confinement", "fusion", "divertor"]
///     }
///
///     fn sanitize_quantity(&self, number: f64, unit: &str) -> Option<ValidatedQuantity> {
///         match unit.to_lowercase().as_str() {
///             "kev" if number > 0.0 && number < 100.0 => {
///                 Some(ValidatedQuantity::valid(number, "keV"))
///             }
///             _ => None
///         }
///     }
/// }
/// ```
pub trait DomainPlugin: Send + Sync {
    /// Returns the unique name/identifier for this domain
    fn name(&self) -> &str;

    /// Returns the version of this domain plugin
    fn version(&self) -> &str {
        "0.1.0"
    }

    /// Returns special units recognized by this domain
    ///
    /// Examples for plasma physics: `["keV", "Tesla", "MW/m²", "MA", "n20"]`
    fn get_special_units(&self) -> Vec<&str>;

    /// Returns context keywords that activate this domain
    ///
    /// These keywords help identify when this domain is relevant.
    /// Examples: `["plasma", "tokamak", "confinement", "fusion"]`
    fn get_context_keywords(&self) -> Vec<&str>;

    /// Returns emergency/critical keywords that require immediate attention
    ///
    /// These trigger the fast-path bypass for safety-critical operations.
    /// Examples: `["SCRAM", "EMERGENCY", "SHUTDOWN", "ABORT"]`
    fn get_emergency_keywords(&self) -> Vec<&str> {
        vec![]
    }

    /// Validates and sanitizes a quantity (number + unit combination)
    ///
    /// Returns `Some(ValidatedQuantity)` if valid, `None` if unrecognized.
    /// This allows domain-specific range checking and unit normalization.
    fn sanitize_quantity(&self, number: f64, unit: &str) -> Option<ValidatedQuantity>;

    /// Returns whether a word is a valid noun in this domain
    fn is_domain_noun(&self, word: &str) -> bool {
        self.get_context_keywords()
            .iter()
            .any(|kw| kw.eq_ignore_ascii_case(word))
    }

    /// Returns synonyms or related terms for a domain word
    fn get_synonyms(&self, word: &str) -> Vec<&str> {
        let _ = word;
        vec![]
    }

    /// Returns the severity level for an emergency keyword (1-10)
    fn emergency_severity(&self, keyword: &str) -> u8 {
        if self.get_emergency_keywords().iter().any(|k| k.eq_ignore_ascii_case(keyword)) {
            10 // Maximum severity by default
        } else {
            0
        }
    }

    /// Optional: Provide custom validation rules as a set of constraints
    fn get_constraints(&self) -> Vec<DomainConstraint> {
        vec![]
    }
}

/// A validated quantity with normalized unit and optional warnings
#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedQuantity {
    /// The numeric value
    pub value: f64,
    /// The normalized unit string
    pub unit: String,
    /// Whether this quantity is valid
    pub is_valid: bool,
    /// Optional warning message
    pub warning: Option<String>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
}

impl ValidatedQuantity {
    /// Create a valid quantity
    pub fn valid(value: f64, unit: &str) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            is_valid: true,
            warning: None,
            confidence: 1.0,
        }
    }

    /// Create a valid quantity with warning
    pub fn valid_with_warning(value: f64, unit: &str, warning: &str) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            is_valid: true,
            warning: Some(warning.to_string()),
            confidence: 0.8,
        }
    }

    /// Create an invalid quantity
    pub fn invalid(value: f64, unit: &str, reason: &str) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            is_valid: false,
            warning: Some(reason.to_string()),
            confidence: 0.0,
        }
    }

    /// Create a quantity with custom confidence
    pub fn with_confidence(value: f64, unit: &str, confidence: f64) -> Self {
        Self {
            value,
            unit: unit.to_string(),
            is_valid: confidence > 0.5,
            warning: None,
            confidence,
        }
    }
}

/// A domain-specific constraint
#[derive(Debug, Clone)]
pub struct DomainConstraint {
    /// Name of the constraint
    pub name: String,
    /// Parameter affected
    pub parameter: String,
    /// Minimum allowed value (inclusive)
    pub min_value: Option<f64>,
    /// Maximum allowed value (inclusive)
    pub max_value: Option<f64>,
    /// Required unit
    pub unit: Option<String>,
    /// Error message if violated
    pub error_message: String,
}

impl DomainConstraint {
    pub fn new(name: &str, parameter: &str) -> Self {
        Self {
            name: name.to_string(),
            parameter: parameter.to_string(),
            min_value: None,
            max_value: None,
            unit: None,
            error_message: format!("Constraint '{}' violated", name),
        }
    }

    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min_value = Some(min);
        self.max_value = Some(max);
        self
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min_value = Some(min);
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max_value = Some(max);
        self
    }

    pub fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }

    pub fn with_error(mut self, msg: &str) -> Self {
        self.error_message = msg.to_string();
        self
    }

    /// Check if a value satisfies this constraint
    pub fn check(&self, value: f64) -> Result<(), String> {
        if let Some(min) = self.min_value {
            if value < min {
                return Err(format!(
                    "{}: {} ({}) is below minimum {}",
                    self.error_message, self.parameter, value, min
                ));
            }
        }
        if let Some(max) = self.max_value {
            if value > max {
                return Err(format!(
                    "{}: {} ({}) exceeds maximum {}",
                    self.error_message, self.parameter, value, max
                ));
            }
        }
        Ok(())
    }
}

/// Registry for domain plugins
#[derive(Default)]
pub struct DomainRegistry {
    domains: Vec<Box<dyn DomainPlugin>>,
    emergency_keywords: HashSet<String>,
}

impl DomainRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a domain plugin
    pub fn register<D: DomainPlugin + 'static>(&mut self, domain: D) {
        // Add emergency keywords to fast lookup set
        for kw in domain.get_emergency_keywords() {
            self.emergency_keywords.insert(kw.to_uppercase());
        }
        self.domains.push(Box::new(domain));
    }

    /// Get all registered domains
    pub fn domains(&self) -> &[Box<dyn DomainPlugin>] {
        &self.domains
    }

    /// Find domains that match a set of context keywords
    pub fn find_matching_domains(&self, keywords: &[&str]) -> Vec<&dyn DomainPlugin> {
        self.domains
            .iter()
            .filter(|domain| {
                let domain_keywords: HashSet<_> = domain
                    .get_context_keywords()
                    .into_iter()
                    .map(|k| k.to_lowercase())
                    .collect();

                keywords.iter().any(|kw| domain_keywords.contains(&kw.to_lowercase()))
            })
            .map(|b| b.as_ref())
            .collect()
    }

    /// Fast-path check for emergency keywords
    ///
    /// This is O(1) for the common case of non-emergency input.
    #[inline]
    pub fn is_emergency(&self, word: &str) -> bool {
        self.emergency_keywords.contains(&word.to_uppercase())
    }

    /// Get the severity of an emergency keyword
    pub fn emergency_severity(&self, word: &str) -> u8 {
        self.domains
            .iter()
            .map(|d| d.emergency_severity(word))
            .max()
            .unwrap_or(0)
    }

    /// Validate a quantity across all registered domains
    pub fn validate_quantity(&self, number: f64, unit: &str) -> Option<ValidatedQuantity> {
        for domain in &self.domains {
            if let Some(validated) = domain.sanitize_quantity(number, unit) {
                return Some(validated);
            }
        }
        None
    }

    /// Get all special units from all domains
    pub fn all_special_units(&self) -> Vec<&str> {
        self.domains
            .iter()
            .flat_map(|d| d.get_special_units())
            .collect()
    }
}

/// Default/Generic domain that accepts common SI units
pub struct GenericSIDomain;

impl DomainPlugin for GenericSIDomain {
    fn name(&self) -> &str {
        "generic-si"
    }

    fn get_special_units(&self) -> Vec<&str> {
        vec![
            // Base SI units
            "m", "kg", "s", "A", "K", "mol", "cd",
            // Common derived units
            "Hz", "N", "Pa", "J", "W", "C", "V", "F", "Ω", "S",
            "Wb", "T", "H", "lm", "lx", "Bq", "Gy", "Sv", "kat",
            // Common prefixes
            "km", "cm", "mm", "μm", "nm", "pm",
            "kHz", "MHz", "GHz",
            "kW", "MW", "GW",
            "kV", "MV",
            "mA", "μA", "nA",
        ]
    }

    fn get_context_keywords(&self) -> Vec<&str> {
        vec![] // Generic domain matches no specific context
    }

    fn sanitize_quantity(&self, number: f64, unit: &str) -> Option<ValidatedQuantity> {
        // Accept any of the known SI units
        let known_units: HashSet<_> = self.get_special_units().into_iter().collect();

        if known_units.contains(unit) {
            Some(ValidatedQuantity::valid(number, unit))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDomain;

    impl DomainPlugin for TestDomain {
        fn name(&self) -> &str { "test" }

        fn get_special_units(&self) -> Vec<&str> {
            vec!["keV", "Tesla"]
        }

        fn get_context_keywords(&self) -> Vec<&str> {
            vec!["plasma", "tokamak"]
        }

        fn get_emergency_keywords(&self) -> Vec<&str> {
            vec!["SCRAM", "EMERGENCY"]
        }

        fn sanitize_quantity(&self, number: f64, unit: &str) -> Option<ValidatedQuantity> {
            match unit.to_lowercase().as_str() {
                "kev" if number > 0.0 && number < 100.0 => {
                    Some(ValidatedQuantity::valid(number, "keV"))
                }
                "kev" => Some(ValidatedQuantity::invalid(number, "keV", "keV out of range")),
                _ => None
            }
        }
    }

    #[test]
    fn test_domain_registration() {
        let mut registry = DomainRegistry::new();
        registry.register(TestDomain);

        assert_eq!(registry.domains().len(), 1);
        assert!(registry.is_emergency("SCRAM"));
        assert!(registry.is_emergency("scram")); // Case insensitive
        assert!(!registry.is_emergency("hello"));
    }

    #[test]
    fn test_quantity_validation() {
        let mut registry = DomainRegistry::new();
        registry.register(TestDomain);

        let valid = registry.validate_quantity(10.0, "keV");
        assert!(valid.is_some());
        assert!(valid.unwrap().is_valid);

        let invalid = registry.validate_quantity(200.0, "keV");
        assert!(invalid.is_some());
        assert!(!invalid.unwrap().is_valid);
    }

    #[test]
    fn test_find_matching_domains() {
        let mut registry = DomainRegistry::new();
        registry.register(TestDomain);

        let matches = registry.find_matching_domains(&["plasma", "fusion"]);
        assert_eq!(matches.len(), 1);

        let no_matches = registry.find_matching_domains(&["medical", "surgery"]);
        assert!(no_matches.is_empty());
    }

    #[test]
    fn test_constraint() {
        let constraint = DomainConstraint::new("temperature", "T")
            .with_range(0.1, 50.0)
            .with_unit("keV")
            .with_error("Temperature out of operational range");

        assert!(constraint.check(10.0).is_ok());
        assert!(constraint.check(0.05).is_err());
        assert!(constraint.check(100.0).is_err());
    }

    #[test]
    fn test_generic_si() {
        let domain = GenericSIDomain;

        let valid = domain.sanitize_quantity(100.0, "Hz");
        assert!(valid.is_some());

        let invalid = domain.sanitize_quantity(100.0, "bananas");
        assert!(invalid.is_none());
    }
}

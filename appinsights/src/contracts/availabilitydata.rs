use serde::Serialize;

// NOTE: This file was automatically generated.

/// Instances of AvailabilityData represent the result of executing an availability test.
#[derive(Debug, Serialize)]
pub struct AvailabilityData {
    ver: i32,
    id: String,
    name: String,
    duration: String,
    success: bool,
    run_location: Option<String>,
    message: Option<String>,
    properties: Option<std::collections::HashMap<String, String>>,
    measurements: Option<std::collections::HashMap<String, f64>>,
}

impl AvailabilityData {
    /// Create a new [AvailabilityData](trait.AvailabilityData.html) instance with default values set by the schema.
    pub fn new(id: String, name: String, duration: String, success: bool) -> Self {
        Self {
            ver: 2,
            id,
            name,
            duration,
            success,
            run_location: None,
            message: None,
            properties: None,
            measurements: None,
        }
    }

    /// Schema version
    pub fn with_ver(&mut self, ver: i32) -> &mut Self {
        self.ver = ver;
        self
    }

    /// Identifier of a test run. Use it to correlate steps of test run and telemetry generated by the service.
    pub fn with_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Name of the test that these availability results represent.
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Duration in format: DD.HH:MM:SS.MMMMMM. Must be less than 1000 days.
    pub fn with_duration(&mut self, duration: String) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Success flag.
    pub fn with_success(&mut self, success: bool) -> &mut Self {
        self.success = success;
        self
    }

    /// Name of the location where the test was run from.
    pub fn with_run_location(&mut self, run_location: Option<String>) -> &mut Self {
        self.run_location = run_location;
        self
    }

    /// Diagnostic message for the result.
    pub fn with_message(&mut self, message: Option<String>) -> &mut Self {
        self.message = message;
        self
    }

    /// Collection of custom properties.
    pub fn with_properties(&mut self, properties: Option<std::collections::HashMap<String, String>>) -> &mut Self {
        self.properties = properties;
        self
    }

    /// Collection of custom measurements.
    pub fn with_measurements(&mut self, measurements: Option<std::collections::HashMap<String, f64>>) -> &mut Self {
        self.measurements = measurements;
        self
    }
}

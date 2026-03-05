use std::collections::HashMap;

/// Runtime values for the Vex language.
/// This is what variables hold in memory when the program runs.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub enum VValue {
    // --- Basic Types ---
    
    /// Whole numbers like 1, 2, 3 or -10
    Int(i64),
    
    /// Numbers with dots like 3.14 or -0.5
    Float(f64),
    
    /// True or false
    Bool(bool),
    
    /// Text like "hello world"
    Str(String),
    
    /// Empty value, means "nothing"
    Null,

    // --- Collections ---
    
    /// A list of values like [1, "two", 3.0]
    List(Vec<VValue>),
    
    /// Key and value pairs like {"name": "Vex", "age": 1}
    Dict(HashMap<String, VValue>),
}

// Simple helper functions for VValue
impl VValue {
    /// Check if the value is a number (Int or Float)
    pub fn is_number(&self) -> bool {
        match self {
            VValue::Int(_) | VValue::Float(_) => true,
            _ => false,
        }
    }

    /// Check if the value is True or False
    pub fn is_truthy(&self) -> bool {
        match self {
            VValue::Null => false,
            VValue::Bool(b) => *b,
            _ => true, // Everything else is true
        }
    }
}
use crate::ir::Diagnostics;
use crate::macro_core::functions::record_unsupported_attributes;
use crate::macro_core::types::delegate_component::{
    DelegateEntries, DelegateKey, DelegateMapping, DelegateStatement, DelegateTable, DelegateValue,
    DelegateValueWithInnerTable, ForDelegateStatement, MultiDelegateKey, PathDelegateKey,
    SingleDelegateKey,
};

/**
    Validate that the attributes in the delegate table constructs are valid.

    At the moment, no attribute is supported, so all attributes are rejected. The walk
    does not stop at the first error: every illegal attribute in the table is recorded in
    `diagnostics`, so one compile reports all of them.
*/
pub trait ValidateAttributes {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics);
}

impl ValidateAttributes for SingleDelegateKey {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        record_unsupported_attributes(&self.attributes, diagnostics);
    }
}

impl ValidateAttributes for MultiDelegateKey {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        record_unsupported_attributes(&self.attributes, diagnostics);

        for key in &self.keys {
            key.validate_attributes(diagnostics);
        }
    }
}

impl ValidateAttributes for PathDelegateKey {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        record_unsupported_attributes(&self.attributes, diagnostics);
    }
}

impl ValidateAttributes for DelegateKey {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        match self {
            DelegateKey::Single(key) => key.validate_attributes(diagnostics),
            DelegateKey::Multi(key) => key.validate_attributes(diagnostics),
            DelegateKey::Path(key) => key.validate_attributes(diagnostics),
        }
    }
}

impl ValidateAttributes for DelegateValueWithInnerTable {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        // The wrapper cannot carry attributes, but the inner table's keys can, so
        // recurse into it rather than letting an attribute be silently dropped.
        self.inner_table.entries.validate_attributes(diagnostics);
    }
}

impl ValidateAttributes for DelegateValue {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        match self {
            DelegateValue::Type(_) => {}
            DelegateValue::WithTable(value) => value.validate_attributes(diagnostics),
        }
    }
}

impl ValidateAttributes for DelegateMapping {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        match self {
            // A Normal or Direct value may open a nested inner table whose keys
            // can hold attributes, so validate the value as well as the key.
            DelegateMapping::Normal(mapping) => {
                mapping.key.validate_attributes(diagnostics);
                mapping.value.validate_attributes(diagnostics);
            }
            DelegateMapping::Direct(mapping) => {
                mapping.key.validate_attributes(diagnostics);
                mapping.value.validate_attributes(diagnostics);
            }
            // A Redirect value is a bare `@`-path with no inner table.
            DelegateMapping::Redirect(mapping) => mapping.key.validate_attributes(diagnostics),
            // Shorthand and preset entries carry no attributes.
            DelegateMapping::TypeShorthand(_)
            | DelegateMapping::GetterShorthand(_)
            | DelegateMapping::Preset(_) => {}
        }
    }
}

impl ValidateAttributes for ForDelegateStatement {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        for mapping in &self.mappings {
            mapping.key.validate_attributes(diagnostics);
            mapping.value.validate_attributes(diagnostics);
        }
    }
}

impl ValidateAttributes for DelegateStatement {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        match self {
            // `namespace` and `open` statements carry no keys that can hold attributes.
            DelegateStatement::Namespace(_) | DelegateStatement::Open(_) => {}
            DelegateStatement::For(statement) => statement.validate_attributes(diagnostics),
        }
    }
}

impl ValidateAttributes for DelegateEntries {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        // Keys nested inside statement forms (`for`/`namespace`/`open`) do not
        // support attributes, so reject any rather than silently discarding them.
        for statement in &self.statements {
            statement.validate_attributes(diagnostics);
        }

        for entry in &self.entries {
            entry.validate_attributes(diagnostics);
        }
    }
}

impl ValidateAttributes for DelegateTable {
    fn validate_attributes(&self, diagnostics: &mut Diagnostics) {
        record_unsupported_attributes(&self.attributes, diagnostics);
        self.entries.validate_attributes(diagnostics);
    }
}

//! Category and section taxonomy for the grok-pi settings panel.
//!
//! Section membership is owned by [`crate::settings::layout`] so this panel
//! and the upstream `settings_modal` share the same setting groups. This
//! module keeps the panel-specific render order for the single-page list.

use crate::settings::SettingCategory;

pub use crate::settings::layout::{OTHER_SECTION, section_for, sections_for};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::SettingsRegistry;

    #[test]
    fn every_setting_has_a_section() {
        let registry = SettingsRegistry::defaults();
        let orphans: Vec<&str> = registry
            .all()
            .iter()
            .filter(|meta| section_for(meta.key) == OTHER_SECTION)
            .map(|meta| meta.key)
            .collect();
        assert!(
            orphans.is_empty(),
            "settings with no declared section: {orphans:?} — \
             add them to crate::settings::layout::section_for",
        );
    }

    #[test]
    fn every_section_is_declared_for_its_category() {
        let registry = SettingsRegistry::defaults();
        for meta in registry.all() {
            let section = section_for(meta.key);
            let declared = sections_for(meta.category);
            assert!(
                declared.contains(&section),
                "`{}` maps to section `{section}`, which is not in \
                 sections_for({:?}) = {declared:?}",
                meta.key,
                meta.category,
            );
        }
    }

    #[test]
    fn declared_sections_are_unique_per_category() {
        for category in SettingCategory::ALL {
            let mut seen = std::collections::HashSet::new();
            for section in sections_for(*category) {
                assert!(
                    seen.insert(*section),
                    "duplicate section `{section}` in sections_for({category:?})",
                );
            }
        }
    }

}

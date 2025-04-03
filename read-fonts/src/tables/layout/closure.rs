//! Support Layout Closure

use super::{FeatureList, LangSys, ReadError, Script, ScriptList, Tag};
use crate::{collections::IntSet, TableRef};

const MAX_SCRIPTS: u16 = 500;
const MAX_LANGSYS: u16 = 2000;
const MAX_FEATURE_INDICES: u16 = 1500;
struct CollectFeaturesContext<'a> {
    script_count: u16,
    langsys_count: u16,
    feature_index_count: u16,
    visited_script: IntSet<u32>,
    visited_langsys: IntSet<u32>,
    feature_indices: &'a mut IntSet<u16>,

    feature_indices_filter: Option<IntSet<u16>>,
    feature_list: &'a FeatureList<'a>,
    table_head: usize,
}

impl<'a> CollectFeaturesContext<'a> {
    pub(crate) fn new(
        features: Option<&IntSet<Tag>>,
        table_head: usize,
        feature_list: &'a FeatureList<'a>,
        feature_indices: &'a mut IntSet<u16>,
    ) -> Self {
        let mut this = Self {
            script_count: 0,
            langsys_count: 0,
            feature_index_count: 0,
            visited_script: IntSet::empty(),
            visited_langsys: IntSet::empty(),
            feature_indices: feature_indices,
            feature_indices_filter: None,
            feature_list: feature_list,
            table_head: table_head,
        };
        this.compute_feature_filter(features);
        this
    }

    fn compute_feature_filter(&mut self, features: Option<&IntSet<Tag>>) {
        let Some(features) = features else {
            return;
        };

        let mut indices = IntSet::empty();
        for (idx, record) in self.feature_list.feature_records().iter().enumerate() {
            let tag = record.feature_tag();
            if features.contains(tag) {
                indices.insert(idx as u16);
            }
        }
        self.feature_indices_filter = Some(indices);
    }

    pub(crate) fn script_visited(&mut self, s: &Script) -> bool {
        if self.script_count > MAX_SCRIPTS {
            return true;
        }

        self.script_count += 1;

        let delta = (s.offset_data().as_bytes().as_ptr() as usize - self.table_head) as u32;
        if self.visited_script.contains(delta) {
            return true;
        }
        self.visited_script.insert(delta);
        false
    }

    pub(crate) fn langsys_visited(&mut self, langsys: &LangSys) -> bool {
        if self.langsys_count > MAX_LANGSYS {
            return true;
        }

        self.langsys_count += 1;

        let delta = (langsys.offset_data().as_bytes().as_ptr() as usize - self.table_head) as u32;
        if self.visited_langsys.contains(delta) {
            return true;
        }
        self.visited_langsys.insert(delta);
        false
    }

    pub(crate) fn feature_indices_limit_exceeded(&mut self, count: u16) -> bool {
        let (new_count, overflow) = self.feature_index_count.overflowing_add(count);
        if overflow {
            self.feature_index_count = MAX_FEATURE_INDICES;
            return true;
        }
        self.feature_index_count = new_count;
        new_count > MAX_FEATURE_INDICES
    }
}

impl ScriptList<'_> {
    /// Return a set of all feature indices underneath the specified scripts, languages and features
    /// if no script is provided, all scripts will be queried
    /// if no language is provided, all languages will be queried
    /// if no feature is provided, all features will be queried
    pub fn collect_features(
        &self,
        c: &mut CollectFeaturesContext,
        scripts: Option<&IntSet<Tag>>,
        languages: Option<&IntSet<Tag>>,
        features: Option<&IntSet<Tag>>,
    ) -> Result<IntSet<Tag>, ReadError> {
        let script_records = self.script_records();
        let font_data = self.offset_data();
        let mut out = IntSet::empty();
        if scripts.is_none() {
            // All scripts
            for record in script_records {
                let script = record.script(font_data)?;
                script.collect_features()?;
            }
        } else {
            let scripts = scripts.unwrap();
            for tag in scripts.iter() {
                let Some(idx) = self.index_for_tag(tag) else {
                    continue;
                };
                let script = script_records[idx as usize].script(font_data)?;
                script.collect_features()?;
            }
        }
        Ok(out)
    }
}

impl Script<'_> {
    fn collect_features(
        &self,
        c: &mut CollectFeaturesContext,
        languages: Option<&IntSet<Tag>>,
    ) -> Result<(), ReadError> {
        let lang_sys_records = self.lang_sys_records();
        let font_data = self.offset_data();
        if languages.is_none() {
            // All languages
            if let Some(default_lang_sys) = self.default_lang_sys().transpose()? {
                default_lang_sys.collect_features(c);
            }

            for record in lang_sys_records {
                let lang_sys = record.lang_sys(font_data)?;
                lang_sys.collect_features(c);
            }
        } else {
            let languages = languages.unwrap();
            for tag in languages.iter() {
                let Some(idx) = self.lang_sys_index_for_tag(tag) else {
                    continue;
                };
                let lang_sys = lang_sys_records[idx as usize].lang_sys(font_data)?;
                lang_sys.collect_features(c);
            }
        }

        Ok(())
    }
}

impl LangSys<'_> {
    fn collect_features(&self, c: &mut CollectFeaturesContext) {
        if c.langsys_visited(&self) {
            return;
        }

        if c.feature_indices_filter.is_none() {
            // All features
            let required_feature_idx = self.required_feature_index();
            if required_feature_idx != 0xFFFF && !c.feature_indices_limit_exceeded(1) {
                c.feature_indices.insert(required_feature_idx);
            }

            if !c.feature_indices_limit_exceeded(count) {
                c.feature_indices.extend(iter);
            }
        } else {
        }
    }
}

use crate::revision::filter_rev::FilterConfigurationRevision;
use crate::revision::grid_group::GroupConfigurationRevision;
use crate::revision::{FieldRevision, FieldTypeRevision};
use indexmap::IndexMap;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub fn gen_grid_filter_id() -> String {
    nanoid!(6)
}

pub fn gen_grid_group_id() -> String {
    nanoid!(6)
}

pub fn gen_grid_sort_id() -> String {
    nanoid!(6)
}

pub type FilterConfigurations = SettingConfiguration<FilterConfigurationRevision>;
pub type FilterConfigurationRevisionMap = GridObjectRevisionMap<FilterConfigurationRevision>;
pub type FilterConfigurationsByFieldId = HashMap<String, Vec<Arc<FilterConfigurationRevision>>>;
//
pub type GroupConfigurations = SettingConfiguration<GroupConfigurationRevision>;
pub type GroupConfigurationRevisionMap = GridObjectRevisionMap<GroupConfigurationRevision>;
pub type GroupConfigurationsByFieldId = HashMap<String, Vec<Arc<GroupConfigurationRevision>>>;
//
pub type SortConfigurations = SettingConfiguration<SortConfigurationRevision>;
pub type SortConfigurationRevisionMap = GridObjectRevisionMap<SortConfigurationRevision>;
pub type SortConfigurationsByFieldId = HashMap<String, Vec<Arc<SortConfigurationRevision>>>;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct GridSettingRevision {
    pub layout: GridLayoutRevision,

    pub filters: FilterConfigurations,

    #[serde(default)]
    pub groups: GroupConfigurations,

    #[serde(skip)]
    pub sorts: SortConfigurations,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GridLayoutRevision {
    Table = 0,
    Board = 1,
}

impl ToString for GridLayoutRevision {
    fn to_string(&self) -> String {
        let layout_rev = self.clone() as u8;
        layout_rev.to_string()
    }
}

impl std::default::Default for GridLayoutRevision {
    fn default() -> Self {
        GridLayoutRevision::Table
    }
}

impl GridSettingRevision {
    pub fn get_all_groups(&self, field_revs: &[Arc<FieldRevision>]) -> Option<GroupConfigurationsByFieldId> {
        self.groups.get_all_objects(&self.layout, field_revs)
    }

    pub fn get_groups(
        &self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type_rev: &FieldTypeRevision,
    ) -> Option<Vec<Arc<GroupConfigurationRevision>>> {
        self.groups.get_objects(layout, field_id, field_type_rev)
    }

    pub fn get_mut_groups(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
    ) -> Option<&mut Vec<Arc<GroupConfigurationRevision>>> {
        self.groups.get_mut_objects(layout, field_id, field_type)
    }

    pub fn insert_group(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
        group_rev: GroupConfigurationRevision,
    ) {
        self.groups.remove_all(layout);
        self.groups.insert_object(layout, field_id, field_type, group_rev);
    }

    pub fn get_all_filters(&self, field_revs: &[Arc<FieldRevision>]) -> Option<FilterConfigurationsByFieldId> {
        self.filters.get_all_objects(&self.layout, field_revs)
    }

    pub fn get_filters(
        &self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type_rev: &FieldTypeRevision,
    ) -> Option<Vec<Arc<FilterConfigurationRevision>>> {
        self.filters.get_objects(layout, field_id, field_type_rev)
    }

    pub fn get_mut_filters(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
    ) -> Option<&mut Vec<Arc<FilterConfigurationRevision>>> {
        self.filters.get_mut_objects(layout, field_id, field_type)
    }

    pub fn insert_filter(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
        filter_rev: FilterConfigurationRevision,
    ) {
        self.filters.insert_object(layout, field_id, field_type, filter_rev);
    }

    pub fn get_all_sort(&self) -> Option<SortConfigurationsByFieldId> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SortConfigurationRevision {
    pub id: String,
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(transparent)]
pub struct SettingConfiguration<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    /// Each layout contains multiple key/value.
    /// Key:    field_id
    /// Value:  this value contains key/value.
    ///         Key: FieldType,
    ///         Value: the corresponding objects.
    #[serde(with = "indexmap::serde_seq")]
    inner: IndexMap<GridLayoutRevision, IndexMap<String, GridObjectRevisionMap<T>>>,
}

impl<T> SettingConfiguration<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    pub fn get_mut_objects(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
    ) -> Option<&mut Vec<Arc<T>>> {
        let value = self
            .inner
            .get_mut(layout)
            .and_then(|object_rev_map_by_field_id| object_rev_map_by_field_id.get_mut(field_id))
            .and_then(|object_rev_map| object_rev_map.get_mut(field_type));
        if value.is_none() {
            tracing::warn!("Can't find the {:?} with", std::any::type_name::<T>());
        }
        value
    }
    pub fn get_objects(
        &self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type_rev: &FieldTypeRevision,
    ) -> Option<Vec<Arc<T>>> {
        self.inner
            .get(layout)
            .and_then(|object_rev_map_by_field_id| object_rev_map_by_field_id.get(field_id))
            .and_then(|object_rev_map| object_rev_map.get(field_type_rev))
            .cloned()
    }

    pub fn get_all_objects(
        &self,
        layout: &GridLayoutRevision,
        field_revs: &[Arc<FieldRevision>],
    ) -> Option<HashMap<String, Vec<Arc<T>>>> {
        // Acquire the read lock.
        let object_rev_map_by_field_id = self.inner.get(layout)?;
        // Get the objects according to the FieldType, so we need iterate the field_revs.
        let objects_by_field_id = field_revs
            .iter()
            .flat_map(|field_rev| {
                let field_type = &field_rev.field_type_rev;
                let field_id = &field_rev.id;

                let object_rev_map = object_rev_map_by_field_id.get(field_id)?;
                let objects: Vec<Arc<T>> = object_rev_map.get(field_type)?.clone();
                Some((field_rev.id.clone(), objects))
            })
            .collect::<HashMap<String, Vec<Arc<T>>>>();
        Some(objects_by_field_id)
    }

    pub fn insert_object(
        &mut self,
        layout: &GridLayoutRevision,
        field_id: &str,
        field_type: &FieldTypeRevision,
        object: T,
    ) {
        let object_rev_map_by_field_id = self.inner.entry(layout.clone()).or_insert_with(IndexMap::new);
        let object_rev_map = object_rev_map_by_field_id
            .entry(field_id.to_string())
            .or_insert_with(GridObjectRevisionMap::<T>::new);

        object_rev_map
            .entry(field_type.to_owned())
            .or_insert_with(Vec::new)
            .push(Arc::new(object))
    }

    pub fn remove_all(&mut self, layout: &GridLayoutRevision) {
        if let Some(object_rev_map_by_field_id) = self.inner.get_mut(layout) {
            object_rev_map_by_field_id.clear()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(transparent)]
pub struct GridObjectRevisionMap<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    #[serde(with = "indexmap::serde_seq")]
    pub object_by_field_type: IndexMap<FieldTypeRevision, Vec<Arc<T>>>,
}

impl<T> GridObjectRevisionMap<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    pub fn new() -> Self {
        GridObjectRevisionMap::default()
    }
}

impl<T> std::ops::Deref for GridObjectRevisionMap<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    type Target = IndexMap<FieldTypeRevision, Vec<Arc<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.object_by_field_type
    }
}

impl<T> std::ops::DerefMut for GridObjectRevisionMap<T>
where
    T: Debug + Clone + Default + Eq + PartialEq + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object_by_field_type
    }
}

use super::{IDs, Id, IdToID, IndexMap, ItemEnum, ID};

#[derive(Default)]
pub struct DImpl {
    pub inherent: IDs,
    pub trait_: IDs,
    pub auto: IDs,
    pub blanket: IDs,
}
impl DImpl {
    pub fn new(ids: &[Id], index: &IndexMap) -> Box<Self> {
        if ids.is_empty() {
            return Default::default();
        }
        let [mut inherent, mut trait_, mut auto, mut blanket]: [Vec<ID>; 4] = Default::default();
        for Id(id) in ids {
            if id.starts_with("a:") {
                auto.push(id.to_ID());
            } else if id.starts_with("b:") {
                blanket.push(id.to_ID());
            } else {
                let id = Id(id.clone());
                if let Some(item) = index.get(&id) {
                    if let ItemEnum::Impl(impl_) = &item.inner {
                        if impl_.trait_.is_none() {
                            inherent.push(id.into_ID());
                        } else {
                            trait_.push(id.into_ID());
                        }
                    } else {
                        warn!("{id:?} in Crate's index doesn't refer to an impl item");
                    }
                } else {
                    warn!("the impl with {id:?} not found in Crate's index");
                }
            }
        }
        Box::new(DImpl {
            inherent: inherent.into(),
            trait_: trait_.into(),
            auto: auto.into(),
            blanket: blanket.into(),
        })
    }
    pub fn is_empty(&self) -> bool {
        self.inherent.is_empty()
            && self.trait_.is_empty()
            && self.auto.is_empty()
            && self.blanket.is_empty()
    }
}

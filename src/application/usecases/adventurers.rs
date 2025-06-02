use std::sync::Arc;
use anyhow::Result;
use crate::domain::{repositories::adventurers::AdventurersRepository, value_object::adventurer_model::RegisterAdventurerModel};

pub struct AdventurersUseCase <T> 
where
    T: AdventurersRepository + Send + Sync // Send = sent data skip tril; sync = run muti tril
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T> 
where T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self 
        { 
            adventurers_repository,
        }
    }

    pub async fn register(&self, mut register_adventurer_model: RegisterAdventurerModel,) -> Result<i32>
    {
        unimplemented!()
    }
}
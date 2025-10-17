use crate::types::{Vec2, GameEntity};

pub struct GameWorld {
    entities: Vec<GameEntity>,
    next_id: u32,
}

impl GameWorld {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_entity(&mut self, entity_type: crate::types::EntityType, position: Vec2) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        
        self.entities.push(GameEntity {
            id,
            position,
            entity_type,
        });
        
        id
    }
    
    pub fn get_entities(&self) -> &[GameEntity] {
        &self.entities
    }
    
    pub fn update_entity_position(&mut self, id: u32, new_position: Vec2) -> bool {
        if let Some(entity) = self.entities.iter_mut().find(|e| e.id == id) {
            entity.position = new_position;
            true
        } else {
            false
        }
    }
}

impl Default for GameWorld {
    fn default() -> Self {
        Self::new()
    }
}

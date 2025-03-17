#![allow(dead_code)]

use bevy::prelude::{Component, Event, States};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

#[derive(Event)]
pub struct FloorPlanEvent {
    pub floorplan: FloorPlan,
}

#[derive(Component, Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub struct Room {
    pub id: String,
    pub name: String,
}

#[derive(Component, Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub struct Door {
    pub id: String,
    pub name: String,
    pub is_exit: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum FloorPlanError {
    RoomNotFound(String),
    DoorNotFound(String),
    ServiceError(String),
}

pub type FloorPlanResult<T> = Result<T, FloorPlanError>;

#[derive(Debug, Clone, Default, States)]
pub struct FloorPlan {
    pub graph: DiGraph<Room, Door>,
    room_index_map: HashMap<String, NodeIndex>,
    start_room_id: Option<String>,
}

impl std::hash::Hash for FloorPlan {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.room_index_map.keys().for_each(|key| key.hash(state));
        self.start_room_id.hash(state);
    }
}

impl PartialEq for FloorPlan {
    fn eq(&self, other: &Self) -> bool {
        self.room_index_map == other.room_index_map && self.start_room_id == other.start_room_id
    }
}

impl Eq for FloorPlan {}

impl FloorPlan {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            room_index_map: HashMap::new(),
            start_room_id: None,
        }
    }

    pub fn get_world_size(&self) -> (usize, usize) {
        (self.graph.node_count(), self.graph.edge_count())
    }

    pub fn get_room(&self, room_index: NodeIndex) -> FloorPlanResult<&Room> {
        self.graph
            .node_weight(room_index)
            .ok_or_else(|| FloorPlanError::RoomNotFound(room_index.index().to_string()))
    }

    pub fn get_all_room_ids(&self) -> Vec<String> {
        self.room_index_map.keys().cloned().collect()
    }

    pub fn add_room(&mut self, room: Room) -> NodeIndex {
        let room_index = self.graph.add_node(room.clone());
        self.room_index_map.insert(room.id.clone(), room_index);
        if self.start_room_id.is_none() {
            self.start_room_id = Some(room.id);
        }
        room_index
    }

    pub fn set_start_room(&mut self, room_id: &str) -> FloorPlanResult<()> {
        if self.room_index_map.contains_key(room_id) {
            self.start_room_id = Some(room_id.to_string());
            Ok(())
        } else {
            Err(FloorPlanError::RoomNotFound(room_id.to_string()))
        }
    }

    pub fn get_start_room(&self) -> FloorPlanResult<&Room> {
        match &self.start_room_id {
            Some(room_id) => {
                let room_index = self.get_room_idx_by_id(room_id)?;
                self.graph
                    .node_weight(room_index)
                    .ok_or_else(|| FloorPlanError::RoomNotFound(room_id.clone()))
            }
            None => Err(FloorPlanError::RoomNotFound(
                "Start room not set".to_string(),
            )),
        }
    }

    pub fn add_door(&mut self, from: NodeIndex, to: NodeIndex, door: Door) {
        self.graph.add_edge(from, to, door);
    }

    pub fn get_doors(&self, room_index: NodeIndex) -> Vec<&Door> {
        self.graph
            .edges(room_index)
            .map(|edge| edge.weight())
            .collect()
    }

    pub fn get_connected_room(
        &self,
        room_index: NodeIndex,
        door_id: &str,
    ) -> FloorPlanResult<&Room> {
        for edge in self.graph.edges(room_index) {
            if edge.weight().id == door_id {
                let target_index = edge.target();
                return self
                    .graph
                    .node_weight(target_index)
                    .ok_or_else(|| FloorPlanError::RoomNotFound(door_id.to_string()));
            }
        }
        Err(FloorPlanError::DoorNotFound(door_id.to_string()))
    }

    pub fn get_room_idx_by_id(&self, room_id: &str) -> FloorPlanResult<NodeIndex> {
        self.room_index_map
            .get(room_id)
            .copied()
            .ok_or_else(|| FloorPlanError::RoomNotFound(room_id.to_string()))
    }

    pub fn get_room_by_id(&self, room_id: &str) -> FloorPlanResult<&Room> {
        self.get_room_idx_by_id(room_id).map_or_else(
            |_| Err(FloorPlanError::RoomNotFound(room_id.to_string())),
            |room_index| {
                self.graph
                    .node_weight(room_index)
                    .ok_or_else(|| FloorPlanError::RoomNotFound(room_index.index().to_string()))
            },
        )
    }

    pub fn get_doors_and_connected_rooms(
        &self,
        room_id: &str,
    ) -> FloorPlanResult<Vec<(&Door, &Room)>> {
        let room_index = self.get_room_idx_by_id(room_id)?;
        let result = self
            .graph
            .edges(room_index)
            .map(|edge| {
                let door = edge.weight();
                let room = self.graph.node_weight(edge.target()).ok_or_else(|| {
                    FloorPlanError::RoomNotFound(edge.target().index().to_string())
                })?;
                Ok((door, room))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_room_and_door() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };

        let room1_index = floor_plan.add_room(room1);
        let room2_index = floor_plan.add_room(room2);

        let door = Door {
            id: "1".to_string(),
            name: "Door 1".to_string(),
            is_exit: false,
        };
        floor_plan.add_door(room1_index, room2_index, door);

        let doors = floor_plan.get_doors(room1_index);
        assert_eq!(doors.len(), 1);
        assert_eq!(doors[0].name, "Door 1");

        match floor_plan.get_connected_room(room1_index, &doors[0].id) {
            Ok(connected_room) => assert_eq!(connected_room.name, "Room 2"),
            Err(_) => panic!("Connected room not found"),
        }
    }

    #[test]
    fn test_get_connected_room() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };
        let room3 = Room {
            id: "3".to_string(),
            name: "Room 3".to_string(),
        };

        let room1_index = floor_plan.add_room(room1);
        let room2_index = floor_plan.add_room(room2);
        let room3_index = floor_plan.add_room(room3);

        let door1 = Door {
            id: "1".to_string(),
            name: "Door 1".to_string(),
            is_exit: false,
        };
        let door2 = Door {
            id: "2".to_string(),
            name: "Door 2".to_string(),
            is_exit: false,
        };

        floor_plan.add_door(room1_index, room2_index, door1);
        floor_plan.add_door(room2_index, room3_index, door2);

        let doors = floor_plan.get_doors(room2_index);
        assert_eq!(doors.len(), 1);

        match floor_plan.get_connected_room(room2_index, &doors[0].id) {
            Ok(connected_room) => assert_eq!(connected_room.name, "Room 3"),
            Err(_) => panic!("Connected room not found for door 1"),
        }

        match floor_plan.get_connected_room(room2_index, &doors[0].id) {
            Ok(connected_room) => assert_eq!(connected_room.name, "Room 3"),
            Err(_) => panic!("Connected room not found for door 2"),
        }
    }

    #[test]
    fn test_get_doors_and_connected_rooms() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };
        let room3 = Room {
            id: "3".to_string(),
            name: "Room 3".to_string(),
        };

        floor_plan.add_room(room1.clone());
        floor_plan.add_room(room2.clone());
        floor_plan.add_room(room3.clone());

        let door1 = Door {
            id: "1".to_string(),
            name: "Door 1".to_string(),
            is_exit: false,
        };
        let door2 = Door {
            id: "2".to_string(),
            name: "Door 2".to_string(),
            is_exit: false,
        };

        floor_plan.add_door(
            floor_plan.get_room_idx_by_id(&room1.id).unwrap(),
            floor_plan.get_room_idx_by_id(&room2.id).unwrap(),
            door1.clone(),
        );
        floor_plan.add_door(
            floor_plan.get_room_idx_by_id(&room2.id).unwrap(),
            floor_plan.get_room_idx_by_id(&room3.id).unwrap(),
            door2.clone(),
        );

        match floor_plan.get_doors_and_connected_rooms(&room2.id) {
            Ok(doors_and_rooms) => {
                assert_eq!(doors_and_rooms.len(), 1);
                assert_eq!(doors_and_rooms[0].0.name, "Door 2");
                assert_eq!(doors_and_rooms[0].1.name, "Room 3");
            }
            Err(_) => panic!("Failed to get doors and connected rooms"),
        }
    }

    #[test]
    fn test_start_room() {
        let mut floor_plan = FloorPlan::new();

        let room1 = Room {
            id: "1".to_string(),
            name: "Room 1".to_string(),
        };
        let room2 = Room {
            id: "2".to_string(),
            name: "Room 2".to_string(),
        };

        floor_plan.add_room(room1.clone());
        floor_plan.add_room(room2.clone());

        assert_eq!(floor_plan.get_start_room().unwrap().name, "Room 1");

        floor_plan.set_start_room(&room2.id).unwrap();
        assert_eq!(floor_plan.get_start_room().unwrap().name, "Room 2");
    }
}

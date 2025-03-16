use std::fs;

use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, FloorPlanResult, Room};
use bevy::prelude::*;
use serde_json::json;
use serde_yaml::Value;

use super::k8s_json::{get_names, get_namespaces};

pub fn connect_rooms_with_doors(
    plan: &mut FloorPlan,
    room1: &Room,
    room2: &Room,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    debug!("Connecting rooms with doors");
    let door1 = Door {
        id: door_id.to_string(),
        name: format!("Door to {}", room2.name),
        is_exit: false,
    };
    *door_id += 1;
    plan.add_door(
        plan.get_room_idx_by_id(&room1.id)?,
        plan.get_room_idx_by_id(&room2.id)?,
        door1,
    );

    let door2 = Door {
        id: door_id.to_string(),
        name: format!("Door to {}", room1.name),
        is_exit: true, // second door is always the way out
    };
    *door_id += 1;
    plan.add_door(
        plan.get_room_idx_by_id(&room2.id)?,
        plan.get_room_idx_by_id(&room1.id)?,
        door2,
    );

    debug!("Connected rooms with doors");
    Ok(())
}

fn add_rooms(
    plan: &mut FloorPlan,
    json_value: &serde_json::Value,
    namespace: &str,
    outer_room: &Room,
    door_id_generator: &mut usize,
    kind: &str,
) -> FloorPlanResult<()> {
    if let Ok(resources) = get_names(json_value, kind, namespace) {
        for r in resources {
            let room = Room {
                id: format!("{namespace}-{}-{}", r.kind, r.name),
                name: format!("{} {}", r.kind, r.name),
            };
            plan.add_room(room.clone());
            connect_rooms_with_doors(plan, &room, outer_room, door_id_generator)?;

            // if there is any parent, connect the room to the parent
            if let Some(parent) = r.parent {
                let parent_room_id = format!("{namespace}-{}-{}", parent.kind, parent.name);
                let cplan = plan.clone(); //todo: is this really necessary?
                let parent_room = cplan.get_room_by_id(&parent_room_id);
                if let Ok(parent_room) = parent_room {
                    connect_rooms_with_doors(plan, &room, parent_room, door_id_generator)?;
                } else {
                    warn!("Owner room not found: {parent_room_id}");
                }
            }

            for container in r.children {
                let container_room = Room {
                    id: format!("{namespace}-{}-{}-{}", r.kind, "container", container.name),
                    name: format!("{} {}", "container", container.name),
                };
                plan.add_room(container_room.clone());
                connect_rooms_with_doors(plan, &container_room, &room, door_id_generator)?;
                for volume_mount in container.children {
                    let volume_mount_room = Room {
                        id: format!(
                            "{namespace}-{}-{}-{}-{}",
                            r.kind, "container", container.name, volume_mount.name
                        ),
                        name: format!("{} {}", "volume mount", volume_mount.name),
                    };
                    plan.add_room(volume_mount_room.clone());
                    connect_rooms_with_doors(
                        plan,
                        &volume_mount_room,
                        &container_room,
                        door_id_generator,
                    )?;
                }
            }
        }
        Ok(())
    } else {
        Err(crate::floorplan::FloorPlanError::RoomNotFound(
            "no resources".to_string(),
        ))
    }
}

fn setup_hallway_and_rooms(
    plan: &mut FloorPlan,
    json_value: &serde_json::Value, // might want to pass this in a pre-parsed format: TODO
    namespace: &str,
    outer_room: &Room, // will often be the cluster lobby
    door_id_generator: &mut usize,
    kind: &str, // hallways collect similar resources
) -> FloorPlanResult<()> {
    let hallway = Room {
        id: format!("{namespace}-{kind}s"),
        name: format!("{namespace} {kind}s Hallway"),
    };
    plan.add_room(hallway.clone());
    connect_rooms_with_doors(plan, outer_room, &hallway, door_id_generator)?;
    add_rooms(
        plan,
        json_value,
        namespace,
        &hallway,
        door_id_generator,
        kind,
    )?;
    Ok(())
}

fn generate_k8s_floorplan_from_file() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    if let Ok(yaml_content) = fs::read_to_string("assets/k8s.yaml") {
        if let Ok(yaml_value) = serde_yaml::from_str::<Value>(&yaml_content) {
            let json_value = json!(yaml_value);

            let cluster_room = Room {
                id: "cluster".to_string(),
                name: "Cluster Lobby".to_string(),
            };
            floorplan.add_room(cluster_room.clone());

            let mut door_id = 0;
            if let Ok(namespaces) = get_namespaces(&json_value) {
                for namespace in namespaces {
                    let namespace_room = Room {
                        id: namespace.clone(),
                        name: format!("{namespace} NS Hallway"),
                    };
                    floorplan.add_room(namespace_room.clone());
                    connect_rooms_with_doors(
                        &mut floorplan,
                        &cluster_room,
                        &namespace_room,
                        &mut door_id,
                    )?;

                    for kind in &[
                        "Deployments",
                        "DaemonSets",
                        "ReplicaSets",
                        "Services",
                        "ConfigMaps",
                        "Pod",
                    ] {
                        setup_hallway_and_rooms(
                            &mut floorplan,
                            &json_value,
                            &namespace,
                            &namespace_room,
                            &mut door_id,
                            kind,
                        )?;
                    }
                }
            }
        } else {
            panic!("Failed to parse k8s yaml content");
        }

        Ok(floorplan)
    } else {
        error!("No k8s yaml file found");
        Err(crate::floorplan::FloorPlanError::RoomNotFound(
            "no file".to_string(),
        ))
    }
}

pub fn fire_k8s_file_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_k8s_floorplan_from_file() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        warn!("No K8S FloorPlanEvent");
    }
}

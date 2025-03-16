use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, FloorPlanResult, Room};
use bevy::prelude::*;

fn door_adder(
    plan: &mut FloorPlan,
    room1: &Room,
    room2: &Room,
    door: &Door,
) -> FloorPlanResult<()> {
    plan.add_door(
        plan.get_room_idx_by_id(&room1.id)?,
        plan.get_room_idx_by_id(&room2.id)?,
        door.clone(),
    );
    Ok(())
}

fn create_rooms(lim: u8) -> Vec<Room> {
    (0..lim)
        .map(|i| Room {
            id: i.to_string(),
            name: format!("Room {i}"),
        })
        .collect()
}

fn create_doors(lim: u8) -> Vec<Door> {
    (0..lim)
        .map(|i| Door {
            id: i.to_string(),
            name: format!("Door {i}"),
            is_exit: false,
        })
        .collect()
}

pub fn fire_room2_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_room2_floorplan() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        warn!("No 2Room FlooplanEvent");
    }
}

// create a 2 room floorplan
fn generate_room2_floorplan() -> FloorPlanResult<FloorPlan> {
    info!("generating 2Room FloorPlanEvent");

    let mut floorplan = FloorPlan::new();
    let rooms = create_rooms(2);

    floorplan.add_room(rooms[0].clone());
    floorplan.add_room(rooms[1].clone());

    let doors = create_doors(2);

    door_adder(&mut floorplan, &rooms[0], &rooms[1], &doors[0].clone())?;
    door_adder(&mut floorplan, &rooms[1], &rooms[0], &doors[1].clone())?;

    Ok(floorplan)
}

pub fn fire_room5_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_room5_floorplan() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        error!("No 5Room FloorPlanEvent");
    }
}

// create a 5 room floorplan
fn generate_room5_floorplan() -> FloorPlanResult<FloorPlan> {
    info!("generating 5Room FloorPlanEvent");

    let mut floorplan = FloorPlan::new();

    let rooms = create_rooms(5);

    floorplan.add_room(rooms[0].clone());
    floorplan.add_room(rooms[1].clone());
    floorplan.add_room(rooms[2].clone());
    floorplan.add_room(rooms[3].clone());
    floorplan.add_room(rooms[4].clone());

    let doors = create_doors(10);

    door_adder(&mut floorplan, &rooms[0], &rooms[1], &doors[0])?;
    door_adder(&mut floorplan, &rooms[1], &rooms[0], &doors[1])?;

    door_adder(&mut floorplan, &rooms[1], &rooms[2], &doors[2])?;
    door_adder(&mut floorplan, &rooms[2], &rooms[1], &doors[3])?;

    door_adder(&mut floorplan, &rooms[1], &rooms[3], &doors[4])?;
    door_adder(&mut floorplan, &rooms[3], &rooms[1], &doors[5])?;

    door_adder(&mut floorplan, &rooms[3], &rooms[4], &doors[6])?;
    door_adder(&mut floorplan, &rooms[4], &rooms[3], &doors[7])?;

    Ok(floorplan)
}

pub fn fire_room25_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_room25_floorplan() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        error!("No 25Room FloorPlanEvent");
    }
}

// create a 25 room floorplan
fn generate_room25_floorplan() -> FloorPlanResult<FloorPlan> {
    info!("generating 25Room FloorPlanEvent");
    let mut floorplan = FloorPlan::new();

    let rooms = create_rooms(25);

    for room in &rooms {
        floorplan.add_room(room.clone());
    }

    let doors = create_doors(60);

    door_adder(&mut floorplan, &rooms[0], &rooms[1], &doors[0])?;
    door_adder(&mut floorplan, &rooms[1], &rooms[0], &doors[1])?;

    door_adder(&mut floorplan, &rooms[0], &rooms[2], &doors[2])?;
    door_adder(&mut floorplan, &rooms[2], &rooms[0], &doors[3])?;

    door_adder(&mut floorplan, &rooms[1], &rooms[3], &doors[4])?;
    door_adder(&mut floorplan, &rooms[3], &rooms[1], &doors[5])?;

    door_adder(&mut floorplan, &rooms[2], &rooms[3], &doors[6])?;
    door_adder(&mut floorplan, &rooms[3], &rooms[2], &doors[7])?;

    door_adder(&mut floorplan, &rooms[2], &rooms[4], &doors[8])?;
    door_adder(&mut floorplan, &rooms[4], &rooms[2], &doors[9])?;

    door_adder(&mut floorplan, &rooms[3], &rooms[5], &doors[10])?;
    door_adder(&mut floorplan, &rooms[5], &rooms[3], &doors[11])?;

    door_adder(&mut floorplan, &rooms[3], &rooms[6], &doors[12])?;
    door_adder(&mut floorplan, &rooms[6], &rooms[3], &doors[13])?;

    door_adder(&mut floorplan, &rooms[3], &rooms[7], &doors[14])?;
    door_adder(&mut floorplan, &rooms[7], &rooms[3], &doors[15])?;

    door_adder(&mut floorplan, &rooms[3], &rooms[8], &doors[16])?;
    door_adder(&mut floorplan, &rooms[8], &rooms[3], &doors[17])?;

    door_adder(&mut floorplan, &rooms[8], &rooms[9], &doors[18])?;
    door_adder(&mut floorplan, &rooms[9], &rooms[8], &doors[19])?;

    door_adder(&mut floorplan, &rooms[9], &rooms[10], &doors[20])?;
    door_adder(&mut floorplan, &rooms[10], &rooms[9], &doors[21])?;

    door_adder(&mut floorplan, &rooms[9], &rooms[11], &doors[22])?;
    door_adder(&mut floorplan, &rooms[11], &rooms[9], &doors[23])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[12], &doors[24])?;
    door_adder(&mut floorplan, &rooms[12], &rooms[11], &doors[25])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[13], &doors[26])?;
    door_adder(&mut floorplan, &rooms[13], &rooms[11], &doors[27])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[14], &doors[28])?;
    door_adder(&mut floorplan, &rooms[14], &rooms[11], &doors[29])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[15], &doors[30])?;
    door_adder(&mut floorplan, &rooms[15], &rooms[11], &doors[31])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[16], &doors[32])?;
    door_adder(&mut floorplan, &rooms[16], &rooms[11], &doors[33])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[17], &doors[34])?;
    door_adder(&mut floorplan, &rooms[17], &rooms[11], &doors[31])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[18], &doors[35])?;
    door_adder(&mut floorplan, &rooms[18], &rooms[11], &doors[36])?;

    door_adder(&mut floorplan, &rooms[11], &rooms[19], &doors[37])?;
    door_adder(&mut floorplan, &rooms[19], &rooms[11], &doors[38])?;

    door_adder(&mut floorplan, &rooms[19], &rooms[20], &doors[39])?;
    door_adder(&mut floorplan, &rooms[20], &rooms[19], &doors[40])?;

    door_adder(&mut floorplan, &rooms[20], &rooms[21], &doors[41])?;
    door_adder(&mut floorplan, &rooms[21], &rooms[20], &doors[42])?;

    door_adder(&mut floorplan, &rooms[20], &rooms[22], &doors[43])?;
    door_adder(&mut floorplan, &rooms[22], &rooms[20], &doors[44])?;

    door_adder(&mut floorplan, &rooms[20], &rooms[23], &doors[45])?;
    door_adder(&mut floorplan, &rooms[23], &rooms[20], &doors[46])?;

    door_adder(&mut floorplan, &rooms[23], &rooms[24], &doors[47])?;
    door_adder(&mut floorplan, &rooms[24], &rooms[23], &doors[48])?;

    door_adder(&mut floorplan, &rooms[9], &rooms[11], &doors[49])?;
    door_adder(&mut floorplan, &rooms[11], &rooms[9], &doors[50])?;

    Ok(floorplan)
}

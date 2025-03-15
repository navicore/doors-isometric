use super::k8s_api::get_names;
use crate::cli::Cli;
use crate::floorplan::{FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use crate::integration::integration_utils::IntegrationResource;
use crate::integration::k8s_file::k8s_integration_systems::connect_rooms_with_doors;
use bevy::prelude::*;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use clap::Parser;
use kube::{
    api::{Api, ListParams},
    Client,
};
use std::time::Duration;

async fn create_k8s_client() -> FloorPlanResult<Client> {
    Client::try_default()
        .await
        .map_err(|e| crate::floorplan::FloorPlanError::ServiceError(e.to_string()))
}

async fn fetch_namespaces(
    client: &Client,
) -> FloorPlanResult<Vec<k8s_openapi::api::core::v1::Namespace>> {
    let namespaces: Api<k8s_openapi::api::core::v1::Namespace> = Api::all(client.clone());
    let lp = ListParams::default();
    namespaces
        .list(&lp)
        .await
        .map(|ns_list| ns_list.items)
        .map_err(|e| crate::floorplan::FloorPlanError::ServiceError(e.to_string()))
}

fn create_cluster_room() -> RoomData {
    RoomData {
        id: "cluster".to_string(),
        name: "Cluster Lobby".to_string(),
    }
}

/**
* for a given namespace, walk through the resources and create rooms for each and add them to the
* floorplan
*/
async fn process_namespace(
    floorplan: &mut FloorPlan,
    cluster_room: &RoomData,
    door_id: &mut usize,
    client: &Client,
    namespace: &str,
) -> FloorPlanResult<()> {
    debug!("processing namespace {namespace}");
    let namespace_room = create_namespace_room(namespace);
    floorplan.add_room(namespace_room.clone());
    connect_rooms_with_doors(floorplan, cluster_room, &namespace_room, door_id)?;

    for kind in &[
        "Deployment",
        "DaemonSet",
        "ReplicaSet",
        "Service",
        "ConfigMap",
        "Ingress",
        "Pod",
    ] {
        setup_hallway_and_rooms(floorplan, namespace, &namespace_room, door_id, kind, client)
            .await?;
    }

    Ok(())
}

fn create_namespace_room(namespace: &str) -> RoomData {
    RoomData {
        id: namespace.to_string(),
        name: format!("{namespace} NS Hallway"),
    }
}

async fn setup_hallway_and_rooms(
    plan: &mut FloorPlan,
    namespace: &str,
    outer_room: &RoomData,
    door_id_generator: &mut usize,
    kind: &str,
    client: &Client,
) -> FloorPlanResult<()> {
    debug!("Setting up {kind} hallway and rooms");
    let hallway = create_hallway_room(namespace, kind);
    plan.add_room(hallway.clone());
    connect_rooms_with_doors(plan, outer_room, &hallway, door_id_generator)?;

    add_rooms(plan, client, namespace, &hallway, door_id_generator, kind).await?;
    debug!("Finished setting up {kind} hallway and rooms");
    Ok(())
}

fn create_hallway_room(namespace: &str, kind: &str) -> RoomData {
    RoomData {
        id: format!("{namespace}-{kind}s"),
        name: format!("{namespace} {kind}s Hallway"),
    }
}

async fn add_rooms(
    plan: &mut FloorPlan,
    client: &Client,
    namespace: &str,
    outer_room: &RoomData,
    door_id_generator: &mut usize,
    kind: &str,
) -> FloorPlanResult<()> {
    debug!("Adding {kind} rooms");
    if let Ok(resources) = get_names(client, kind, namespace).await {
        for r in resources {
            let room = create_resource_room(namespace, &r);
            plan.add_room(room.clone());
            connect_rooms_with_doors(plan, &room, outer_room, door_id_generator)?;

            if let Some(parent) = &r.parent {
                connect_to_parent_room(plan, namespace, &room, parent, door_id_generator)?;
            }

            add_container_rooms(plan, namespace, &r, &room, door_id_generator)?;
        }
        debug!("Finished adding {kind} rooms");
    }
    Ok(())
}

fn create_resource_room(namespace: &str, r: &IntegrationResource) -> RoomData {
    RoomData {
        id: format!("{namespace}-{}-{}", r.kind, r.name),
        name: format!("{} {}", r.kind, r.name),
    }
}

fn connect_to_parent_room(
    plan: &mut FloorPlan,
    namespace: &str,
    room: &RoomData,
    parent: &IntegrationResource,
    door_id_generator: &mut usize,
) -> FloorPlanResult<()> {
    let parent_room_id = format!("{namespace}-{}-{}", parent.kind, parent.name);
    let cplan = plan.clone();
    let parent_room = cplan.get_room_by_id(&parent_room_id);
    if let Ok(parent_room) = parent_room {
        connect_rooms_with_doors(plan, room, parent_room, door_id_generator)?;
    } else {
        debug!("Owner room not found: {parent_room_id}");
    }
    Ok(())
}

fn add_container_rooms(
    plan: &mut FloorPlan,
    namespace: &str,
    r: &IntegrationResource,
    room: &RoomData,
    door_id_generator: &mut usize,
) -> FloorPlanResult<()> {
    for container in r.children.clone() {
        let container_room = create_container_room(namespace, r, &container);
        plan.add_room(container_room.clone());
        connect_rooms_with_doors(plan, &container_room, room, door_id_generator)?;
        add_volume_mount_rooms(
            plan,
            namespace,
            r,
            &container,
            &container_room,
            door_id_generator,
        )?;
    }
    Ok(())
}

fn create_container_room(
    namespace: &str,
    r: &IntegrationResource,
    container: &IntegrationResource,
) -> RoomData {
    RoomData {
        id: format!("{namespace}-{}-{}-{}", r.kind, "container", container.name),
        name: format!("{} {}", "container", container.name),
    }
}

fn add_volume_mount_rooms(
    plan: &mut FloorPlan,
    namespace: &str,
    r: &IntegrationResource,
    container: &IntegrationResource,
    container_room: &RoomData,
    door_id_generator: &mut usize,
) -> FloorPlanResult<()> {
    for volume_mount in container.children.clone() {
        let volume_mount_room = create_volume_mount_room(namespace, r, container, &volume_mount);
        plan.add_room(volume_mount_room.clone());
        connect_rooms_with_doors(plan, &volume_mount_room, container_room, door_id_generator)?;
    }
    Ok(())
}

fn create_volume_mount_room(
    namespace: &str,
    r: &IntegrationResource,
    container: &IntegrationResource,
    volume_mount: &IntegrationResource,
) -> RoomData {
    RoomData {
        id: format!(
            "{namespace}-{}-{}-{}-{}",
            r.kind, "container", container.name, volume_mount.name
        ),
        name: format!("{} {}", "volume mount", volume_mount.name),
    }
}

async fn generate() -> FloorPlanResult<FloorPlan> {
    let client = create_k8s_client().await?;
    let ns_list = fetch_namespaces(&client).await?;
    let mut floorplan = FloorPlan::new();
    let cluster_room = create_cluster_room();
    floorplan.add_room(cluster_room.clone());

    let mut door_id = 0;
    for ns in ns_list {
        if let Some(namespace) = ns.metadata.name {
            process_namespace(
                &mut floorplan,
                &cluster_room,
                &mut door_id,
                &client,
                &namespace,
            )
            .await?;
        }
    }

    Ok(floorplan)
}

async fn publish_floorplan(ctx: &mut TaskContext) -> FloorPlanResult<()> {
    let floorplan = generate().await?;
    ctx.run_on_main_thread(move |ctx| {
        if let Some(mut events) = ctx.world.get_resource_mut::<Events<FloorPlanEvent>>() {
            events.send(FloorPlanEvent { floorplan });
            debug!("...Generated new floorplan");
        } else {
            panic!("No FloorPlanEvent resource found");
        }
    })
    .await;
    Ok(())
}

pub fn init_k8s_live_floorplan_publisher(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        loop {
            if let Err(e) = publish_floorplan(&mut ctx).await {
                panic!("No K8S FloorPlanEvent: {e:?}");
            }
            let generator_poll_secs = Cli::parse().generator_poll_secs.unwrap_or(60);
            tokio::time::sleep(Duration::from_secs(generator_poll_secs.into())).await;
            debug!("Generating new floorplan...");
        }
    });
}

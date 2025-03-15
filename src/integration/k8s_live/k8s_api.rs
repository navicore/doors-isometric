use crate::integration::integration_utils::IntegrationResource;
use bevy::log::debug;
use kube::core::{ApiResource, DynamicObject};
use kube::{
    api::{Api, ListParams},
    Client,
};
use std::error::Error;

/**
* keep this updated for any 'kind' you want to support - note that different kinds are in different
* groups
*/
fn get_api_params(kind: &str) -> (&str, &str) {
    match kind {
        "DaemonSet" | "ReplicaSet" | "Deployment" => ("apps", "v1"),
        "Ingress" => ("networking.k8s.io", "v1"),
        _ => ("", "v1"),
    }
}

fn build_api_resource(kind: &str, group: &str, version: &str) -> ApiResource {
    let api_version = if group.is_empty() {
        version.to_string()
    } else {
        format!("{group}/{version}")
    };

    ApiResource {
        group: group.to_string(),
        version: version.to_string(),
        api_version,
        kind: kind.to_string(),
        plural: format!("{}s", kind.to_lowercase()),
    }
}

async fn fetch_resource_list(
    client: &Client,
    namespace: &str,
    resource: &ApiResource,
) -> Result<Vec<DynamicObject>, Box<dyn Error>> {
    let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), namespace, resource);
    let lp = ListParams::default();
    let resource_list = api
        .list(&lp)
        .await
        .map_err(|e| format!("Failed to list resources: {e}"))?;
    Ok(resource_list.items)
}

/**
* call for each resource type or 'kind' to extract the names of the resources
*/
fn extract_integration_resources(
    kind: &str,
    resource_list: Vec<DynamicObject>,
) -> Vec<IntegrationResource> {
    let mut resources = Vec::new();
    for resource in resource_list {
        if let Some(name) = resource.metadata.name.clone() {
            debug!("Found {kind} {name}");
            let owner_reference = get_owner_reference(&resource);
            let containers = get_containers(&resource);
            let owner = owner_reference
                .map(|(kind, name)| IntegrationResource::new(name, kind, None, Vec::new()));
            resources.push(IntegrationResource::new(
                name,
                kind.to_string(),
                owner,
                containers,
            ));
        }
    }
    resources
}

/**
 * an `owner_references` could be a replicaset to a pod or a deployment to a replicaset
*/
fn get_owner_reference(v: &DynamicObject) -> Option<(String, String)> {
    v.metadata
        .owner_references
        .as_ref()
        .and_then(|refs| refs.first())
        .map(|owner_ref| (owner_ref.kind.clone(), owner_ref.name.clone()))
}

fn get_containers(v: &DynamicObject) -> Vec<IntegrationResource> {
    v.data["spec"]["containers"]
        .as_array()
        .map(|containers| {
            containers
                .iter()
                .filter_map(|container| {
                    let container_name = container["name"].as_str().map(String::from);
                    let volume_mounts = get_volume_mounts(container);
                    container_name.map(|n| IntegrationResource {
                        name: n,
                        kind: "Container".to_string(),
                        parent: None,
                        children: volume_mounts,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_volume_mounts(container: &serde_json::Value) -> Vec<IntegrationResource> {
    container["volumeMounts"]
        .as_array()
        .map(|volume_mounts| {
            volume_mounts
                .iter()
                .filter_map(|volume_mount| {
                    volume_mount["name"].as_str().map(|n| IntegrationResource {
                        name: n.to_string(),
                        kind: "VolumeMount".to_string(),
                        parent: None,
                        children: Vec::new(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

/**
* this is the main API for the k8s api - it fetches the names of resources of a given kind in a
* given namespace
*/
pub async fn get_names(
    client: &Client,
    kind: &str,
    namespace: &str,
) -> Result<Vec<IntegrationResource>, Box<dyn Error>> {
    debug!("Getting names for {kind} in {namespace}");

    let (group, version) = get_api_params(kind);
    let resource = build_api_resource(kind, group, version);
    let resource_list = fetch_resource_list(client, namespace, &resource).await?;
    Ok(extract_integration_resources(kind, resource_list))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::log::{debug, info};
    use kube::Client;

    #[tokio::test]
    async fn test_list_namespaces() {
        let client = Client::try_default()
            .await
            .expect("Failed to create client");
        let namespaces: Api<k8s_openapi::api::core::v1::Namespace> = Api::all(client);

        match namespaces.list(&ListParams::default()).await {
            Ok(ns_list) => {
                println!("Found {} namespaces", ns_list.items.len());
                for ns in ns_list {
                    println!("Namespace: {}", ns.metadata.name.unwrap_or_default());
                }
            }
            Err(e) => {
                eprintln!("Error fetching namespaces: {}", e);
                panic!("Failed to fetch namespaces");
            }
        }
    }

    #[tokio::test]
    async fn test_get_names_pods() {
        let client = Client::try_default()
            .await
            .expect("Failed to create client");
        let namespace = "kube-system";
        let kind = "Pod";

        match get_names(&client, kind, namespace).await {
            Ok(resources) => {
                assert!(!resources.is_empty());
                println!("Found {} Pods", resources.len());
                for resource in resources {
                    println!("Pod: {}", resource.name);
                }
            }
            Err(e) => {
                eprintln!("Error fetching Pods: {}", e);
                panic!("Failed to fetch Pods");
            }
        }
    }

    #[tokio::test]
    async fn test_get_names_replicasets() {
        info!("info");
        debug!("debug");
        let client = Client::try_default()
            .await
            .expect("Failed to create client");
        let namespace = "kube-system";
        let kind = "ReplicaSet";

        match get_names(&client, kind, namespace).await {
            Ok(resources) => {
                assert!(!resources.is_empty());
                println!("Found {} ReplicaSets", resources.len());
                for resource in resources {
                    println!("ReplicaSet: {}", resource.name);
                }
            }
            Err(e) => {
                eprintln!("Error fetching {kind}: {e}");
                panic!("Failed to fetch {kind}");
            }
        }
    }

    #[tokio::test]
    async fn test_get_names_services() {
        let client = Client::try_default()
            .await
            .expect("Failed to create client");
        let namespace = "kube-system";
        let kind = "Service";

        match get_names(&client, kind, namespace).await {
            Ok(resources) => {
                assert!(!resources.is_empty());
                println!("Found {} Services", resources.len());
                for resource in resources {
                    println!("Service: {}", resource.name);
                }
            }
            Err(e) => {
                eprintln!("Error fetching Services: {}", e);
                panic!("Failed to fetch Services");
            }
        }
    }

    #[tokio::test]
    async fn test_get_names_configmaps() {
        let client = Client::try_default()
            .await
            .expect("Failed to create client");
        let namespace = "kube-system";
        let kind = "ConfigMap";

        match get_names(&client, kind, namespace).await {
            Ok(resources) => {
                assert!(!resources.is_empty());
                println!("Found {} ConfigMaps", resources.len());
                for resource in resources {
                    println!("ConfigMap: {}", resource.name);
                }
            }
            Err(e) => {
                eprintln!("Error fetching ConfigMaps: {}", e);
                panic!("Failed to fetch ConfigMaps");
            }
        }
    }
}

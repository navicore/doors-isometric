use std::{collections::HashSet, error::Error};

use jsonpath_lib::select;

use crate::integration::integration_utils::IntegrationResource;

pub fn get_namespaces(json_value: &serde_json::Value) -> Result<Vec<String>, Box<dyn Error>> {
    let namespaces: HashSet<String> = select(json_value, "$..metadata.namespace")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(namespaces.into_iter().collect())
}

fn get_owner_reference(v: &serde_json::Value) -> Option<(String, String)> {
    v["metadata"]["ownerReferences"]
        .as_array()
        .and_then(|refs| refs.first())
        .and_then(|owner_ref| {
            let owner_kind = owner_ref["kind"].as_str().map(String::from);
            let owner_name = owner_ref["name"].as_str().map(String::from);
            match (owner_kind, owner_name) {
                (Some(kind), Some(name)) => Some((kind, name)),
                _ => None,
            }
        })
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

fn get_containers(v: &serde_json::Value) -> Vec<IntegrationResource> {
    v["spec"]["containers"]
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

pub fn get_names(
    json_value: &serde_json::Value,
    kind: &str,
    namespace: &str,
) -> Result<Vec<IntegrationResource>, Box<dyn Error>> {
    let query = format!("$..[?(@.kind == '{kind}' && @.metadata.namespace == '{namespace}')]");

    let resource: Vec<IntegrationResource> = select(json_value, &query)?
        .iter()
        .filter_map(|v| {
            let name = v["metadata"]["name"].as_str().map(String::from);
            let owner_reference = get_owner_reference(v);
            let containers = get_containers(v);
            let owner = owner_reference
                .map(|(kind, name)| IntegrationResource::new(name, kind, None, Vec::new()));
            name.map(|n| IntegrationResource::new(n, kind.to_string(), owner, containers))
        })
        .collect();

    Ok(resource)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_names_with_parent_and_containers() {
        let json_value = json!({
            "items": [
                {
                    "kind": "Pod",
                    "metadata": {
                        "name": "pod1",
                        "namespace": "default",
                        "ownerReferences": [
                            {
                                "kind": "ReplicaSet",
                                "name": "rs1"
                            }
                        ]
                    },
                    "spec": {
                        "containers": [
                            {
                                "name": "container1"
                            },
                            {
                                "name": "container2"
                            }
                        ]
                    }
                },
                {
                    "kind": "Pod",
                    "metadata": {
                        "name": "pod2",
                        "namespace": "default"
                    },
                    "spec": {
                        "containers": [
                            {
                                "name": "container3"
                            }
                        ]
                    }
                }
            ]
        });

        let result = get_names(&json_value, "Pod", "default").unwrap();
        assert_eq!(result.len(), 2);

        let pod1 = &result[0];
        assert_eq!(pod1.name, "pod1");
        assert_eq!(pod1.kind, "Pod");
        assert!(pod1.parent.is_some());
        let parent = pod1.parent.as_ref().unwrap();
        assert_eq!(parent.name, "rs1");
        assert_eq!(parent.kind, "ReplicaSet");
        assert_eq!(
            pod1.children,
            vec![
                IntegrationResource::new(
                    "container1".to_string(),
                    "Container".to_string(),
                    None,
                    Vec::new()
                ),
                IntegrationResource::new(
                    "container2".to_string(),
                    "Container".to_string(),
                    None,
                    Vec::new()
                ),
            ]
        );

        let pod2 = &result[1];
        assert_eq!(pod2.name, "pod2");
        assert_eq!(pod2.kind, "Pod");
        assert!(pod2.parent.is_none());
        assert_eq!(
            pod2.children,
            vec![IntegrationResource::new(
                "container3".to_string(),
                "Container".to_string(),
                None,
                Vec::new()
            ),]
        );
    }

    #[test]
    fn test_get_names_without_parent_and_containers() {
        let json_value = json!({
            "items": [
                {
                    "kind": "Service",
                    "metadata": {
                        "name": "service1",
                        "namespace": "default"
                    }
                }
            ]
        });

        let result = get_names(&json_value, "Service", "default").unwrap();
        assert_eq!(result.len(), 1);

        let service = &result[0];
        assert_eq!(service.name, "service1");
        assert_eq!(service.kind, "Service");
        assert!(service.parent.is_none());
        assert!(service.children.is_empty());
    }
}

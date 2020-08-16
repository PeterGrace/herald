use futures::{StreamExt, TryStreamExt};
use thiserror::Error;
//use strum_macros::EnumString;
//use std::str::FromStr;
use kube::{
    api::{Api, ListParams},
    Client,
};
//use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
pub use crate::watching::watch_types::WatchTypes;
use kube_runtime::watcher;
use prometheus::HistogramVec;
use crate::models::watcher_item_spec::WatcherItemSpec;
use std::collections::{BTreeMap, HashMap};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::apps::v1::Deployment;
use kube_runtime::watcher::{Event};
use kube::api::Meta;
use tokio::sync::mpsc::{channel, Sender};
use tokio::select;

#[derive(Error, Debug)]
pub enum WatchError {
    #[error("Unknown error when watching {0}: {1}")]
    UnknownThing(String, String),
    #[error("Received targetKind that does not match a known object: {0}")]
    UnknownKind(String),
    #[error("Misc unknown: {0}")]
    Unknown(String)
}

lazy_static! {
    static ref HERALD_FN_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "herald_fn_duration_seconds",
        "The number of seconds it took to process a watcher object",
        &["function"]
    )
    .unwrap();
}
pub fn btree_to_string(input: BTreeMap<String, String>) -> String {
    let mut result = String::from("");
    for key in input.keys() {
        if result.len() == 0 {
            result = format!("{}={}", key, input.get(key).unwrap());
        } else {
            result = format!("{}, {}={}", result, key, input.get(key).unwrap());
        }
    }
    result
}

pub async fn create_specific_watcher(input_obj: WatcherItemSpec) -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    if let Some(match_labels) = input_obj.selector.unwrap().match_labels {
        let label_str = btree_to_string(match_labels);
        let lp = ListParams::default()
            .allow_bookmarks()
            .labels(label_str.as_str());
        let kind = input_obj.target_kind.unwrap();
        info!("Creating watcher for kind {} with labelstr {}", kind, label_str);
        match kind.to_lowercase().as_str() {
            "deployment" => {
                let watched: Api<Deployment> = Api::all(client);
                let mut w = watcher(watched, lp).boxed();
                while let Some(status) = w.try_next().await? {
                    match status {
                        Event::Applied(s) => info!("Detected apply on spawn-watch-object: {}", Meta::name(&s)),
                        Event::Deleted(s) => info!("Detected delete on spawned-watch-object: {}", Meta::name(&s)),
                        Event::Restarted(s) => {
                            for deployment in s.iter() {
                                info!("Detected Restart on spawned-watched-object: {}", Meta::name(deployment));
                            }
                        },
                        //_ => info!("Error: {}", WatchError::Unknown(String::from("got a watch event we don't understand")))
                    }
                }
            },
            "node" => {
                let watched: Api<Node> = Api::all(client);
                let mut w = watcher(watched, lp).boxed();
                while let Some(event) = w.try_next().await? {
                    info!("Got: {:?}", event);
                }
            },
            _ => {
                info!("{}",WatchError::UnknownKind(kind));
            }
        }
    } else {
        info!("{}",WatchError::UnknownThing(String::from("could not decode match_labels"), "die".to_string()));
    }
    Ok(())
}

pub async fn create_and_start_watchers() -> anyhow::Result<()> {
    let mut watch_registry: HashMap<String, Vec<Sender<()>>> = HashMap::new();
    let client = Client::try_default().await?;
    let lp = ListParams::default().allow_bookmarks();

    //Watcher(Watcher),
    let watched: Api<crate::models::watcher_spec::Watcher> = Api::all(client);
    let mut w = watcher(watched, lp).boxed();
    while let Some(watcher_status) = w.try_next().await? {
        match watcher_status {
            Event::Applied(s) => {
                info!("Processing delete on Watcher: {}",&s.name());
                if let Some(watch_channels) = watch_registry.get_mut(&s.name()) {
                    for watch_channel in watch_channels {
                        watch_channel.send(()).await;
                    }
                }
                let mut watch_vec: Vec<Sender<()>> = Vec::new();
                let watcher_name = s.name();
                info!("Processing apply on Watcher: {}",&s.name());
                for item in s.spec.watchers.unwrap() {
                    let (tx, mut rx) = channel(1);
                    tokio::spawn(async move {
                        select!{
                         _ = create_specific_watcher(item) => info!("watching thread exited...?"),
                         _ = rx.next() => info!("Received word we should exit")
                        }
                    });
                    watch_vec.push(tx);
                }
                watch_registry.insert(watcher_name,  watch_vec);

            },
            Event::Deleted(s) => {
                info!("Processing delete on Watcher: {}",&s.name());
                if let Some(watch_channels) = watch_registry.get_mut(&s.name()) {
                    for watch_channel in watch_channels {
                        watch_channel.send(()).await;
                }
                }
            },
            Event::Restarted(s) => {
                for object in s.iter() {
                    info!("Processing delete on Watcher: {}",&object.name());
                    // first, delete all preexisting watches for this object
                    if let Some(watch_channels) = watch_registry.get_mut(&object.name()) {
                        for watch_channel in watch_channels {
                            watch_channel.send(()).await;
                        }
                    }

                    // now, recreate all the watches
                    info!("Processing apply on Watcher: {}",&object.name());
                    let watch_list = object.spec.watchers.clone();
                    let mut watch_channels = Vec::new();
                    for item in watch_list.unwrap() {
                        let label_str = btree_to_string(item.clone().selector.unwrap().match_labels.unwrap());
                        let id = format!("{}+{}",item.clone().target_kind.unwrap(),label_str);
                        info!("Inserting channel into watch registry for key {}", id);
                        let (tx, mut rx) = channel(1);
                        tokio::spawn(async move {
                            select!{
                         _ = create_specific_watcher(item) => info!("watching thread exited...?"),
                         _ = rx.next() => info!("Received word we should exit")
                        }
                        });
                        watch_channels.push(tx);
                    }
                    watch_registry.insert(object.name(),  watch_channels);

                }
            },
            //_ => info!("Error: {}", WatchError::Unknown(String::from("got a watch event we don't understand")))
        }
    }
    Ok(())
}

// fn process_watcher(w: crate::models::watcher_spec::Watcher) -> () {
//     let timer = HERALD_FN_HISTOGRAM
//         .with_label_values(&["process_watcher"])
//         .start_timer();
//     info!("watcher: {}", Meta::name(&w));
//     for w_ in w.spec.watchers {
//         info!("watch kind: {:#?}", w_);
//     }
//     timer.observe_duration();
// }

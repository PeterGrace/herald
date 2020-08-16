use futures::{stream, StreamExt, TryStreamExt};
use thiserror::Error;
//use strum_macros::EnumString;
//use std::str::FromStr;
use kube::{
    api::{Api, ListParams},
    Client,
};
//use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
pub use crate::watching::watch_types::WatchTypes;
use kube_runtime::utils::try_flatten_applied;
use kube_runtime::watcher;
use prometheus::HistogramVec;
use crate::models::watcher_item_spec::WatcherItemSpec;
use std::collections::BTreeMap;
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::api::apps::v1::Deployment;
use kube_runtime::watcher::{Event};
use kube::api::Meta;

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
        result = format!("{}, {}={}", result, key, input.get(key).unwrap());
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
        match kind.to_lowercase().as_str() {
            "deployment" => {
                let watched: Api<Deployment> = Api::all(client);
                let mut w = watcher(watched, lp).boxed();
                while let Some(status) = w.try_next().await? {
                    match status {
                        Event::Applied(s) => info!("Added object: {}", Meta::name(&s)),
                        Event::Deleted(s) => info!("Modified object: {}", Meta::name(&s)),
                        //Event::Restarted(s) => info!("Deleted object: {}", Meta::name(&s)),
                        _ => info!("Error: {}", WatchError::Unknown(String::from("got a watch event we don't understand")))
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
    let client = Client::try_default().await?;
    let lp = ListParams::default().allow_bookmarks();

    let mut stream_vec = Vec::new();
    //Watcher(Watcher),
    stream_vec.push(
        try_flatten_applied(watcher(Api::all(client.clone()), lp.clone()))
            .map_ok(|watched| WatchTypes::Watcher(watched))
            .map_err(|e| WatchError::UnknownThing(String::from("Watcher"), e.to_string()))
            .boxed(),
    );

    let mut combined_stream = stream::select_all(stream_vec);
    loop {
        let result = combined_stream.try_next().await;
        match result {
            Ok(o) => match o {
                Some(WatchTypes::Watcher(w)) => {
                    for item in w.spec.watchers.unwrap() {
                        tokio::spawn(async move {
                            create_specific_watcher(item).await;
                        });
                    }
                },
                Some(_) => (),
                None => info!("Error on reading"),
            },
            Err(e) => {
                info!("Error: {}", e);
            }
        }
    }
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

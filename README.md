# Herald
![Build](https://github.com/PeterGrace/herald/workflows/Build/badge.svg)
### Intro
Herald is a project that aims to monitor specified Kubernetes objects,
and issue webhooks to a specified service based on a template string.

### Why
I am writing Herald because I had a need for this application, and because I
have been looking for a legitimate project to cut my teeth on Rust.  This app is
an attempt to get better at writing Rust code while also providing a service that
I need.

### How it works
  - a CRD (Watcher) is specified that lists a specific Kubernetes `kind`,
  a labelSelector, one or more target namespaces, and finally a set of info
  about a webhook destination.
  - When Herald detects a modification to a kind, that matches the namespaces
  and labelselector,it will emit a webhook as specified to a remote service,
  e.g. slack or discord

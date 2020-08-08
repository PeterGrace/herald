# HelmReleaseSpecRollback

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**disable_hooks** | Option<**bool**> | If set, will prevent hooks from running during rollback | [optional]
**enable** | Option<**bool**> | If set, will perform rollbacks for this release on upgrade failures | [optional]
**force** | Option<**bool**> | If set, will force resource update through delete/recreate if needed | [optional]
**max_retries** | Option<**i64**> | The maximum amount of retries that should be attempted for a rolled back release if retries are enabled, defaults to 5, 0 equals infinite | [optional]
**recreate** | Option<**bool**> |  | [optional]
**retry** | Option<**bool**> | If set, the upgrade of a rolled back release will be retried until the maximum amount of retries is reached | [optional]
**timeout** | Option<**i64**> | Time in seconds to wait for any individual Kubernetes operation, defaults to 300 seconds | [optional]
**wait** | Option<**bool**> | If set, will wait until the minimum number of Pods of a Deployment are in a ready state before marking the release as successful | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



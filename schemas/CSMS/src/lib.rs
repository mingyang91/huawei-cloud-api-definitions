#[cfg(feature = "BatchCreateOrDeleteTags")]
pub mod BatchCreateOrDeleteTags;
#[cfg(feature = "CreateSecret")]
pub mod CreateSecret;
#[cfg(feature = "CreateSecretEvent")]
pub mod CreateSecretEvent;
#[cfg(feature = "CreateSecretTag")]
pub mod CreateSecretTag;
#[cfg(feature = "CreateSecretVersion")]
pub mod CreateSecretVersion;
#[cfg(feature = "DeleteSecret")]
pub mod DeleteSecret;
#[cfg(feature = "DeleteSecretEvent")]
pub mod DeleteSecretEvent;
#[cfg(feature = "DeleteSecretForSchedule")]
pub mod DeleteSecretForSchedule;
#[cfg(feature = "DeleteSecretStage")]
pub mod DeleteSecretStage;
#[cfg(feature = "DeleteSecretTag")]
pub mod DeleteSecretTag;
#[cfg(feature = "DownloadSecretBlob")]
pub mod DownloadSecretBlob;
#[cfg(feature = "ListNotificationRecords")]
pub mod ListNotificationRecords;
#[cfg(feature = "ListProjectSecretsTags")]
pub mod ListProjectSecretsTags;
#[cfg(feature = "ListResourceInstances")]
pub mod ListResourceInstances;
#[cfg(feature = "ListSecretEvents")]
pub mod ListSecretEvents;
#[cfg(feature = "ListSecrets")]
pub mod ListSecrets;
#[cfg(feature = "ListSecretTags")]
pub mod ListSecretTags;
#[cfg(feature = "ListSecretVersions")]
pub mod ListSecretVersions;
#[cfg(feature = "RestoreSecret")]
pub mod RestoreSecret;
#[cfg(feature = "RotateSecret")]
pub mod RotateSecret;
#[cfg(feature = "ShowSecret")]
pub mod ShowSecret;
#[cfg(feature = "ShowSecretEvent")]
pub mod ShowSecretEvent;
#[cfg(feature = "ShowSecretStage")]
pub mod ShowSecretStage;
#[cfg(feature = "ShowSecretVersion")]
pub mod ShowSecretVersion;
#[cfg(feature = "UpdateSecret")]
pub mod UpdateSecret;
#[cfg(feature = "UpdateSecretEvent")]
pub mod UpdateSecretEvent;
#[cfg(feature = "UpdateSecretStage")]
pub mod UpdateSecretStage;
#[cfg(feature = "UpdateVersion")]
pub mod UpdateVersion;
#[cfg(feature = "UploadSecretBlob")]
pub mod UploadSecretBlob;

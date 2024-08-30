#[cfg(feature = "BatchCreateVolumeTags")]
pub mod BatchCreateVolumeTags;
#[cfg(feature = "BatchDeleteVolumeTags")]
pub mod BatchDeleteVolumeTags;
#[cfg(feature = "ChangeVolumeChargeMode")]
pub mod ChangeVolumeChargeMode;
#[cfg(feature = "CinderAcceptVolumeTransfer")]
pub mod CinderAcceptVolumeTransfer;
#[cfg(feature = "CinderCreateVolumeTransfer")]
pub mod CinderCreateVolumeTransfer;
#[cfg(feature = "CinderDeleteVolumeTransfer")]
pub mod CinderDeleteVolumeTransfer;
#[cfg(feature = "CinderListAvailabilityZones")]
pub mod CinderListAvailabilityZones;
#[cfg(feature = "CinderListQuotas")]
pub mod CinderListQuotas;
#[cfg(feature = "CinderListVolumeTransfers")]
pub mod CinderListVolumeTransfers;
#[cfg(feature = "CinderListVolumeTypes")]
pub mod CinderListVolumeTypes;
#[cfg(feature = "CinderShowVolumeTransfer")]
pub mod CinderShowVolumeTransfer;
#[cfg(feature = "CreateSnapshot")]
pub mod CreateSnapshot;
#[cfg(feature = "CreateVolume")]
pub mod CreateVolume;
#[cfg(feature = "DeleteSnapshot")]
pub mod DeleteSnapshot;
#[cfg(feature = "DeleteVolume")]
pub mod DeleteVolume;
#[cfg(feature = "ListSnapshots")]
pub mod ListSnapshots;
#[cfg(feature = "ListVersions")]
pub mod ListVersions;
#[cfg(feature = "ListVolumes")]
pub mod ListVolumes;
#[cfg(feature = "ListVolumesByTags")]
pub mod ListVolumesByTags;
#[cfg(feature = "ListVolumeTags")]
pub mod ListVolumeTags;
#[cfg(feature = "ModifyVolumeQoS")]
pub mod ModifyVolumeQoS;
#[cfg(feature = "ResizeVolume")]
pub mod ResizeVolume;
#[cfg(feature = "RetypeVolume")]
pub mod RetypeVolume;
#[cfg(feature = "RollbackSnapshot")]
pub mod RollbackSnapshot;
#[cfg(feature = "ShowJob")]
pub mod ShowJob;
#[cfg(feature = "ShowSnapshot")]
pub mod ShowSnapshot;
#[cfg(feature = "ShowVersion")]
pub mod ShowVersion;
#[cfg(feature = "ShowVolume")]
pub mod ShowVolume;
#[cfg(feature = "ShowVolumeTags")]
pub mod ShowVolumeTags;
#[cfg(feature = "UnsubscribePostpaidVolume")]
pub mod UnsubscribePostpaidVolume;
#[cfg(feature = "UpdateSnapshot")]
pub mod UpdateSnapshot;
#[cfg(feature = "UpdateVolume")]
pub mod UpdateVolume;

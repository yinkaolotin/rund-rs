use crate::internal::common::{Error, ErrorType};

use std::{convert::TryFrom, fs, path::Path};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The base configuration for the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    /// Version of the Open Container Initiative Runtime Specification with which the bundle complies.
    pub oci_version: String,

    /// Configures callbacks for container lifecycle events.
    pub hooks: Option<Hooks>,

    /// Contains arbitrary metadata for the container.
    pub annotations: Option<HashMap<String, String>>,

    /// Configures the container's hostname.
    pub hostname: Option<String>,

    /// Configures the container's domainname.
    pub domain_name: Option<String>,

    /// Configures additional mounts (on top of Root). The runtime MUST mount entries in the listed order.
    pub mounts: Option<Vec<Mount>>,

    /// Configures the container's root filesystem.
    pub root: Option<Root>,

    /// Configures the container process.
    pub process: Option<Process>,

    /// Platform-specific configuration for Linux based containers.
    pub linux: Option<Linux>,
}

impl TryFrom<&Path> for Spec {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let json_str = fs::read_to_string(path).map_err(|_| Self::Error {
            err_type: ErrorType::Runtime,
            msg: "spec file not found".to_string(),
        })?;
        let spec = serde_json::from_str(&json_str).map_err(|_| Self::Error {
            err_type: ErrorType::Runtime,
            msg: "failed to deserialise spec from config file".to_string(),
        })?;
        Ok(spec)
    }
}

/// Hooks specifies a command that is run in the container at a particular event in the lifecycle of a container
/// Hooks MUST be called in the listed order. The state of the container MUST be passed to hooks over stdin so that they may do work appropriate to the current state of the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hooks {
    /// List of hooks to be run after the container has been created but before pivot_root or any equivalent operation has been called. It is called in the Runtime Namespace
    pub create_runtime: Option<Vec<Hook>>,

    /// List of hooks to be run after the container has been created but before pivot_root or any equivalent operation has been called/ It is called in the Container Namespace
    pub create_container: Option<Vec<Hook>>,

    /// List of hooks to be run after the start operation is called but before the container process is started. It is called in the Container Namespace
    pub start_container: Option<Vec<Hook>>,

    /// List of hooks to be run after the container process is started. It is called in the Runtime Namespace
    pub poststart: Option<Vec<Hook>>,

    /// List of hooks to be run after the container process exits. It is called in the Runtime Namespace
    pub poststop: Option<Vec<Hook>>,
}

/// Specifies a command that is run at a particular event in the lifecycle of a container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    /// absolute path
    pub path: String,

    pub args: Option<Vec<String>>,

    pub env: Option<Vec<String>>,

    /// the number of seconds before aborting the hook.
    pub timeout: Option<i64>,
}

/// Specifies a mount for a container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    /// The absolute path where the mount will be placed in the container.
    pub destination: String,

    ///	 Specifies the source path of the mount.
    pub source: Option<String>,

    /// fstab style mount options of the filesystem to be used.
    pub options: Option<Vec<String>>,

    /// specifies the mount kind.
    #[serde(rename = "type")]
    pub mount_type: Option<String>,
    /*
    other fields
    */
}

/// Contains information about the container's root filesystem on the host.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    /// The absolute path to the container's root filesystem.
    pub path: String,

    /// Makes the root filesystem for the container readonly before the process is executed.
    pub readonly: Option<bool>,
}

/// contains information to start a specific application inside the container.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    /// Specifies the binary and arguments for the application to execute.
    pub args: Option<Vec<String>>,

    /// The current working directory for the process. Must be relative to the container's root(absolute).
    pub cwd: String,

    /// Populates the process environment for the process.
    pub env: Option<Vec<String>>,

    /// Creates an interactive terminal for the container.
    pub terminal: Option<bool>,

    /// Specifies user information for the process.
    pub user: User,

    /// Linux capabilities that are kept for the process.
    pub capabilities: Option<Capabilities>,

    /// Specifies the apparmor profile for the container.
    pub apparmor_profile: Option<String>,

    /// Specifies an oom_score_adj for the container.
    pub oom_score_adj: Option<i64>,

    /// Specifies the selinux context that the container process is run as.
    pub selinux_label: Option<String>,

    /// Controls whether additional privileges could be gained by processes in the container.
    pub no_new_privileges: Option<bool>,

    /// Specifies rlimit options to apply to the process.
    pub rlimits: Option<Vec<Rlimit>>,
    /*
      pub io_priority: Option<IoPriority>,
      pub scheduler: Option<Scheduler>,
      pub console_size: Option<ConsoleSize>,
      pub exec_cpu_affinity: Option<ExecCPUAffinity>,
    */
}

/// Specifies specific user (and group) information for the container process.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The user id.
    pub uid: Option<u32>,

    /// The group id
    pub gid: Option<u32>,

    /// Additional group ids set for the container's process.
    pub additional_gids: Option<Vec<u32>>,
    /*
    pub umask: Option<u32>,
    */
}

/// LinuxCapabilities specifies the list of allowed capabilities that are kept for a process.
/// https://man7.org/linux/man-pages/man7/capabilities.7.html
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// The set of capabilities checked by the kernel.
    pub bounding: Option<Vec<String>>,

    /// The limiting superset for effective capabilities.
    pub permitted: Option<Vec<String>>,

    /// The set of capabilities checked by the kernel.
    pub effective: Option<Vec<String>>,

    /// The capabilities preserved across execve.
    pub inheritable: Option<Vec<String>>,

    /// The ambient set of capabilities that are kept.
    pub ambient: Option<Vec<String>>,
}

/// POSIXRlimit type and restrictions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    /// Type of the rlimit to set
    #[serde(rename = "type")]
    pub rlimit_type: String,

    /// The soft limit for the specified type
    pub soft: u64,

    /// The hard limit for the specified type
    pub hard: u64,
}

/// Contains platform-specific configuration for Linux based containers.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    /// list of device nodes that are created for the container
    pub devices: Option<Vec<Device>>,

    /// Specifies user mappings for supporting user namespaces.
    pub uid_mappings: Option<Vec<IDMapping>>,

    /// Specifies group mappings for supporting user namespaces.
    pub gid_mappings: Option<Vec<IDMapping>>,

    /// Set of key value pairs that are set for the container on start
    pub sysctl: Option<HashMap<String, String>>,

    /// Contain cgroup information for handling resource constraints for the container
    pub resources: Option<Resources>,

    /// Specifies the path to cgroups that are created and/or joined by the container.
    // The path is expected to be relative to the cgroups mountpoint.
    // If resources are specified, the cgroups at CgroupsPath will be updated based on resources.
    pub cgroups_path: Option<String>,

    /// Specifies the seccomp security settings for the container.
    pub seccomp: Option<Seccomp>,

    /// the rootfs mount propagation mode for the container.
    pub rootfs_propagation: Option<String>,

    /// Contains the namespaces that are created and/or joined by the container
    pub namespaces: Option<Vec<Namespace>>,

    /// Masks over the provided paths inside the container.
    pub masked_paths: Option<Vec<String>>,

    /// Sets the provided paths as RO inside the container.
    pub readonly_paths: Option<Vec<String>>,

    /// Specifies the selinux context for the mounts in the container.
    pub mount_label: Option<String>,
    /*
    pub intel_rdt: Option<IntelRdt>,
    pub personality: Option<Personality>,
    pub time_offsets: Option<HashMap<String, TimeOffset>>,
    */
}

/// Represents the mknod information for a Linux special device file
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Path to the device.
    pub path: String,

    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    pub device_type: String,

    /// The device's major number.
    pub major: i64,

    /// The device's minor number.
    pub minor: i64,

    /// permission bits for the device.
    pub file_mode: Option<u32>,

    /// UID of the device.
    pub uid: Option<u32>,

    /// Gid of the device.
    pub gid: Option<u32>,
}

/// Specifies UID/GID mappings
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IDMapping {
    /// The starting UID/GID in the container
    #[serde(rename = "containerID")]
    pub container_id: u32,

    /// The starting UID/GID on the host to be mapped to 'ContainerID'
    #[serde(rename = "hostID")]
    pub host_id: u32,

    /// The number of IDs to be mapped
    pub size: u32,
}

/// Has container runtime resource constraints
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resources {
    /// Configures the device allowlist.
    pub devices: Option<Vec<DeviceCgroup>>,

    /// Memory restriction configuration
    pub memory: Option<Memory>,

    /// CPU resource restriction configuration
    pub cpu: Option<CPU>,

    /// Task resource restriction configuration.
    pub pids: Option<Pids>,

    /// BlockIO restriction configuration
    #[serde(rename = "blockIO")]
    pub block_io: Option<BlockIO>,

    /// Hugetlb limits (in bytes). Default to reservation limits if supported.
    pub hugepage_limits: Option<Vec<HugepageLimit>>,

    /// Network restriction configuration
    pub network: Option<Network>,
    /*
    pub rdma: Option<HashMap<String, Rdma>>,
    pub unified: Option<HashMap<String, String>>,
    */
}

/// Represents a device rule for the devices specified to the device controller
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCgroup {
    /// Allow or deny
    pub allow: bool,

    /// Cgroup access permissions format, rwm.
    pub access: Option<String>,

    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    pub device_type: Option<String>,

    /// The device's major number.
    pub major: Option<i64>,

    /// The device's minor number.
    pub minor: Option<i64>,
}

/// For Linux cgroup 'memory' resource management
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    /// Memory limit (in bytes).
    pub limit: Option<i64>,

    /// Memory reservation or soft_limit (in bytes).
    pub reservation: Option<i64>,

    /// Total memory limit (memory + swap).
    pub swap: Option<i64>,

    // Kernel memory limit for tcp (in bytes)
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,

    // How aggressive the kernel will swap memory pages.
    pub swappiness: Option<u64>,

    // DisableOOMKiller disables the OOM killer for out of memory conditions
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    /*
    other fields
    */
}

/// LinuxCPU for Linux cgroup 'cpu' resource management
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CPU {
    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    pub shares: Option<u64>,

    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    pub quota: Option<i64>,

    /// CPU period to be used for hardcapping (in usecs).
    pub period: Option<u64>,

    /// How much time realtime scheduling may use (in usecs).
    pub realtime_runtime: Option<i64>,

    /// CPU period to be used for realtime scheduling (in usecs).
    pub realtime_period: Option<u64>,

    /// CPUs to use within the cpuset. Default is to use any CPU available.
    pub cpus: Option<String>,

    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    pub mems: Option<String>,
    /*
    other fields
    */
}

/// LinuxPids for Linux cgroup 'pids' resource management (Linux 4.3)
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pids {
    /// Maximum number of PIDs. Default is "no limit".
    pub limit: i64,
}

/// For Linux cgroup 'blkio' resource management
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockIO {
    /// Specifies per cgroup weight
    pub weight: Option<u16>,

    /// Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only
    pub leaf_weight: Option<u16>,

    /// Weight per cgroup per device, can override BlkioWeight
    pub weight_device: Option<Vec<WeightDevice>>,
    /*
    throttle devices
    */
}

/// Holds a `major:minor weight` pair for weightDevice
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightDevice {
    /// The device's major number.
    pub major: i64,

    /// The device's minor number.
    pub minor: i64,

    /// The bandwidth rate for the device.
    pub weight: Option<u16>,

    /// The bandwidth rate for the device while competing with the cgroup's child cgroups, CFQ scheduler only
    pub leaf_weight: Option<u16>,
}

/// Corresponds to limiting kernel hugepages. Default to reservation limits if supported. Otherwise fallback to page fault limits.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HugepageLimit {
    /// The hugepage size. Format: "<size><unit-prefix>B' (e.g. 64KB, 2MB, 1GB, etc.).
    pub page_size: String,

    // The limit of "hugepagesize" hugetlb reservations (if supported) or usage.
    pub limit: u64,
}

/// Identification and priority configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    /// Set class identifier for container's network packets
    #[serde(rename = "classID")]
    pub class_id: Option<u32>,

    /// Set priority of network traffic for container
    pub priorities: Option<Vec<Priority>>,
}

/// For network interfaces
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    /// The name of the network interface
    pub name: String,

    /// Priority for the interface
    pub priority: u32,
}

/// Represents syscall restrictions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seccomp {
    pub default_action: LinuxSeccompAction,
    pub architectures: Option<Vec<Arch>>,
    pub syscalls: Option<Vec<Syscall>>,
    /*
     other fields
    */
}

/// Used to match a syscall in Seccomp
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Syscall {
    pub names: Vec<String>,
    pub action: LinuxSeccompAction,
    /*
    other fields
    */
}

/// The configuration for a Linux namespace
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    /// The type of namespace
    #[serde(rename = "type")]
    pub ns_type: NamespaceType,

    /// A path to an existing namespace persisted on disk that can be joined and is of the same type
    pub path: Option<String>,
}

/// Additional architectures to be used for system calls. By default only the native architecture of the kernel is permitted
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Arch {
    #[serde(rename = "SCMP_ARCH_X86")]
    X86,
    #[serde(rename = "SCMP_ARCH_X86_64")]
    X86_64,
    #[serde(rename = "SCMP_ARCH_X32")]
    X32,
}

/// SeccompAction taken upon Seccomp rule match
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinuxSeccompAction {
    #[serde(rename = "SCMP_ACT_KILL")]
    Kill,
    #[serde(rename = "SCMP_ACT_KILL_PROCESS")]
    KillProcess,
    #[serde(rename = "SCMP_ACT_KILL_THREAD")]
    KillThread,
    #[serde(rename = "SCMP_ACT_TRAP")]
    Trap,
    #[serde(rename = "SCMP_ACT_ERRNO")]
    Errno,
    #[serde(rename = "SCMP_ACT_TRACE")]
    Trace,
    #[serde(rename = "SCMP_ACT_ALLOW")]
    Allow,
    #[serde(rename = "SCMP_ACT_LOG")]
    Log,
    #[serde(rename = "SCMP_ACT_NOTIFY")]
    Notify,
}

/// One of the Linux namespaces
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NamespaceType {
    #[serde(rename = "pid")]
    PID,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "mount")]
    Mount,
    #[serde(rename = "ipc")]
    IPC,
    #[serde(rename = "uts")]
    UTS,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "cgroup")]
    Cgroup,
    #[serde(rename = "time")]
    Time,
}

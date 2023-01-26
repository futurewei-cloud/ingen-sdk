/// Emulated Guest Process Environment
#[derive(Debug)]
pub struct EgpEnv {
    /// Data folders for the entire VM. The files in this folder will be visible to all processes within the same VM.
    /// > NOTE: This field will never be empty.
    pub vm_data_folders: Vec<String>,

    /// Data folders for this process. The files in this folder will be visible to this process only, unless explicitly set to VM folder by the host.
    /// > NOTE: This field will never be empty.
    pub process_data_folders: Vec<String>,

    // Location metadata
    /// Region where the VM is running in.
    /// > NOTE: Will be empty, if not set. We are not using Option here to make it easier to use, as most of the time, it will be used in metrics/logs.
    pub region: String,

    /// Data center where the VM is running in.
    /// > NOTE: Will be empty, if not set. We are not using Option here to make it easier to use, as most of the time, it will be used in metrics/logs.
    pub dc: String,

    /// Cluster where the VM is running in.
    /// > NOTE: Will be empty, if not set. We are not using Option here to make it easier to use, as most of the time, it will be used in metrics/logs.
    pub cluster: String,

    /// Host where the VM is running in.
    /// Although this info is usually not exposed to end user, but for emulation purpose, we expose it in case it is used.
    /// > NOTE: Will be empty, if not set. We are not using Option here to make it easier to use, as most of the time, it will be used in metrics/logs.
    pub host: String,

    // VM metadata
    /// Name of the VM.
    /// > NOTE: This field will never be empty.
    pub vm_name: String,

    /// Owner of the VM.
    /// > NOTE: Will be empty, if not set. We are not using Option here to make it easier to use, as most of the time, it will be used in metrics/logs.
    pub vm_owner: String,

    /// VM role. This is used for grouping the VMs with the same set of functionality, e.g. worker-nodes.
    /// > NOTE: This field will never be empty. If not set by upstream service, we will use VM name as VM role.
    pub vm_role: String,

    // IP
    /// Primary V4 IP of this VM. It will be the first IP on the first NIC for this VM. If no address is given, it will be set to None.
    pub primary_ip_v4: Option<String>,

    /// Primary V6 IP of this VM. It will be the first IP on the first NIC for this VM. If no address is given, it will be set to None.
    pub primary_ip_v6: Option<String>,
}

impl EgpEnv {
    /// Parse current environment variables related to emulated guest process execution envirionment.
    pub fn parse() -> Self {
        let mut env = EgpEnv {
            vm_data_folders: Self::parse_folders_to_vec ("EVM_DATA_FOLDER"),
            process_data_folders: Self::parse_folders_to_vec ("EGP_DATA_FOLDER"),

            region: std::env::var("EVM_LOC_REGION").unwrap_or_default(),
            dc: std::env::var("EVM_LOC_DC").unwrap_or_default(),
            cluster: std::env::var("EVM_LOC_CLUSTER").unwrap_or_default(),
            host: std::env::var("EVM_LOC_HOST").unwrap_or_default(),

            vm_name: std::env::var("EVM_NAME").expect("Failed to get VM name"),
            vm_owner: std::env::var("EVM_OWNER").unwrap_or_default(),
            vm_role: std::env::var("EVM_ROLE").expect("Failed to get VM role"),

            primary_ip_v4: Default::default(),
            primary_ip_v6: Default::default(),
        };

        let server_addresses_v4 = std::env::var("EVM_PRIVATE_IPV4").unwrap_or_default();
        if !server_addresses_v4.is_empty() {
            env.primary_ip_v4 = server_addresses_v4.split(',').next().map(|x| x.to_string());
        }

        let server_addresses_v6 = std::env::var("EVM_PRIVATE_IPV6").unwrap_or_default();
        if !server_addresses_v6.is_empty() {
            env.primary_ip_v6 = server_addresses_v6.split(',').next().map(|x| x.to_string());
        }

        env
    }

    fn parse_folders_to_vec (data_folders_env_name: &str) -> Vec<String> {
        let data_folders_env_value = std::env::var(data_folders_env_name).expect("Failed to get data folder");
        let folders_vec = data_folders_env_value.split(',').collect::<Vec<&str>>();

        let mut folders_vec_string = Vec::new ();
        for folder in folders_vec {
            folders_vec_string.push (folder.to_string ());
        }
        folders_vec_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn egp_env_can_parse_valid_config() {
        std::env::set_var("EVM_DATA_FOLDER", "/data");
        std::env::set_var("EGP_DATA_FOLDER", "/process_data");
        std::env::set_var("EVM_LOC_REGION", "test-region");
        std::env::set_var("EVM_LOC_DC", "test-dc");
        std::env::set_var("EVM_LOC_CLUSTER", "test-cluster");
        std::env::set_var("EVM_LOC_HOST", "test-host");
        std::env::set_var("EVM_NAME", "test-vm");
        std::env::set_var("EVM_OWNER", "test-owner");
        std::env::set_var("EVM_ROLE", "test-role");
        std::env::set_var("EVM_PRIVATE_IPV4", "10.0.0.1");
        std::env::set_var("EVM_PRIVATE_IPV6", "[0001::1]");

        let env = EgpEnv::parse();
        assert_eq!(env.vm_data_folders, vec!["/data".to_string()]);
        assert_eq!(env.process_data_folders, vec!["/process_data".to_string()]);
        assert_eq!(env.region, "test-region");
        assert_eq!(env.dc, "test-dc");
        assert_eq!(env.cluster, "test-cluster");
        assert_eq!(env.host, "test-host");
        assert_eq!(env.vm_name, "test-vm");
        assert_eq!(env.vm_owner, "test-owner");
        assert_eq!(env.vm_role, "test-role");
        assert_eq!(env.primary_ip_v4, Some("10.0.0.1".to_string()));
        assert_eq!(env.primary_ip_v6, Some("[0001::1]".to_string()));
    }

    #[test]
    fn egp_env_can_parse_multiple_ips() {
        std::env::set_var("EVM_DATA_FOLDER", "/data");
        std::env::set_var("EGP_DATA_FOLDER", "/process_data");
        std::env::set_var("EVM_NAME", "test-vm");
        std::env::set_var("EVM_ROLE", "test-role");
        std::env::set_var("EVM_PRIVATE_IPV4", "10.0.0.1,10.0.0.2,10.0.0.3");
        std::env::set_var("EVM_PRIVATE_IPV6", "[0001::1],[0001::2],[0001::3]");

        std::env::remove_var("EVM_LOC_REGION");
        std::env::remove_var("EVM_LOC_DC");
        std::env::remove_var("EVM_LOC_CLUSTER");
        std::env::remove_var("EVM_LOC_HOST");
        std::env::remove_var("EVM_OWNER");

        let env = EgpEnv::parse();
        assert_eq!(env.vm_data_folders, vec!["/data".to_string()]);
        assert_eq!(env.process_data_folders, vec!["/process_data".to_string()]);
        assert_eq!(env.vm_name, "test-vm");
        assert_eq!(env.vm_role, "test-role");
        assert_eq!(env.primary_ip_v4, Some("10.0.0.1".to_string()));
        assert_eq!(env.primary_ip_v6, Some("[0001::1]".to_string()));
    }

    #[test]
    fn egp_env_can_parse_empty_config() {
        std::env::set_var("EVM_DATA_FOLDER", "/data");
        std::env::set_var("EGP_DATA_FOLDER", "/process_data");
        std::env::set_var("EVM_LOC_REGION", "");
        std::env::set_var("EVM_LOC_DC", "");
        std::env::set_var("EVM_LOC_CLUSTER", "");
        std::env::set_var("EVM_LOC_HOST", "");
        std::env::set_var("EVM_NAME", "test-vm");
        std::env::set_var("EVM_OWNER", "");
        std::env::set_var("EVM_ROLE", "test-role");
        std::env::set_var("EVM_PRIVATE_IPV4", "");
        std::env::set_var("EVM_PRIVATE_IPV6", "");

        let env = EgpEnv::parse();
        assert_eq!(env.vm_data_folders, vec!["/data".to_string()]);
        assert_eq!(env.process_data_folders, vec!["/process_data".to_string()]);
        assert_eq!(env.region, "");
        assert_eq!(env.dc, "");
        assert_eq!(env.cluster, "");
        assert_eq!(env.host, "");
        assert_eq!(env.vm_name, "test-vm");
        assert_eq!(env.vm_owner, "");
        assert_eq!(env.vm_role, "test-role");
        assert!(env.primary_ip_v4.is_none());
        assert!(env.primary_ip_v6.is_none());
    }

    #[test]
    fn egp_env_can_parse_unset_config() {
        std::env::set_var("EVM_DATA_FOLDER", "/data");
        std::env::set_var("EGP_DATA_FOLDER", "/process_data");
        std::env::set_var("EVM_NAME", "test-vm");
        std::env::set_var("EVM_ROLE", "test-role");

        std::env::remove_var("EVM_LOC_REGION");
        std::env::remove_var("EVM_LOC_DC");
        std::env::remove_var("EVM_LOC_CLUSTER");
        std::env::remove_var("EVM_LOC_HOST");
        std::env::remove_var("EVM_OWNER");
        std::env::remove_var("EVM_PRIVATE_IPV4");
        std::env::remove_var("EVM_PRIVATE_IPV6");

        let env = EgpEnv::parse();
        assert_eq!(env.vm_data_folders, vec!["/data".to_string()]);
        assert_eq!(env.process_data_folders, vec!["/process_data".to_string()]);
        assert_eq!(env.vm_name, "test-vm");
        assert_eq!(env.vm_role, "test-role");

        assert_eq!(env.region, "");
        assert_eq!(env.dc, "");
        assert_eq!(env.cluster, "");
        assert_eq!(env.host, "");
        assert_eq!(env.vm_owner, "");
        assert!(env.primary_ip_v4.is_none());
        assert!(env.primary_ip_v6.is_none());
    }
}

use std::time::Duration;
use snmp::{SyncSession, Value};

const AGENT_ADDRESS: &str = "192.168.1.6:161";
const COMMUNITY: &[u8] = b"hellothere";

fn router_info() -> String {
    let timeout = Duration::from_secs(3);
    let uptime_oid = &[1, 3, 6, 1, 2, 1, 1, 3, 0];
    
    let mut session = SyncSession::new(AGENT_ADDRESS, COMMUNITY, Some(timeout), 0)
        .expect("Failed to create SNMP session");
    
    // Using `get` to fetch the actual OID's value, instead of `getnext`
    let response = session.get(uptime_oid).expect("Failed to get OID");

    if let Some((_oid, Value::Timeticks(uptime))) = response.varbinds.into_iter().next() {
        let uptime_in_seconds = uptime as u64 / 100;  // Convert Timeticks to seconds
        let hours = uptime_in_seconds / 3600;
        let minutes = (uptime_in_seconds % 3600) / 60;
        let seconds = uptime_in_seconds % 60;
        
        // Format the output string
        return format!(
            "Router uptime: {} ticks ({} hours, {} minutes, {} seconds)", 
            uptime, hours, minutes, seconds
        );
    }

    // Return an error message if no valid response was received
    "Failed to retrieve uptime".to_string()
}

fn main() {
    let info = router_info();
    println!("{}", info);
}

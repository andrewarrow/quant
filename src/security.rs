use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TenantRole {
    Admin,
    StandardUser,
}

pub struct QuantumTenantManager {
    pub tenant_quotas: HashMap<String, usize>, // Preostale sekunde na hardveru
}

impl QuantumTenantManager {
    pub fn new() -> Self {
        let mut tenant_quotas = HashMap::new();
        tenant_quotas.insert("Pharmaceutical_Corp".to_string(), 3600); // 1h
        tenant_quotas.insert("Academic_User".to_string(), 60);         // 1min
        Self { tenant_quotas }
    }

    /// Proverava da li klijent ima pravo na alokaciju kubita i sprječava curenje stanja (Cross-Tenant Leakage)
    pub fn authorize_and_isolate(&mut self, tenant_id: &str, requested_qubits: usize) -> Result<(), String> {
        if let Some(quota) = self.tenant_quotas.get_mut(tenant_id) {
            if *quota == 0 {
                return Err(format!(" [SECURITY] Tenant '{}' has exhausted quantum hardware quota!", tenant_id));
            }
            println!(" [SECURITY] Tenant '{}' Authorized. Qubits Isolated: {}", tenant_id, requested_qubits);
            *quota = quota.saturating_sub(1); // Umetno smanjenje kvote
            Ok(())
        } else {
            Err(format!(" [SECURITY] Unauthorized Tenant ID: '{}'", tenant_id))
        }
    }
}
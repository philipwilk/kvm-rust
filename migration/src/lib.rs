pub use sea_orm_migration::prelude::*;

pub mod identities;
mod m20221020_174326_host_table;
mod m20221020_181400_running_vms_table;
mod m20221022_213306_vdev_vm_uuid_table;
mod m20221022_213722_vdevs_table;
mod m20221022_214219_vdisks_table;
mod m20221022_214350_vdisks_vm_uuid_table;
mod m20221022_214851_vm_templates_table;
mod m20221022_222629_vms_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221020_174326_host_table::Migration),
            Box::new(m20221020_181400_running_vms_table::Migration),
            Box::new(m20221022_213306_vdev_vm_uuid_table::Migration),
            Box::new(m20221022_213722_vdevs_table::Migration),
            Box::new(m20221022_214219_vdisks_table::Migration),
            Box::new(m20221022_214350_vdisks_vm_uuid_table::Migration),
            Box::new(m20221022_214851_vm_templates_table::Migration),
            Box::new(m20221022_222629_vms_table::Migration),
        ]
    }
}

use sea_orm_migration::prelude::*;
/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Uuid {
    Uuid,
}
#[derive(Iden)]
pub enum VdevsVmUuid {
    Table,
    Uuid,
    DeviceOrder,
}
#[derive(Iden)]
pub enum RunningVms {
    Table,
    Uuid,
    State,
    Host,
    IsOrphan,
    VmUuid,
    Fd,
}
#[derive(Iden)]
pub enum Hosts {
    Table,
    Uuid,
    FriendlyName,
    Memory,
    Cpus,
    Arch,
    Ip,
    IsManager,
    State,
}
#[derive(Iden)]
pub enum Vdevs {
    Table,
    Uuid,
    Address,
    Type,
    AddressEnd,
    IsSystem,
    Host,
}
#[derive(Iden)]
pub enum Vdisks {
    Table,
    Uuid,
    Path,
    Host,
}
#[derive(Iden)]
pub enum VdisksVmUuid {
    Table,
    VDiskUuid,
    BootOrder,
}
#[derive(Iden)]
pub enum VmTemplates {
    Table,
    Uuid,
    FriendlyName,
    Memory,
    Vcpus,
    BootDeviceType,
    VDevsUuid,
    VDisksUuid,
    Arch,
}
#[derive(Iden)]
pub enum Vms {
    Table,
    Uuid,
    FriendlyName,
    Memory,
    VCpus,
    BootDeviceType,
    VDisksUuid,
    Template,
    VDevsUuid,
    Arch,
}

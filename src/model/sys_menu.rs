use std::default;

/// 菜单接口模型

#[derive(Default)]
struct SysMenu {
    ID: u8,
    parent_ID: u8,
    menu_name: String,
    icon: String,
    value: String,
    menu_type: u8,
    url: String,
    sort: u8,
    create_time: String,
    children: Vec<SysMenu>,
}

impl SysMenu {
    pub fn table_menu() -> SysMenu {
        let sys_menu: SysMenu = Default::default();
        sys_menu
    }
}

/// 新增菜单
pub fn add_sys_menu() {}
